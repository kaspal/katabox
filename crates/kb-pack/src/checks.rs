//! `kb-pack validate --run`: the checks that need the sandbox (ADR 0016). Every script
//! (references, `brute.py`, `naive.py`, `gen.py`, `validate.py`) runs through kb-judge,
//! never on the host.
//!
//! Per problem:
//! 1. `validate.py` accepts every committed test input;
//! 2. every reference, in every language, is Accepted on all tests within half the time
//!    limit (the limit is one CPU budget per run, so the sum of test times counts);
//! 3. with `gen.py` + `solutions/brute.py`: 500 seeded small inputs, each accepted by
//!    `validate.py`, on which every reference agrees with brute (the problem's checker);
//! 4. with `solutions/naive.py`: it is correct but gets Time Limit Exceeded on the tests.

use std::fs;
use std::path::Path;

use kb_core::{Language, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, ScriptLimits};
use serde_json::{Map, Value};

/// Seeds for the stress test: 1..=STRESS_SEEDS.
pub const STRESS_SEEDS: u64 = 500;

/// A reference may use at most this share of the time limit (percent).
pub const SPEED_SHARE: u32 = 50;

/// Time for the brute and stress runs (500 small tests in one run).
const STRESS_TIME_LIMIT_MS: u32 = 60_000;

/// What a check is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Check {
    Load,
    Validator,
    Reference(Language),
    Stress,
    Naive,
}

impl Check {
    fn label(self) -> String {
        match self {
            Check::Load => "load".into(),
            Check::Validator => "validator".into(),
            Check::Reference(l) => l.id().into(),
            Check::Stress => "stress".into(),
            Check::Naive => "naive".into(),
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
    std::path::absolute(root)
        .map(JudgeEnv::dev)
        .map_err(|e| format!("{}: {e}", root.display()))
}

/// Run every sandbox check on one problem folder.
pub fn check_problem(dir: &Path, env: &JudgeEnv) -> Vec<Finding> {
    let id = dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();
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

    // 1. The validator accepts every committed test.
    if let Some(v) = &validator {
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
                    add(
                        Check::Validator,
                        true,
                        format!("{} tests valid", problem.tests.len()),
                    );
                } else {
                    add(
                        Check::Validator,
                        false,
                        format!("validate.py rejects {}", bad.join("; ")),
                    );
                }
            }
            Err(e) => add(Check::Validator, false, e.to_string()),
        }
    }

    // 2. References: Accepted, within half the time limit.
    let budget_us = u64::from(problem.time_limit_ms) * 1000 * u64::from(SPEED_SHARE) / 100;
    let mut references = Vec::new();
    for lang in kb_judge::languages::SUPPORTED {
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
        match kb_judge::judge(&problem, &problem.tests, lang, &src, env) {
            Ok(r) if r.verdict.is_accepted() && r.time_us <= budget_us => {
                add(
                    Check::Reference(lang),
                    true,
                    format!(
                        "AC {}/{} tests, {} us (budget {budget_us} us)",
                        r.total_tests, r.total_tests, r.time_us
                    ),
                );
            }
            Ok(r) if r.verdict.is_accepted() => add(
                Check::Reference(lang),
                false,
                format!(
                    "too slow: {} us, over {SPEED_SHARE}% of the {} ms limit ({budget_us} us)",
                    r.time_us, problem.time_limit_ms
                ),
            ),
            Ok(r) => add(Check::Reference(lang), false, describe_failure(&r)),
            Err(e) => add(Check::Reference(lang), false, e.to_string()),
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

    // 4. The weak-test guard: naive.py is correct but too slow.
    if let Ok(naive) = fs::read_to_string(dir.join("solutions/naive.py")) {
        let (ok, detail) = match kb_judge::judge(
            &problem,
            &problem.tests,
            Language::Python,
            &naive,
            env,
        ) {
            Ok(r) if r.verdict == Verdict::TimeLimitExceeded => {
                let at = r
                    .first_failure
                    .as_ref()
                    .map(|f| format!("{} test {}", kind_name(f.kind), f.index))
                    .unwrap_or_default();
                (true, format!("naive.py gets TLE at {at}"))
            }
            Ok(r) if r.verdict.is_accepted() => (
                false,
                format!(
                    "naive.py passed every test in {} us: the max-size tests do not force the intended complexity",
                    r.time_us
                ),
            ),
            Ok(r) => (
                false,
                format!(
                    "naive.py must be correct but slow; it got {}",
                    describe_failure(&r)
                ),
            ),
            Err(e) => (false, e.to_string()),
        };
        add(Check::Naive, ok, detail);
    }
    out
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
    relaxed.time_limit_ms = STRESS_TIME_LIMIT_MS;
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
    let reference = fs::read_to_string(dir.join("solutions/ref.py"))
        .map_err(|e| format!("solutions/ref.py: {e}"))?;
    let inputs = kb_judge::generate_inputs(env, &generator, seeds, size, ScriptLimits::default())
        .map_err(|e| e.to_string())?;
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
    relaxed.time_limit_ms = STRESS_TIME_LIMIT_MS;
    let tests = generated_tests(inputs);
    let outputs = outputs_of(
        &relaxed,
        &tests,
        Language::Python,
        &reference,
        env,
        "ref.py",
    )?;
    let hidden = dir.join("tests").join("hidden");
    fs::create_dir_all(&hidden).map_err(|e| e.to_string())?;
    let mut written = Vec::new();
    for (i, (t, out)) in tests.iter().zip(outputs).enumerate() {
        let path = hidden.join(format!("{}.json", first + u32::try_from(i).unwrap_or(0)));
        if path.exists() {
            return Err(format!("{} exists; pick another --from", path.display()));
        }
        let mut obj = Map::new();
        obj.insert("input".into(), Value::Object(t.input.clone()));
        obj.insert("output".into(), out);
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
