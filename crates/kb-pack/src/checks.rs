//! `kb-pack validate --run`: the checks that need the sandbox (ADR 0016). Every script
//! (references, `brute.py`, `naive.py`, `gen.py`, `validate.py`) runs through kb-judge,
//! never on the host.
//!
//! Per problem:
//! 1. `validate.py` accepts every committed test input;
//! 2. every reference, in every language, is Accepted on all tests using at most half of
//!    its language's CPU budget for the run (ADR 0017: the problem's base limit × the
//!    language factor, plus its startup allowance);
//! 3. with `gen.py` + `solutions/brute.py`: 500 seeded small inputs, each accepted by
//!    `validate.py`, on which every reference agrees with brute (the problem's checker);
//! 4. with `solutions/naive.py` and `solutions/naive.rs`: each is correct but too slow.
//!    naive.py gets Time Limit Exceeded under Python's limit; naive.rs is measured under
//!    3× the Rust cap and must need at least 1.5× it (the weak-test margin).
//!
//! An SQL problem (ADR 0031) gets its own set ([`check_sql_problem`]): the validator,
//! `ref.sql` Accepted through the real harness within half its CPU cap per test,
//! `wrong.sql` caught (Wrong Answer), and the stress test: 500 generated inputs on which
//! `ref.sql` (plain sqlite3, one run) agrees with `brute.py`'s rows.
//!
//! Limits are judged at the env's time scale; `kb-pack validate` uses 1.0, the reference
//! machine, and prints this machine's factor so a slower machine can be spotted.

use std::fs;
use std::path::Path;

use kb_core::{Language, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, ScriptLimits, languages, limits};
use serde_json::{Map, Value};

/// Seeds for the stress test: 1..=STRESS_SEEDS.
pub const STRESS_SEEDS: u64 = 500;

/// A reference may use at most this share of its language's CPU budget (percent).
pub const SPEED_SHARE: u64 = 50;

/// Base limit for the brute, stress and generate runs (500 small tests, or a few max
/// ones, in one run). Python, the slowest language, gets 10× this.
const RELAXED_BASE_MS: u32 = 6_000;

/// The weak-test guard runs in these languages, with `solutions/naive.<ext>` (ADR 0017).
pub const NAIVE_LANGUAGES: [Language; 2] = [Language::Python, Language::Rust];

/// naive.rs must need at least this many times its CPU cap (the weak-test margin), so a
/// faster machine or a lucky run cannot let it pass.
pub const NAIVE_MARGIN: f64 = 1.5;
/// naive.rs is measured once under a cap this many times the normal one.
pub const NAIVE_PROBE: u32 = 3;

/// The base limit whose CPU cap is `times` × the cap of `base_ms` (or just above it).
pub fn relaxed_base_ms(base_ms: u32, def: &languages::LanguageDef, scale: f64, times: u32) -> u32 {
    let target = limits::cpu_cap_ms(base_ms, def, scale).saturating_mul(times);
    let mut base = base_ms.saturating_mul(times);
    while base > 0 && limits::cpu_cap_ms(base - 1, def, scale) >= target {
        base -= 1;
    }
    while limits::cpu_cap_ms(base, def, scale) < target {
        base += 1;
    }
    base
}

/// The margin report for naive.rs: `cpu_ms` used out of `cap_ms` when it finished under
/// the probe cap, or `None` when it ran out of time there too.
fn naive_margin(name: &str, cpu_ms: Option<u64>, cap_ms: u32) -> (bool, String) {
    match cpu_ms {
        None => (
            true,
            format!(
                "{name} needs ≥ {NAIVE_PROBE}.0× its CPU cap of {cap_ms} ms (TLE at {} ms)",
                cap_ms * NAIVE_PROBE
            ),
        ),
        Some(cpu) => {
            let ratio = cpu as f64 / f64::from(cap_ms);
            let text = format!("{name} needs {ratio:.2}× its CPU cap ({cpu} of {cap_ms} ms)");
            if ratio >= NAIVE_MARGIN {
                (true, text)
            } else {
                (
                    false,
                    format!(
                        "{text}: below the {NAIVE_MARGIN}× margin; make the max-size tests force the intended complexity"
                    ),
                )
            }
        }
    }
}

/// What a check is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Check {
    Load,
    Validator,
    Reference(Language),
    Stress,
    Naive(Language),
    Profile,
    /// `wrong.sql` must be caught (SQL problems).
    Wrong,
}

impl Check {
    fn label(self) -> String {
        match self {
            Check::Load => "load".into(),
            Check::Validator => "validator".into(),
            Check::Reference(l) => l.id().into(),
            Check::Stress => "stress".into(),
            Check::Naive(l) => format!("naive.{}", l.extension()),
            Check::Profile => "profile".into(),
            Check::Wrong => "wrong.sql".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub problem: String,
    pub check: Check,
    pub ok: bool,
    pub detail: String,
}

/// The sandbox environment for a root (made absolute).
pub fn env_for(root: &Path) -> Result<JudgeEnv, String> {
    let mut env = std::path::absolute(root)
        .map(JudgeEnv::dev)
        .map_err(|e| format!("{}: {e}", root.display()))?;
    // Expected outputs and stress comparisons need a program's whole stdout (ADR 0028).
    env.full_program_output = true;
    // Checks measure CPU time; the wall limit only has to catch hangs, so a busy machine
    // can't end a CPU-bounded run early.
    env.wall_factor = MEASURE_WALL_FACTOR;
    Ok(env)
}

/// kb-pack's wall-clock limit, in multiples of the CPU limit (the app uses 3).
pub const MEASURE_WALL_FACTOR: u32 = 10;
/// Runs stopped by the wall clock (not the CPU limit) are retried this many times before
/// the check gives up as inconclusive.
const WALL_RETRIES: u32 = 2;

/// Whether a Time Limit Exceeded came from the wall clock, not the CPU limit.
fn stopped_by_wall(r: &kb_judge::Report) -> bool {
    r.verdict == Verdict::TimeLimitExceeded
        && r.first_failure
            .as_ref()
            .and_then(|f| f.message.as_ref())
            .is_some_and(|m| m.code == kb_core::MessageCode::WallTimeLimit)
}

/// `kb_judge::judge` for a timing check: a run the wall clock stopped says nothing about
/// CPU time, so it is run again; if every try ends that way the check is inconclusive (an
/// error), never a pass.
fn judge_timed(
    problem: &kb_judge::Problem,
    lang: Language,
    source: &str,
    env: &JudgeEnv,
) -> Result<kb_judge::Report, String> {
    for _ in 0..=WALL_RETRIES {
        let r = kb_judge::judge(problem, &problem.tests, lang, source, env)
            .map_err(|e| e.to_string())?;
        if !stopped_by_wall(&r) {
            return Ok(r);
        }
    }
    Err(format!(
        "inconclusive: the wall clock stopped the run {} times before its CPU limit (the machine is too busy to measure)",
        WALL_RETRIES + 1
    ))
}

/// Run every sandbox check on one problem folder.
pub fn check_problem(dir: &Path, env: &JudgeEnv) -> Vec<Finding> {
    let id = crate::validate::problem_id_of(dir);
    let mut out = Vec::new();
    let mut add = |check: Check, ok: bool, detail: String| {
        out.push(Finding {
            problem: id.clone(),
            check,
            ok,
            detail,
        })
    };
    let problem = match Problem::load(dir) {
        Ok(p) => p,
        Err(e) => {
            add(Check::Load, false, e.to_string());
            return out;
        }
    };
    let script = |name: &str| fs::read_to_string(dir.join(name)).ok();
    let validator = script("validate.py");
    if problem.mode == kb_core::ProblemMode::Sql {
        drop(add);
        out.extend(check_sql_problem(
            dir,
            &id,
            &problem,
            env,
            validator.as_deref(),
        ));
        return out;
    }

    // 1. The validator accepts every committed test.
    if let Some(v) = &validator {
        let (ok, detail) = validator_finding(&problem, env, v);
        add(Check::Validator, ok, detail);
    }

    // 2. References: Accepted, within half of their language's CPU budget.
    let mut references = Vec::new();
    for lang in languages::SUPPORTED {
        let cap_ms = match languages::definition(lang) {
            Ok(def) => limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale),
            Err(e) => {
                add(Check::Reference(lang), false, e.to_string());
                continue;
            }
        };
        let budget_us = u64::from(cap_ms) * 1000 * SPEED_SHARE / 100;
        let file = dir
            .join("solutions")
            .join(format!("ref.{}", lang.extension()));
        let src = match fs::read_to_string(&file) {
            Ok(s) => s,
            Err(e) => {
                add(
                    Check::Reference(lang),
                    false,
                    format!("{}: {e}", file.display()),
                );
                continue;
            }
        };
        match judge_timed(&problem, lang, &src, env) {
            Ok(r) if r.verdict.is_accepted() && limited_cpu(&problem, &r) <= budget_us => {
                add(
                    Check::Reference(lang),
                    true,
                    format!(
                        "AC {}/{} tests, {} ms CPU{} of {} ms allowed ({SPEED_SHARE}% of {cap_ms} ms); tests {} ms",
                        r.total_tests,
                        r.total_tests,
                        ms(limited_cpu(&problem, &r)),
                        per_test(&problem),
                        ms(budget_us),
                        ms(r.time_us)
                    ),
                );
            }
            Ok(r) if r.verdict.is_accepted() => add(
                Check::Reference(lang),
                false,
                format!(
                    "too slow: {} ms CPU{}, over {SPEED_SHARE}% of the {cap_ms} ms {} budget ({} ms); tests {} ms",
                    ms(limited_cpu(&problem, &r)),
                    per_test(&problem),
                    lang.id(),
                    ms(budget_us),
                    ms(r.time_us)
                ),
            ),
            Ok(r) => add(Check::Reference(lang), false, describe_failure(&r)),
            Err(e) => add(Check::Reference(lang), false, e),
        }
        references.push((lang, src));
    }

    // 3. Stress: references agree with brute on 500 generated inputs.
    match (
        script("gen.py"),
        fs::read_to_string(dir.join("solutions/brute.py")),
    ) {
        (Some(generator), Ok(brute)) => {
            let (ok, detail) = match stress(
                &problem,
                env,
                &generator,
                &brute,
                validator.as_deref(),
                &references,
            ) {
                Ok(n) => (
                    true,
                    format!("{n} inputs: every reference agrees with brute.py"),
                ),
                Err(e) => (false, e),
            };
            add(Check::Stress, ok, detail);
        }
        (None, Err(_)) => {}
        _ => add(
            Check::Stress,
            false,
            "gen.py and solutions/brute.py come as a pair".into(),
        ),
    }

    // 4. The weak-test guard: each naive solution is correct but too slow in its own
    //    language.
    for lang in NAIVE_LANGUAGES {
        let name = format!("naive.{}", lang.extension());
        let Ok(naive) = fs::read_to_string(dir.join("solutions").join(&name)) else {
            continue;
        };
        if lang == Language::Rust {
            // Measured: one run under a cap NAIVE_PROBE times the normal one.
            let (ok, detail) = match languages::definition(lang) {
                Err(e) => (false, e.to_string()),
                Ok(def) => {
                    let cap = limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale);
                    let mut relaxed = problem.clone();
                    relaxed.time_limit_ms =
                        relaxed_base_ms(problem.time_limit_ms, &def, env.time_scale, NAIVE_PROBE);
                    match judge_timed(&relaxed, lang, &naive, env) {
                        Ok(r) if r.verdict == Verdict::TimeLimitExceeded => {
                            naive_margin(&name, None, cap)
                        }
                        Ok(r) if r.verdict.is_accepted() => {
                            naive_margin(&name, Some(ms(limited_cpu(&problem, &r))), cap)
                        }
                        Ok(r) => (
                            false,
                            format!(
                                "{name} must be correct but slow; it got {}",
                                describe_failure(&r)
                            ),
                        ),
                        Err(e) => (false, e),
                    }
                }
            };
            add(Check::Naive(lang), ok, detail);
            continue;
        }
        let (ok, detail) = match judge_timed(&problem, lang, &naive, env) {
            Ok(r) if r.verdict == Verdict::TimeLimitExceeded => {
                let at = r
                    .first_failure
                    .as_ref()
                    .map(|f| format!("{} test {}", kind_name(f.kind), f.index))
                    .unwrap_or_default();
                (true, format!("{name} gets TLE at {at}"))
            }
            Ok(r) if r.verdict.is_accepted() => (
                false,
                format!(
                    "{name} passed every test with {} ms CPU: the max-size tests do not force the intended complexity in {}",
                    ms(r.cpu_us),
                    lang.id()
                ),
            ),
            Ok(r) => (
                false,
                format!(
                    "{name} must be correct but slow; it got {}",
                    describe_failure(&r)
                ),
            ),
            Err(e) => (false, e),
        };
        add(Check::Naive(lang), ok, detail);
    }
    // 5. The complexity profiler's inputs (ADR 0024): gen.py still makes them, they
    // match their committed hashes, and the cache for the build is refreshed.
    if let Some(sizes) = crate::validate::read_problem_toml(dir)
        .ok()
        .and_then(|m| m.profile_sizes)
    {
        let (ok, detail) = match crate::profile::fill_cache(dir, env, &sizes) {
            Ok(d) => (true, d),
            Err(e) => (false, e),
        };
        add(Check::Profile, ok, detail);
    }
    out
}

/// The validator's verdict on every committed test (check 1).
fn validator_finding(problem: &Problem, env: &JudgeEnv, v: &str) -> (bool, String) {
    let inputs: Vec<&Map<String, Value>> = problem.tests.iter().map(|t| &t.input).collect();
    match kb_judge::validate_inputs(env, v, &inputs, ScriptLimits::default()) {
        Ok(answers) => {
            let bad: Vec<String> = answers
                .iter()
                .zip(&problem.tests)
                .filter_map(|(a, t)| {
                    a.as_ref()
                        .map(|m| format!("{} test {}: {m}", kind_name(t.kind), t.index))
                })
                .collect();
            if bad.is_empty() {
                (true, format!("{} tests valid", problem.tests.len()))
            } else {
                (false, format!("validate.py rejects {}", bad.join("; ")))
            }
        }
        Err(e) => (false, e.to_string()),
    }
}

/// The SQL judge (ADR 0031), retried like [`judge_timed`] when the wall clock stopped it.
fn judge_sql_timed(
    problem: &Problem,
    query: &str,
    env: &JudgeEnv,
) -> Result<kb_judge::Report, String> {
    for _ in 0..=WALL_RETRIES {
        let r = kb_judge::sql::judge_sql(problem, &problem.tests, query, env, &mut |_| {})
            .map_err(|e| e.to_string())?;
        if !stopped_by_wall(&r) {
            return Ok(r);
        }
    }
    Err(format!(
        "inconclusive: the wall clock stopped the run {} times before its CPU limit (the machine is too busy to measure)",
        WALL_RETRIES + 1
    ))
}

/// The sandbox checks of an SQL problem (ADR 0031).
pub fn check_sql_problem(
    dir: &Path,
    id: &str,
    problem: &Problem,
    env: &JudgeEnv,
    validator: Option<&str>,
) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut add = |check: Check, ok: bool, detail: String| {
        out.push(Finding {
            problem: id.to_string(),
            check,
            ok,
            detail,
        })
    };
    let read = |name: &str| fs::read_to_string(dir.join(name)).map_err(|e| format!("{name}: {e}"));
    // 1. The validator accepts every committed test.
    if let Some(v) = validator {
        let (ok, detail) = validator_finding(problem, env, v);
        add(Check::Validator, ok, detail);
    }
    // 2. ref.sql: Accepted, its slowest test within half the per-test CPU cap.
    let reference = read("solutions/ref.sql");
    let (ok, detail) = match (&reference, languages::definition(kb_judge::sql::HOST)) {
        (Err(e), _) => (false, e.clone()),
        (_, Err(e)) => (false, e.to_string()),
        (Ok(query), Ok(def)) => {
            let cap_ms = limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale);
            let budget_us = u64::from(cap_ms) * 1000 * SPEED_SHARE / 100;
            match judge_sql_timed(problem, query, env) {
                Ok(r) if r.verdict.is_accepted() && limited_cpu(problem, &r) <= budget_us => (
                    true,
                    format!(
                        "AC {}/{} tests, {} ms CPU (slowest test) of {} ms allowed ({SPEED_SHARE}% of {cap_ms} ms); tests {} ms",
                        r.total_tests,
                        r.total_tests,
                        ms(limited_cpu(problem, &r)),
                        ms(budget_us),
                        ms(r.time_us)
                    ),
                ),
                Ok(r) if r.verdict.is_accepted() => (
                    false,
                    format!(
                        "too slow: {} ms CPU (slowest test), over {SPEED_SHARE}% of the {cap_ms} ms budget",
                        ms(limited_cpu(problem, &r))
                    ),
                ),
                Ok(r) => (false, describe_failure(&r)),
                Err(e) => (false, e),
            }
        }
    };
    add(Check::Reference(Language::Sql), ok, detail);
    // 3. wrong.sql: a plausible mistake the tests must catch with Wrong Answer.
    let (ok, detail) = match read("solutions/wrong.sql") {
        Err(e) => (false, e),
        Ok(query) => match judge_sql_timed(problem, &query, env) {
            Ok(r) if r.verdict == Verdict::WrongAnswer => {
                let at = r
                    .first_failure
                    .as_ref()
                    .map(|f| format!("{} test {}", kind_name(f.kind), f.index))
                    .unwrap_or_default();
                (true, format!("wrong.sql gets WA at {at}"))
            }
            Ok(r) if r.verdict.is_accepted() => (
                false,
                "wrong.sql passed every test: the tests do not catch its mistake".into(),
            ),
            Ok(r) => (
                false,
                format!(
                    "wrong.sql must run and give wrong rows; it got {}",
                    describe_failure(&r)
                ),
            ),
            Err(e) => (false, e),
        },
    };
    add(Check::Wrong, ok, detail);
    // 4. Stress: ref.sql agrees with brute.py on 500 generated inputs.
    let (ok, detail) = match (read("gen.py"), read("solutions/brute.py"), &reference) {
        (Ok(generator), Ok(brute), Ok(query)) => {
            match sql_stress(problem, env, &generator, &brute, validator, query) {
                Ok(n) => (true, format!("{n} inputs: ref.sql agrees with brute.py")),
                Err(e) => (false, e),
            }
        }
        (Err(e), _, _) | (_, Err(e), _) => (false, e),
        (_, _, Err(e)) => (false, e.clone()),
    };
    add(Check::Stress, ok, detail);
    out
}

fn sql_stress(
    problem: &Problem,
    env: &JudgeEnv,
    generator: &str,
    brute: &str,
    validator: Option<&str>,
    query: &str,
) -> Result<usize, String> {
    let schema = problem.sql.as_ref().ok_or("not an SQL problem")?;
    let seeds: Vec<u64> = (1..=STRESS_SEEDS).collect();
    let inputs =
        kb_judge::generate_inputs(env, generator, &seeds, "small", ScriptLimits::default())
            .map_err(|e| e.to_string())?;
    for (i, inp) in inputs.iter().enumerate() {
        kb_judge::sql::check_tables(schema, inp).map_err(|e| {
            format!(
                "gen.py made tables that do not fit schema.sql (seed {}): {e}",
                i + 1
            )
        })?;
    }
    let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
    if let Some(v) = validator {
        let answers = kb_judge::validate_inputs(env, v, &refs, ScriptLimits::default())
            .map_err(|e| e.to_string())?;
        if let Some((i, msg)) = answers
            .iter()
            .enumerate()
            .find_map(|(i, a)| a.as_ref().map(|m| (i, m)))
        {
            return Err(format!(
                "gen.py made an input validate.py rejects (seed {}): {msg}\n        input: {}",
                i + 1,
                Value::Object(inputs[i].clone())
            ));
        }
    }
    let expected = kb_judge::brute_rows(env, brute, &refs, ScriptLimits::default())
        .map_err(|e| e.to_string())?;
    let actual = kb_judge::sql_rows(env, &schema.text, query, &refs, ScriptLimits::default())
        .map_err(|e| e.to_string())?;
    for (i, (e, a)) in expected.iter().zip(actual).enumerate() {
        let a = a.map_err(|err| format!("ref.sql fails on seed {}: {err}", i + 1))?;
        if let kb_judge::CheckOutcome::Wrong(m) = problem.checker.check(e, &a) {
            return Err(format!(
                "ref.sql disagrees with brute.py on seed {} ({})\n        input:    {}\n        brute.py: {e}\n        ref.sql:  {a}",
                i + 1,
                m.text,
                Value::Object(inputs[i].clone())
            ));
        }
    }
    Ok(inputs.len())
}

/// Microseconds as whole milliseconds, for messages.
/// The CPU the time limit measures: the whole run for a function problem, the slowest
/// test's run for a program problem (one run per test; ADR 0028).
fn limited_cpu(problem: &kb_judge::Problem, r: &kb_judge::Report) -> u64 {
    // SQL problems also run once per test (ADR 0031).
    if problem.mode != kb_core::ProblemMode::Function {
        r.tests.iter().map(|t| t.result.time_us).max().unwrap_or(0)
    } else {
        r.cpu_us
    }
}

fn per_test(problem: &kb_judge::Problem) -> &'static str {
    if problem.mode == kb_core::ProblemMode::Program {
        " (slowest test)"
    } else {
        ""
    }
}

fn ms(us: u64) -> u64 {
    us / 1000
}

fn kind_name(k: TestKind) -> &'static str {
    match k {
        TestKind::Example => "example",
        TestKind::Public => "public",
        TestKind::Hidden => "hidden",
        TestKind::Custom => "generated",
    }
}

fn describe_failure(r: &kb_judge::Report) -> String {
    let why = r
        .first_failure
        .as_ref()
        .map(|f| {
            let msg = f
                .message
                .as_ref()
                .map(|m| m.text.lines().take(6).collect::<Vec<_>>().join(" / "))
                .unwrap_or_default();
            format!("{} test {}: {msg}", kind_name(f.kind), f.index)
        })
        .or_else(|| {
            r.compile_output
                .as_ref()
                .map(|m| m.text.lines().take(6).collect::<Vec<_>>().join(" / "))
        })
        .unwrap_or_default();
    format!("{} {why}", r.verdict.code())
}

/// The tests for generated inputs (expected values filled in later).
fn generated_tests(inputs: Vec<Map<String, Value>>) -> Vec<TestCase> {
    inputs
        .into_iter()
        .enumerate()
        .map(|(i, input)| TestCase {
            kind: TestKind::Custom,
            index: u32::try_from(i + 1).unwrap_or(u32::MAX),
            input,
            expected: Value::Null,
        })
        .collect()
}

/// The values a solution returns on `tests` (correctness is not checked: expected values
/// are placeholders). Fails if it does not return a value for every test.
pub(crate) fn outputs_of(
    problem: &Problem,
    tests: &[TestCase],
    language: Language,
    src: &str,
    env: &JudgeEnv,
    who: &str,
) -> Result<Vec<Value>, String> {
    let report =
        kb_judge::judge(problem, tests, language, src, env).map_err(|e| format!("{who}: {e}"))?;
    let mut out = Vec::with_capacity(tests.len());
    for (i, t) in tests.iter().enumerate() {
        match report.tests.get(i) {
            Some(o)
                if matches!(o.result.verdict, Verdict::Accepted | Verdict::WrongAnswer)
                    && o.actual.is_some() =>
            {
                out.push(o.actual.clone().unwrap_or(Value::Null));
            }
            Some(o) => {
                return Err(format!(
                    "{who} failed on seed {} ({}): {}",
                    t.index,
                    o.result.verdict.code(),
                    o.result
                        .message
                        .as_ref()
                        .map(|m| m.text.clone())
                        .unwrap_or_default()
                ));
            }
            None => {
                return Err(format!(
                    "{who} stopped before seed {} ({})",
                    t.index,
                    describe_failure(&report)
                ));
            }
        }
    }
    Ok(out)
}

fn stress(
    problem: &Problem,
    env: &JudgeEnv,
    generator: &str,
    brute: &str,
    validator: Option<&str>,
    references: &[(Language, String)],
) -> Result<usize, String> {
    let seeds: Vec<u64> = (1..=STRESS_SEEDS).collect();
    let inputs =
        kb_judge::generate_inputs(env, generator, &seeds, "small", ScriptLimits::default())
            .map_err(|e| e.to_string())?;
    if let Some(v) = validator {
        let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
        let answers = kb_judge::validate_inputs(env, v, &refs, ScriptLimits::default())
            .map_err(|e| e.to_string())?;
        if let Some((i, msg)) = answers
            .iter()
            .enumerate()
            .find_map(|(i, a)| a.as_ref().map(|m| (i, m)))
        {
            return Err(format!(
                "gen.py made an input validate.py rejects (seed {}): {msg}\n        input: {}",
                i + 1,
                Value::Object(inputs[i].clone())
            ));
        }
    }
    let mut relaxed = problem.clone();
    relaxed.time_limit_ms = RELAXED_BASE_MS;
    let mut tests = generated_tests(inputs);
    let expected = outputs_of(&relaxed, &tests, Language::Python, brute, env, "brute.py")?;
    for (t, e) in tests.iter_mut().zip(expected) {
        t.expected = e;
    }
    for (lang, src) in references {
        let r = kb_judge::judge(&relaxed, &tests, *lang, src, env)
            .map_err(|e| format!("{}: {e}", lang.id()))?;
        if !r.verdict.is_accepted() {
            let f = r.first_failure.as_ref();
            let seed = f.map(|f| f.index).unwrap_or(0);
            let input = f
                .map(|f| Value::Object(f.input.clone()))
                .unwrap_or(Value::Null);
            let expected = f.map(|f| f.expected.clone()).unwrap_or(Value::Null);
            let got = f.and_then(|f| f.actual.clone()).unwrap_or(Value::Null);
            return Err(format!(
                "{} disagrees with brute.py on seed {seed} ({})\n        input:    {input}\n        brute.py: {expected}\n        {}: {got}",
                lang.id(),
                r.verdict.code(),
                lang.id()
            ));
        }
    }
    Ok(tests.len())
}

/// `kb-pack generate`: run `gen.py` for `seeds` at `size`, check the inputs with
/// `validate.py`, compute outputs with `solutions/ref.py` (all in the sandbox), and write
/// `tests/hidden/<first..>.json`. Returns the files written.
pub fn generate_tests(
    dir: &Path,
    env: &JudgeEnv,
    size: &str,
    seeds: &[u64],
    first: u32,
) -> Result<Vec<std::path::PathBuf>, String> {
    let problem = Problem::load(dir).map_err(|e| e.to_string())?;
    let generator = fs::read_to_string(dir.join("gen.py")).map_err(|e| format!("gen.py: {e}"))?;
    let reference_file = if problem.mode == kb_core::ProblemMode::Sql {
        "solutions/ref.sql"
    } else {
        "solutions/ref.py"
    };
    let reference = fs::read_to_string(dir.join(reference_file))
        .map_err(|e| format!("{reference_file}: {e}"))?;
    let inputs = kb_judge::generate_inputs(env, &generator, seeds, size, ScriptLimits::default())
        .map_err(|e| e.to_string())?;
    if let Some(schema) = &problem.sql {
        for (i, inp) in inputs.iter().enumerate() {
            kb_judge::sql::check_tables(schema, inp).map_err(|e| {
                format!(
                    "gen.py made tables that do not fit schema.sql (seed {}): {e}",
                    seeds[i]
                )
            })?;
        }
    }
    if let Ok(v) = fs::read_to_string(dir.join("validate.py")) {
        let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
        let answers = kb_judge::validate_inputs(env, &v, &refs, ScriptLimits::default())
            .map_err(|e| e.to_string())?;
        if let Some((i, msg)) = answers
            .iter()
            .enumerate()
            .find_map(|(i, a)| a.as_ref().map(|m| (i, m)))
        {
            return Err(format!(
                "validate.py rejects the input for seed {}: {msg}",
                seeds[i]
            ));
        }
    }
    let mut relaxed = problem.clone();
    relaxed.time_limit_ms = RELAXED_BASE_MS;
    let tests = generated_tests(inputs);
    let outputs = match &problem.sql {
        // An SQL problem's expected rows come from ref.sql (ADR 0031).
        Some(schema) => {
            let refs: Vec<&Map<String, Value>> = tests.iter().map(|t| &t.input).collect();
            kb_judge::sql_rows(
                env,
                &schema.text,
                &reference,
                &refs,
                ScriptLimits::default(),
            )
            .map_err(|e| e.to_string())?
            .into_iter()
            .enumerate()
            .map(|(i, r)| r.map_err(|e| format!("ref.sql fails on seed {}: {e}", seeds[i])))
            .collect::<Result<Vec<_>, _>>()?
        }
        None => outputs_of(
            &relaxed,
            &tests,
            Language::Python,
            &reference,
            env,
            "ref.py",
        )?,
    };
    let hidden = dir.join("tests").join("hidden");
    fs::create_dir_all(&hidden).map_err(|e| e.to_string())?;
    let mut written = Vec::new();
    for (i, (t, out)) in tests.iter().zip(outputs).enumerate() {
        let path = hidden.join(format!("{}.json", first + u32::try_from(i).unwrap_or(0)));
        if path.exists() {
            return Err(format!("{} exists; pick another --from", path.display()));
        }
        let mut obj = Map::new();
        if problem.mode == kb_core::ProblemMode::Program {
            // A program problem's test file is {"stdin", "stdout"} (ADR 0028).
            let stdin = t
                .input
                .get(kb_judge::problem::STDIN_PARAM)
                .cloned()
                .unwrap_or_default();
            obj.insert("stdin".into(), stdin);
            obj.insert("stdout".into(), out);
        } else {
            obj.insert("input".into(), Value::Object(t.input.clone()));
            obj.insert("output".into(), out);
        }
        let text = serde_json::to_string(&Value::Object(obj)).map_err(|e| e.to_string())?;
        fs::write(&path, text + "\n").map_err(|e| e.to_string())?;
        written.push(path);
    }
    Ok(written)
}

/// Print findings like the rest of `validate`: one line per check, then a summary.
pub fn print_findings(findings: &[Finding]) -> usize {
    let mut failed = 0;
    for f in findings {
        let tag = if f.ok { "ok  " } else { "FAIL" };
        if !f.ok {
            failed += 1;
        }
        println!("{tag}  {} {:<10} {}", f.problem, f.check.label(), f.detail);
    }
    failed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_margin_probe_and_messages() {
        let rust = languages::definition(Language::Rust).unwrap();
        // Rust: a 300 ms base gives a 400 ms cap; 3× that is 1200 ms, from a 1100 ms base.
        assert_eq!(limits::cpu_cap_ms(300, &rust, 1.0), 400);
        assert_eq!(relaxed_base_ms(300, &rust, 1.0, 3), 1100);
        assert_eq!(limits::cpu_cap_ms(1100, &rust, 1.0), 1200);
        let (ok, text) = naive_margin("naive.rs", Some(703), 400);
        assert!(ok, "{text}");
        assert_eq!(text, "naive.rs needs 1.76× its CPU cap (703 of 400 ms)");
        let (ok, text) = naive_margin("naive.rs", Some(500), 400);
        assert!(!ok && text.contains("below the 1.5× margin"), "{text}");
        let (ok, text) = naive_margin("naive.rs", None, 400);
        assert!(ok && text.contains("≥ 3.0×"), "{text}");
    }
}
