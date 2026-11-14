//! The debugger's replay runs (ADR 0022).
//!
//! kb-runner runs a process to completion (stdin up front, stdout at the end), so the
//! debugger never pauses a live process. Instead every debugger action is one ordinary
//! sandboxed run of the language's debug harness:
//!
//! - [`trace`] runs one test and records every step (a line of the user's file, with its
//!   call depth), the prints and the result;
//! - [`stop`] replays the same test to one step, describes the stack, variables, requested
//!   expansions and evaluations there, and exits without running further.
//!
//! A replay must take the same path: the harness reports a hash of the steps it took and
//! [`stop`] compares it with the trace ([`DebugRun::Diverged`] otherwise). Python's debug
//! run fixes `PYTHONHASHSEED` and seeds `random` so ordinary programs replay exactly.

use kb_core::debug::{
    DebugOutcome, DebugSnapshot, EvalResult, Expansion, PrintChunk, StackFrame, Step, StopInfo,
    Trace, Truncation, VarInfo,
};
use kb_core::{Language, Limits};
use kb_runner::{SandboxSpec, Termination};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::JudgeError;
use crate::codegen::debug_files;
use crate::judge::{JudgeEnv, WorkDir, new_nonce};
use crate::languages::definition;
use crate::limits;
use crate::problem::Problem;

/// The most steps one Python trace records; longer runs are truncated (still navigable).
pub const MAX_STEPS: u32 = 200_000;
/// JavaScript/TypeScript (ADR 0023): every pause goes through the inspector (about 0.1 ms
/// each), so a trace records fewer steps …
pub const JS_MAX_STEPS: u32 = 15_000;
/// … and at most this many pauses (several pauses on one line make one step), which
/// keeps a trace, and a stop near its end, around 3 s (measured: 30 000 pauses took 4.1 s).
pub const JS_MAX_PAUSES: u32 = 20_000;
/// The most bytes of encoded steps one trace sends (one byte per step for the first 91
/// distinct (line, depth) pairs, three after that).
pub const TRACE_BUDGET_BYTES: usize = 1 << 20;
/// Room on stdout for everything else: the code table, the result, the prints, a
/// stop's variables, expansions and evaluations.
const RECORD_ALLOWANCE: usize = 512 * 1024;
/// Tracing slows the program down: the debug CPU cap is the language's Run cap times this
/// (Python; JavaScript/TypeScript uses [`JS_CPU_FACTOR`]).
pub const CPU_FACTOR: u32 = 5;
/// Stepping through Node's inspector costs CPU in both threads.
pub const JS_CPU_FACTOR: u32 = 8;
/// … but never more than this.
pub const CPU_MAX_MS: u32 = 60_000;

/// The largest visualizer snapshot a stop may carry (ADR 0027); the harness enforces it
/// too. Well inside [`RECORD_ALLOWANCE`]: a stop run prints no trace and no prints.
pub const SNAPSHOT_MAX_BYTES: usize = 256 * 1024;
/// The most variables one snapshot request names.
pub const SNAPSHOT_MAX_NAMES: usize = 16;

/// Trace tokens: printable ASCII without `"`, `\` and `~` (the harness's `ALPHABET`).
fn alphabet() -> Vec<char> {
    (33u8..126)
        .map(char::from)
        .filter(|c| !matches!(c, '"' | '\\' | '~'))
        .collect()
}

/// What to describe at a stop.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct StopRequest {
    pub step: u32,
    /// Index into the stack (0 = innermost).
    pub frame: u32,
    /// Paths to expand: `[scope, name, i, j, …]`.
    pub expand: Vec<Vec<Value>>,
    /// Watch and console expressions.
    pub evals: Vec<String>,
    /// Variables for the visualizer's snapshot (ADR 0027); `None` = no snapshot.
    pub visualize: Option<Vec<String>>,
}

/// The result of one replay run.
#[derive(Debug, Clone, PartialEq)]
pub enum DebugRun {
    Trace(Trace),
    Stop(StopInfo),
    /// This replay went another way than the trace: the program is not deterministic.
    Diverged,
    /// The solution did not load (syntax error, exception at import, missing function).
    LoadError(String),
    /// The run hit a sandbox limit before it could report.
    Limit(DebugLimit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugLimit {
    Time,
    Memory,
    Output,
}

/// The most steps one trace of `lang` records.
pub fn max_steps(lang: Language) -> u32 {
    if lang == Language::Python {
        MAX_STEPS
    } else {
        JS_MAX_STEPS
    }
}

fn cpu_factor(lang: Language) -> u32 {
    if lang == Language::Python {
        CPU_FACTOR
    } else {
        JS_CPU_FACTOR
    }
}

/// Whether `lang` can be debugged.
pub fn supported(lang: Language) -> bool {
    definition(lang).is_ok_and(|d| d.debug.is_some())
}

/// Whether `lang` can debug a program problem (ADR 0028): Python only for now.
pub fn program_supported(lang: Language) -> bool {
    lang == Language::Python
}

/// Record every step of one test (`input` is checked against the signature by the caller).
pub fn trace(
    problem: &Problem,
    language: Language,
    source: &str,
    input: &Map<String, Value>,
    env: &JudgeEnv,
) -> Result<DebugRun, JudgeError> {
    let request = serde_json::json!({
        "mode": "trace",
        "input": input,
        "max_steps": max_steps(language),
        "max_pauses": JS_MAX_PAUSES,
        "trace_budget": TRACE_BUDGET_BYTES,
    });
    let (records, facts) = run(problem, language, source, request, env)?;
    interpret(records, &facts, None)
}

/// Replay the test of `trace` to `req.step` and describe it there.
pub fn stop(
    problem: &Problem,
    language: Language,
    source: &str,
    input: &Map<String, Value>,
    trace: &Trace,
    req: &StopRequest,
    env: &JudgeEnv,
) -> Result<DebugRun, JudgeError> {
    if req.step as usize >= trace.steps.len() {
        return Err(JudgeError::Internal(format!(
            "step {} is outside the trace ({} steps)",
            req.step,
            trace.steps.len()
        )));
    }
    let request = stop_request(language, input, req);
    let (records, facts) = run(problem, language, source, request, env)?;
    interpret(records, &facts, Some((trace, req.step)))
}

/// The stop run's request (the runner adds the nonce and the print cap).
fn stop_request(language: Language, input: &Map<String, Value>, req: &StopRequest) -> Value {
    let mut request = serde_json::json!({
        "mode": "stop",
        "input": input,
        "step": req.step,
        "frame": req.frame,
        "expand": req.expand,
        "evals": req.evals,
        "max_steps": max_steps(language),
    });
    // Only when asked: without it the request (and the harness's work) is as before.
    if let Some(names) = &req.visualize {
        let names: Vec<&String> = names.iter().take(SNAPSHOT_MAX_NAMES).collect();
        request["visualize"] = serde_json::json!(names);
    }
    request
}

/// What the sandbox reported besides stdout.
struct RunFacts {
    termination: Termination,
    memory_limit_hit: bool,
    stderr: String,
    prefixes: Vec<String>,
}

fn run(
    problem: &Problem,
    language: Language,
    source: &str,
    mut request: Value,
    env: &JudgeEnv,
) -> Result<(Vec<Result<Record, String>>, RunFacts), JudgeError> {
    let def = definition(language)?;
    if def.debug.is_none() {
        return Err(JudgeError::Unsupported(language));
    }
    // A program problem (ADR 0028) is traced as a whole program, in Python only for now:
    // TypeScript's module evaluation is asynchronous to the inspector (deferred).
    let is_program = problem.mode == kb_core::ProblemMode::Program;
    if is_program && !program_supported(language) {
        return Err(JudgeError::Unsupported(language));
    }
    let toolchain = env.toolchain(language)?;
    let files = debug_files(&def, &problem.signature, source, is_program)?;
    let (program, args) = def.debug_command(toolchain, problem.memory_mb)?;

    let mut limits = Limits::default_for(language);
    limits.memory_mb = problem.memory_mb;
    limits.cpu_time_ms = limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale)
        .saturating_mul(cpu_factor(language))
        .min(CPU_MAX_MS);
    let print_cap = limits.max_output_kb as usize * 1024;

    let work = WorkDir::create(&env.runs_dir)?;
    for (name, contents) in &files {
        std::fs::write(work.path.join(name), contents)?;
    }
    if is_program {
        // The harness puts this file on fd 0 for the program.
        let stdin = request["input"][crate::problem::STDIN_PARAM]
            .as_str()
            .unwrap_or_default()
            .to_string();
        std::fs::write(work.path.join("stdin.txt"), stdin)?;
    }
    let nonce = new_nonce();
    request["nonce"] = Value::String(nonce.clone());
    request["max_output_bytes"] = Value::from(print_cap);

    let mut spec = SandboxSpec::new(&env.sandbox_root, program, &work.path, &limits);
    spec.args = args;
    spec.read_only_dirs = vec![toolchain.to_path_buf()];
    // Fixed string hashing, so sets and dicts iterate the same way on every replay. The
    // child gets no other environment from kb-runner.
    if language == Language::Python {
        spec.env = vec![("PYTHONHASHSEED".into(), "0".into())];
    }
    spec.stdin = serde_json::to_vec(&request).map_err(|e| JudgeError::Internal(e.to_string()))?;
    // An explicit budget: the encoded trace, the records around it and the prints.
    spec.max_output_bytes = TRACE_BUDGET_BYTES + RECORD_ALLOWANCE + 2 * print_cap;
    let outcome = crate::sandbox::run(&spec)?;

    let facts = RunFacts {
        termination: outcome.termination,
        memory_limit_hit: outcome.memory_limit_hit,
        stderr: String::from_utf8_lossy(&outcome.stderr).into_owned(),
        prefixes: work.prefixes(),
    };
    Ok((parse(&outcome.stdout, &nonce), facts))
}

/// Records of the debug harness.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Record {
    Trace {
        table: Vec<[u32; 2]>,
        tokens: String,
        steps: u32,
        truncated: Option<Truncation>,
        prints: Vec<(u32, String)>,
        prints_truncated: bool,
    },
    Outcome(DebugOutcome),
    Stop(RawStop),
    Diverged {},
    LoadError {
        message: String,
    },
}

#[derive(Debug, Deserialize)]
struct RawStop {
    step: u32,
    hash: u64,
    stack: Vec<StackFrame>,
    frame: u32,
    locals: Vec<VarInfo>,
    globals: Vec<VarInfo>,
    expanded: Vec<Expansion>,
    evals: Vec<EvalResult>,
    #[serde(default)]
    snapshot: Option<Value>,
}

/// The harness's snapshot, checked: over [`SNAPSHOT_MAX_BYTES`] is `too_large`, a shape
/// kb-core does not know is `invalid`. Either way the stop itself still succeeds.
fn decode_snapshot(raw: Value) -> DebugSnapshot {
    let size = serde_json::to_string(&raw).map_or(usize::MAX, |s| s.len());
    if size > SNAPSHOT_MAX_BYTES {
        return DebugSnapshot::failed("too_large");
    }
    serde_json::from_value(raw).unwrap_or_else(|_| DebugSnapshot::failed("invalid"))
}

fn parse(stdout: &[u8], nonce: &str) -> Vec<Result<Record, String>> {
    let prefix = format!("{nonce} ");
    String::from_utf8_lossy(stdout)
        .lines()
        .filter_map(|line| line.trim_end_matches('\r').strip_prefix(&prefix))
        .map(|json| serde_json::from_str(json).map_err(|e| format!("{e}")))
        .collect()
}

/// Steps from the harness's code table and tokens.
fn decode(table: &[[u32; 2]], tokens: &str, steps: u32) -> Result<Vec<Step>, String> {
    let alpha = alphabet();
    let index_of = |c: char| alpha.iter().position(|a| *a == c);
    let mut out = Vec::with_capacity(steps as usize);
    let mut chars = tokens.chars();
    while let Some(c) = chars.next() {
        let code = if c == '~' {
            let hi = chars.next().and_then(index_of);
            let lo = chars.next().and_then(index_of);
            match (hi, lo) {
                (Some(hi), Some(lo)) => alpha.len() + hi * alpha.len() + lo,
                _ => return Err("bad trace token".into()),
            }
        } else {
            index_of(c).ok_or("bad trace token")?
        };
        let [line, depth] = *table.get(code).ok_or("trace token outside the table")?;
        out.push(Step { line, depth });
    }
    if out.len() != steps as usize {
        return Err(format!("trace has {} steps, expected {steps}", out.len()));
    }
    Ok(out)
}

fn strip(text: &mut String, prefixes: &[String]) {
    for p in prefixes {
        if text.contains(p.as_str()) {
            *text = text.replace(p.as_str(), "");
        }
    }
}

fn interpret(
    records: Vec<Result<Record, String>>,
    facts: &RunFacts,
    stop_of: Option<(&Trace, u32)>,
) -> Result<DebugRun, JudgeError> {
    let mut trace: Option<Trace> = None;
    for record in records {
        let record = record.map_err(|e| JudgeError::Internal(format!("debug record: {e}")))?;
        match record {
            Record::LoadError { mut message } => {
                strip(&mut message, &facts.prefixes);
                return Ok(DebugRun::LoadError(message));
            }
            Record::Diverged {} => return Ok(DebugRun::Diverged),
            Record::Stop(raw) => {
                let Some((trace, step)) = stop_of else {
                    return Err(JudgeError::Internal("unexpected stop record".into()));
                };
                if raw.step != step || raw.hash != trace.prefix_hash(step as usize) {
                    return Ok(DebugRun::Diverged);
                }
                return Ok(DebugRun::Stop(StopInfo {
                    step: raw.step,
                    stack: raw.stack,
                    frame: raw.frame,
                    locals: raw.locals,
                    globals: raw.globals,
                    expanded: raw.expanded,
                    evals: raw.evals,
                    snapshot: raw.snapshot.map(decode_snapshot),
                }));
            }
            Record::Trace {
                table,
                tokens,
                steps,
                truncated,
                prints,
                prints_truncated,
            } => {
                let steps = decode(&table, &tokens, steps).map_err(JudgeError::Internal)?;
                trace = Some(Trace {
                    steps,
                    truncated,
                    prints: prints
                        .into_iter()
                        .map(|(step, text)| PrintChunk { step, text })
                        .collect(),
                    prints_truncated,
                    outcome: None,
                });
                if truncated.is_some() {
                    break;
                }
            }
            Record::Outcome(mut outcome) => {
                if let Some(e) = outcome.error.as_mut() {
                    strip(e, &facts.prefixes);
                }
                if let Some(t) = trace.as_mut() {
                    t.outcome = Some(outcome);
                }
            }
        }
    }
    if let Some(t) = trace
        && stop_of.is_none()
    {
        return Ok(DebugRun::Trace(t));
    }
    // No usable record: the sandbox stopped the run first.
    Ok(match facts.termination {
        Termination::CpuTimeLimit | Termination::WallTimeLimit => DebugRun::Limit(DebugLimit::Time),
        Termination::OutputLimit => DebugRun::Limit(DebugLimit::Output),
        Termination::Exited if facts.memory_limit_hit => DebugRun::Limit(DebugLimit::Memory),
        Termination::Exited => {
            let tail: String = facts
                .stderr
                .chars()
                .rev()
                .take(2000)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            return Err(JudgeError::Internal(format!(
                "the debug harness ended without a record: {tail}"
            )));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts() -> RunFacts {
        RunFacts {
            termination: Termination::Exited,
            memory_limit_hit: false,
            stderr: String::new(),
            prefixes: vec!["C:\\work\\".into()],
        }
    }

    #[test]
    fn alphabet_has_91_json_safe_characters() {
        let a = alphabet();
        assert_eq!(a.len(), 91);
        assert!(
            a.iter()
                .all(|c| c.is_ascii_graphic() && !matches!(c, '"' | '\\' | '~'))
        );
    }

    #[test]
    fn decodes_short_and_long_tokens() {
        let a = alphabet();
        let mut table: Vec<[u32; 2]> = (0..100).map(|i| [i + 1, 1]).collect();
        table[0] = [3, 1];
        // code 0, code 90 (last short), code 91 ("~" + a[0] + a[0]), code 99.
        let tokens: String = [
            a[0].to_string(),
            a[90].to_string(),
            format!("~{}{}", a[0], a[0]),
            format!("~{}{}", a[0], a[8]),
        ]
        .concat();
        let steps = decode(&table, &tokens, 4).unwrap();
        assert_eq!(
            steps.iter().map(|s| s.line).collect::<Vec<_>>(),
            [3, 91, 92, 100]
        );
        assert!(decode(&table, &tokens, 5).is_err());
        assert!(decode(&table, "\"", 1).is_err());
        assert!(decode(&table[..1], &a[5].to_string(), 1).is_err());
    }

    #[test]
    fn interprets_a_trace_with_its_outcome() {
        let out = b"noise\nN {\"kind\":\"trace\",\"table\":[[3,1],[4,1]],\"tokens\":\"!#!#\",\"steps\":4,\"truncated\":null,\"prints\":[[1,\"hi\\n\"]],\"prints_truncated\":false}\n\
N {\"kind\":\"outcome\",\"status\":\"error\",\"error\":\"File C:\\\\work\\\\solution.py, line 4\",\"line\":4,\"step\":3}\n";
        let run = interpret(parse(out, "N"), &facts(), None).unwrap();
        let DebugRun::Trace(t) = run else {
            panic!("{run:?}")
        };
        assert_eq!(t.steps.len(), 4);
        assert_eq!(t.steps[1], Step { line: 4, depth: 1 });
        assert_eq!(
            t.prints,
            [PrintChunk {
                step: 1,
                text: "hi\n".into()
            }]
        );
        let o = t.outcome.unwrap();
        assert_eq!(o.error.as_deref(), Some("File solution.py, line 4"));
        assert_eq!(o.line, Some(4));
    }

    #[test]
    fn a_truncated_trace_has_no_outcome() {
        let out = b"N {\"kind\":\"trace\",\"table\":[[2,1]],\"tokens\":\"!!\",\"steps\":2,\"truncated\":\"steps\",\"prints\":[],\"prints_truncated\":false}\n";
        let DebugRun::Trace(t) = interpret(parse(out, "N"), &facts(), None).unwrap() else {
            panic!()
        };
        assert_eq!(t.truncated, Some(Truncation::Steps));
        assert!(t.outcome.is_none());
    }

    #[test]
    fn a_stop_with_the_wrong_hash_is_divergence() {
        let trace = Trace {
            steps: vec![Step { line: 3, depth: 1 }, Step { line: 4, depth: 1 }],
            truncated: None,
            prints: vec![],
            prints_truncated: false,
            outcome: None,
        };
        let good = trace.prefix_hash(1);
        let rec = |hash: u64| {
            format!(
                "N {{\"kind\":\"stop\",\"step\":1,\"hash\":{hash},\"stack\":[{{\"function\":\"f\",\"line\":4}}],\"frame\":0,\"locals\":[{{\"name\":\"x\",\"type\":\"int\",\"value\":\"1\",\"children\":0}}],\"globals\":[],\"expanded\":[],\"evals\":[]}}\n"
            )
        };
        let ok = interpret(
            parse(rec(good).as_bytes(), "N"),
            &facts(),
            Some((&trace, 1)),
        )
        .unwrap();
        let DebugRun::Stop(s) = ok else {
            panic!("{ok:?}")
        };
        assert_eq!(s.locals[0].name, "x");
        assert_eq!(s.stack[0].line, 4);
        let bad = interpret(
            parse(rec(good ^ 1).as_bytes(), "N"),
            &facts(),
            Some((&trace, 1)),
        )
        .unwrap();
        assert_eq!(bad, DebugRun::Diverged);
        assert_eq!(s.snapshot, None, "no snapshot unless asked for");
    }

    /// The visualizer's snapshot (ADR 0027): decoded with identity and cycles intact,
    /// `too_large` over the byte cap, `invalid` for an unknown shape; the stop succeeds.
    #[test]
    fn stops_carry_a_checked_snapshot() {
        use kb_core::debug::{SnapObject, SnapValue};
        let trace = Trace {
            steps: vec![Step { line: 3, depth: 1 }],
            truncated: None,
            prints: vec![],
            prints_truncated: false,
            outcome: None,
        };
        let hash = trace.prefix_hash(0);
        let rec = |snapshot: &str| {
            // One record per line.
            let snapshot = snapshot.replace('\n', "");
            format!(
                "N {{\"kind\":\"stop\",\"step\":0,\"hash\":{hash},\"stack\":[{{\"function\":\"f\",\"line\":3}}],\"frame\":0,\"locals\":[],\"globals\":[],\"expanded\":[],\"evals\":[],\"snapshot\":{snapshot}}}\n"
            )
        };
        let stop = |snapshot: &str| match interpret(
            parse(rec(snapshot).as_bytes(), "N"),
            &facts(),
            Some((&trace, 0)),
        )
        .unwrap()
        {
            DebugRun::Stop(s) => s.snapshot.unwrap(),
            other => panic!("{other:?}"),
        };
        // head and slow share node 0; node 1's next is node 0 again (a cycle).
        let s = stop(
            r#"{"roots":[{"name":"head","value":{"t":"ref","id":0}},{"name":"slow","value":{"t":"ref","id":0}}],
                "objects":[{"kind":"list_node","val":{"t":"int","v":"1"},"next":{"t":"ref","id":1},"input":0},
                           {"kind":"list_node","val":{"t":"int","v":"2"},"next":{"t":"ref","id":0},"input":1}],
                "truncated":false}"#,
        );
        assert_eq!(s.roots[0].value, SnapValue::Ref { id: 0 });
        assert_eq!(s.roots[1].value, s.roots[0].value);
        assert!(matches!(
            &s.objects[1],
            SnapObject::ListNode {
                next: SnapValue::Ref { id: 0 },
                input: Some(1),
                ..
            }
        ));
        assert_eq!(s.error, None);

        // Truncated: a list keeps its real length.
        let s = stop(
            r#"{"roots":[{"name":"a","value":{"t":"ref","id":0}}],"objects":[{"kind":"list","type":"list","items":[{"t":"int","v":"0"}],"len":100000}],"truncated":true}"#,
        );
        assert!(s.truncated);
        assert!(
            matches!(&s.objects[0], SnapObject::List { len: 100000, items, .. } if items.len() == 1)
        );

        // Over the byte cap, and a shape kb-core does not know.
        let big = format!(
            r#"{{"roots":[],"objects":[{{"kind":"list","type":"list","items":[{{"t":"str","v":"{}"}}],"len":1}}],"truncated":false}}"#,
            "x".repeat(SNAPSHOT_MAX_BYTES)
        );
        assert_eq!(stop(&big).error.as_deref(), Some("too_large"));
        assert_eq!(
            stop(r#"{"roots":[],"objects":[{"kind":"blob"}]}"#)
                .error
                .as_deref(),
            Some("invalid")
        );
        // The harness's own failure comes through as its message.
        assert_eq!(
            stop(r#"{"error":"RecursionError"}"#).error.as_deref(),
            Some("RecursionError")
        );
    }

    #[test]
    fn visualize_is_sent_only_when_asked_and_capped() {
        let input = Map::new();
        let plain = stop_request(Language::Python, &input, &StopRequest::default());
        assert!(plain.get("visualize").is_none(), "{plain}");
        let req = StopRequest {
            visualize: Some((0..40).map(|i| format!("v{i}")).collect()),
            ..Default::default()
        };
        let asked = stop_request(Language::Python, &input, &req);
        let names = asked["visualize"].as_array().unwrap();
        assert_eq!(names.len(), SNAPSHOT_MAX_NAMES);
        assert_eq!(names[0], "v0");
    }

    #[test]
    fn limits_and_errors() {
        let mut f = facts();
        f.termination = Termination::CpuTimeLimit;
        assert_eq!(
            interpret(vec![], &f, None).unwrap(),
            DebugRun::Limit(DebugLimit::Time)
        );
        f.termination = Termination::OutputLimit;
        assert_eq!(
            interpret(vec![], &f, None).unwrap(),
            DebugRun::Limit(DebugLimit::Output)
        );
        f.termination = Termination::Exited;
        assert!(interpret(vec![], &f, None).is_err());
        let load = b"N {\"kind\":\"load_error\",\"message\":\"C:\\\\work\\\\solution.py: boom\"}\n";
        assert_eq!(
            interpret(parse(load, "N"), &facts(), None).unwrap(),
            DebugRun::LoadError("solution.py: boom".into())
        );
        assert!(interpret(parse(b"N {\"kind\":\"bogus\"}\n", "N"), &facts(), None).is_err());
    }

    #[test]
    fn javascript_records_decode_the_same_way() {
        // The JS harness appends "step" to the outcome and "hash" (a u64, written as raw
        // JSON digits) to the stop, and sends diverged with its step count.
        let out = b"N {\"kind\":\"trace\",\"table\":[[2,1],[3,1]],\"tokens\":\"!#!\",\"steps\":3,\"truncated\":null,\"prints\":[[0,\"start\\n\"]],\"prints_truncated\":false}\n\
N {\"kind\":\"outcome\",\"status\":\"ok\",\"value\":[2,4],\"step\":2}\n";
        let DebugRun::Trace(t) = interpret(parse(out, "N"), &facts(), None).unwrap() else {
            panic!()
        };
        assert_eq!(t.steps.len(), 3);
        assert_eq!(t.outcome.as_ref().unwrap().value, serde_json::json!([2, 4]));
        let hash = t.prefix_hash(2);
        assert!(
            hash > 1 << 53,
            "the test needs a hash beyond f64's exact integers"
        );
        let stop = format!(
            "N {{\"kind\":\"stop\",\"step\":2,\"stack\":[{{\"function\":\"twoSum\",\"line\":2}}],\"frame\":0,\"locals\":[{{\"name\":\"seen\",\"type\":\"Map\",\"value\":\"Map(0) {{}}\",\"children\":0}}],\"globals\":[],\"expanded\":[],\"evals\":[],\"hash\":{hash}}}\n"
        );
        let r = interpret(parse(stop.as_bytes(), "N"), &facts(), Some((&t, 2))).unwrap();
        let DebugRun::Stop(s) = r else {
            panic!("{r:?}")
        };
        assert_eq!(s.locals[0].type_name, "Map");
        let diverged = b"N {\"kind\":\"diverged\",\"steps\":1}\n";
        assert_eq!(
            interpret(parse(diverged, "N"), &facts(), Some((&t, 2))).unwrap(),
            DebugRun::Diverged
        );
    }

    #[test]
    fn step_caps_and_cpu_factors_per_language() {
        assert_eq!(max_steps(Language::Python), MAX_STEPS);
        assert_eq!(max_steps(Language::TypeScript), JS_MAX_STEPS);
        const { assert!(JS_MAX_STEPS < JS_MAX_PAUSES && JS_MAX_STEPS < MAX_STEPS) };
        assert_eq!(cpu_factor(Language::Python), CPU_FACTOR);
        assert_eq!(cpu_factor(Language::TypeScript), JS_CPU_FACTOR);
        assert!(supported(Language::Python) && supported(Language::TypeScript));
        assert!(!supported(Language::Php) && !supported(Language::Rust));
    }
}
