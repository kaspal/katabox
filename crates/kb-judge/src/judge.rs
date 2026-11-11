//! The judge: runs a submission against a batch of tests in one sandboxed run and turns
//! the harness records plus the sandbox outcome into per-test verdicts (ADR 0008).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use kb_core::{
    CompileInfo, Failure, JudgeEvent, Language, Limits, Message, MessageCode, Report, TestCase,
    TestOutcome, TestResult, Verdict, overall_verdict,
};
use kb_runner::{SandboxSpec, Termination};
use serde_json::Value;

use crate::JudgeError;
use crate::checker::{CheckOutcome, Checker};
use crate::codegen::workspace_files;
use crate::languages::{LanguageDef, definition};
use crate::problem::Problem;
use crate::protocol::{self, Record, Request, Status, TestRecord};

/// Extra CPU time on top of the problem's limit for interpreter startup and module
/// loading, which the user does not control. The problem's limit covers the user's code
/// across the whole batch (kb-core `Limits`).
pub const STARTUP_ALLOWANCE: Duration = Duration::from_millis(1_000);

/// Room on stdout for the harness records themselves (return values included), on top
/// of the user's captured prints.
const RECORD_OUTPUT_ALLOWANCE: usize = 8 * 1024 * 1024;

/// Cap on compiler output (stdout + stderr).
const COMPILE_OUTPUT_LIMIT: usize = 1024 * 1024;

/// Compiled binaries kept per language; older ones are deleted (least recently used).
pub const BINARY_CACHE_SIZE: usize = 64;

/// Where the sandbox root and the language toolchains are.
#[derive(Debug, Clone)]
pub struct JudgeEnv {
    /// KataBox sandbox root (contains `.katabox-sandbox-root`).
    pub sandbox_root: PathBuf,
    /// Parent of the per-run work folders; must be inside `sandbox_root`.
    pub runs_dir: PathBuf,
    /// Compiled binaries, as `<cache_dir>/<language>/<hash>.exe`; must be inside
    /// `sandbox_root` and outside `runs_dir`. The sandbox may read and run it.
    pub cache_dir: PathBuf,
    toolchains: Vec<(Language, PathBuf)>,
}

impl JudgeEnv {
    pub fn new(sandbox_root: impl Into<PathBuf>) -> Self {
        let sandbox_root = sandbox_root.into();
        JudgeEnv {
            runs_dir: sandbox_root.join("runs"),
            cache_dir: sandbox_root.join("cache"),
            sandbox_root,
            toolchains: Vec::new(),
        }
    }

    /// The layout `scripts/fetch-toolchains.{ps1,sh}` creates: `<root>/toolchains/`
    /// `python-3.14`, `node-26`, `php-8.5` and `rust-1.99`.
    pub fn dev(sandbox_root: impl Into<PathBuf>) -> Self {
        let env = JudgeEnv::new(sandbox_root);
        let tc = env.sandbox_root.join("toolchains");
        env.with_toolchain(Language::Python, tc.join("python-3.14"))
            .with_toolchain(Language::TypeScript, tc.join("node-26"))
            .with_toolchain(Language::Php, tc.join("php-8.5"))
            .with_toolchain(Language::Rust, tc.join("rust-1.99"))
    }

    /// Use `dir` as the runtime for `lang` (the folder holding `python.exe` / `node.exe`
    /// on Windows, or `bin/` on Linux).
    pub fn with_toolchain(mut self, lang: Language, dir: impl Into<PathBuf>) -> Self {
        self.toolchains.retain(|(l, _)| *l != lang);
        self.toolchains.push((lang, dir.into()));
        self
    }

    /// The program the judge would start for `lang` (the compiler for compiled
    /// languages), or why it is missing.
    pub fn toolchain_program(&self, lang: Language) -> Result<PathBuf, JudgeError> {
        let def = definition(lang)?;
        let toolchain = self.toolchain(lang)?;
        let (program, _) = if def.compile_limits.is_some() {
            def.compile_command(toolchain)?
        } else {
            def.command(toolchain, 1, None)?
        };
        Ok(program)
    }

    pub(crate) fn toolchain(&self, lang: Language) -> Result<&Path, JudgeError> {
        self.toolchains
            .iter()
            .find(|(l, _)| *l == lang)
            .map(|(_, d)| d.as_path())
            .ok_or_else(|| JudgeError::Toolchain(format!("no toolchain configured for {lang}")))
    }
}

/// Judge `source` against `tests` (usually `problem.tests`, or only the public ones for
/// a "Run"). Blocks until the sandboxed run finishes; runs are serialized by kb-runner.
pub fn judge(
    problem: &Problem,
    tests: &[TestCase],
    language: Language,
    source: &str,
    env: &JudgeEnv,
) -> Result<Report, JudgeError> {
    judge_with_progress(problem, tests, language, source, env, &mut |_| {})
}

/// [`judge`], reporting progress to `on_event`: `Compiling` (compiled languages),
/// `Running`, one `Checked` per test that ran, then `Finished`. No event follows an
/// `Err`.
pub fn judge_with_progress(
    problem: &Problem,
    tests: &[TestCase],
    language: Language,
    source: &str,
    env: &JudgeEnv,
    on_event: &mut dyn FnMut(JudgeEvent),
) -> Result<Report, JudgeError> {
    let report = judge_inner(problem, tests, language, source, env, on_event)?;
    let total = u32::try_from(report.tests.len()).unwrap_or(u32::MAX);
    for (i, t) in report.tests.iter().enumerate() {
        on_event(JudgeEvent::Checked {
            kind: t.result.kind,
            index: t.result.index,
            verdict: t.result.verdict,
            done: u32::try_from(i + 1).unwrap_or(u32::MAX),
            total,
        });
    }
    on_event(JudgeEvent::Finished {
        verdict: report.verdict,
    });
    Ok(report)
}

fn judge_inner(
    problem: &Problem,
    tests: &[TestCase],
    language: Language,
    source: &str,
    env: &JudgeEnv,
    on_event: &mut dyn FnMut(JudgeEvent),
) -> Result<Report, JudgeError> {
    if tests.is_empty() {
        return Err(JudgeError::Problem(format!(
            "{}: no tests to run",
            problem.id
        )));
    }
    let def = definition(language)?;
    let toolchain = env.toolchain(language)?;
    let files = workspace_files(&def, &problem.signature, source)?;

    // Compiled languages: compile (or reuse a cached binary), then run the binary with
    // only the cache folder readable. Interpreted: run the harness from the work folder.
    let (binary, compile) = if def.compile_limits.is_some() {
        on_event(JudgeEvent::Compiling);
        let function = problem.signature.function_name(language);
        match compile_cached(&def, toolchain, &files, &function, env)? {
            Compiled::Binary(path, info) => (Some(path), Some(info)),
            Compiled::Failed(message, info) => {
                return Ok(compile_failure(tests, message, info));
            }
        }
    } else {
        (None, None)
    };
    let (program, args) = def.command(toolchain, problem.memory_mb, binary.as_deref())?;
    let read_only = match &binary {
        Some(b) => b.parent().unwrap_or(&env.cache_dir).to_path_buf(),
        None => toolchain.to_path_buf(),
    };

    let mut limits = Limits::default_for(language);
    limits.memory_mb = problem.memory_mb;
    let startup_ms = u32::try_from(STARTUP_ALLOWANCE.as_millis()).unwrap_or(u32::MAX);
    limits.cpu_time_ms = problem.time_limit_ms.saturating_add(startup_ms);
    let print_cap = limits.max_output_kb as usize * 1024;

    let work = WorkDir::create(&env.runs_dir)?;
    if binary.is_none() {
        for (name, contents) in &files {
            std::fs::write(work.path.join(name), contents)?;
        }
    }
    let nonce = nonce();
    let request = Request {
        nonce: &nonce,
        max_output_bytes: print_cap,
        tests: tests.iter().map(|t| &t.input).collect(),
    };

    let mut spec = SandboxSpec::new(&env.sandbox_root, program, &work.path, &limits);
    spec.args = args;
    spec.read_only_dirs = vec![read_only];
    spec.stdin = serde_json::to_vec(&request).map_err(|e| JudgeError::Internal(e.to_string()))?;
    spec.max_output_bytes = RECORD_OUTPUT_ALLOWANCE + print_cap * (tests.len() + 1);
    on_event(JudgeEvent::Running {
        total: u32::try_from(tests.len()).unwrap_or(u32::MAX),
    });
    let outcome = kb_runner::run(&spec)?;

    let facts = RunFacts {
        termination: outcome.termination,
        exit_code: outcome.exit_code,
        memory_limit_hit: outcome.memory_limit_hit,
        memory_kb: u32::try_from(outcome.peak_memory_bytes.min(spec.memory_bytes) / 1024)
            .unwrap_or(u32::MAX),
        stderr: String::from_utf8_lossy(&outcome.stderr).into_owned(),
        time_limit_ms: problem.time_limit_ms,
    };
    let records = protocol::parse(&outcome.stdout, &nonce);
    let mut report = assemble(&problem.checker, tests, records, &facts);
    work.clean_messages(&mut report);
    report.compile = compile;
    Ok(report)
}

enum Compiled {
    Binary(PathBuf, CompileInfo),
    Failed(Message, CompileInfo),
}

/// Compile the workspace in the sandbox, or reuse the binary compiled from the same
/// inputs. The cache key covers the compiler, its arguments, the harness and the source.
fn compile_cached(
    def: &LanguageDef,
    toolchain: &Path,
    files: &[(String, String)],
    function: &str,
    env: &JudgeEnv,
) -> Result<Compiled, JudgeError> {
    let (compiler, args) = def.compile_command(toolchain)?;
    let limits = def
        .compile_limits
        .ok_or_else(|| JudgeError::Internal(format!("{} has no compile limits", def.language)))?;
    let dir = env.cache_dir.join(def.language.id());
    std::fs::create_dir_all(&dir)?;
    let ext = Path::new(&def.binary_file())
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let cached = dir.join(format!("{}{ext}", cache_key(&compiler, &args, files)?));
    if cached.is_file() {
        // Mark as recently used for eviction.
        let _ = std::fs::File::options()
            .write(true)
            .open(&cached)
            .and_then(|f| f.set_modified(SystemTime::now()));
        return Ok(Compiled::Binary(
            cached,
            CompileInfo {
                cached: true,
                time_us: 0,
            },
        ));
    }

    let work = WorkDir::create(&env.runs_dir)?;
    for (name, contents) in files {
        std::fs::write(work.path.join(name), contents)?;
    }
    let run_limits = Limits {
        cpu_time_ms: limits.cpu_time_ms,
        memory_mb: limits.memory_mb,
        max_processes: limits.max_processes,
        max_output_kb: 1024,
    };
    let mut spec = SandboxSpec::new(&env.sandbox_root, compiler, &work.path, &run_limits);
    spec.args = args;
    spec.read_only_dirs = vec![toolchain.to_path_buf()];
    spec.wall_time = limits.wall_time();
    spec.max_output_bytes = COMPILE_OUTPUT_LIMIT;
    // Keep the folder so the binary can be copied out; WorkDir deletes it afterwards.
    spec.keep_work_dir = true;
    let outcome = kb_runner::run(&spec)?;
    let info = CompileInfo {
        cached: false,
        time_us: u64::try_from(outcome.wall_time.as_micros()).unwrap_or(u64::MAX),
    };

    let over_limit = outcome.termination != Termination::Exited
        || outcome.memory_limit_hit
        || outcome.process_limit_hit;
    if over_limit {
        let text = format!(
            "compilation exceeded its limits ({} ms CPU, {} MB, {} processes): {:?}",
            limits.cpu_time_ms, limits.memory_mb, limits.max_processes, outcome.termination
        );
        return Ok(Compiled::Failed(
            Message::new(MessageCode::CompileLimit, text),
            info,
        ));
    }
    let built = work.path.join(def.binary_file());
    if outcome.exit_code != 0 || !built.is_file() {
        let mut out = String::from_utf8_lossy(&outcome.stderr).into_owned();
        out.push_str(&String::from_utf8_lossy(&outcome.stdout));
        let code = if out.contains(&format!("cannot find function `{function}`")) {
            MessageCode::FunctionNotFound
        } else {
            MessageCode::CompileError
        };
        let mut message = Message::new(code, out.trim_end());
        work.clean(&mut message);
        return Ok(Compiled::Failed(message, info));
    }

    // Copy (not move) into the cache folder so the file inherits the folder's sandbox
    // grant, then rename within the folder so a half-written binary is never used.
    let tmp = dir.join(format!(".tmp-{}-{}", std::process::id(), work.id()));
    std::fs::copy(&built, &tmp)?;
    std::fs::rename(&tmp, &cached)?;
    evict(&dir, BINARY_CACHE_SIZE);
    Ok(Compiled::Binary(cached, info))
}

/// Hex SHA-256 of everything that determines the compiled binary.
fn cache_key(
    compiler: &Path,
    args: &[String],
    files: &[(String, String)],
) -> Result<String, JudgeError> {
    use sha2::{Digest, Sha256};
    let meta = std::fs::metadata(compiler)?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let mut h = Sha256::new();
    let mut field = |bytes: &[u8]| {
        h.update((bytes.len() as u64).to_le_bytes());
        h.update(bytes);
    };
    field(compiler.to_string_lossy().as_bytes());
    field(&meta.len().to_le_bytes());
    field(&mtime.to_le_bytes());
    for a in args {
        field(a.as_bytes());
    }
    for (name, contents) in files {
        field(name.as_bytes());
        field(contents.as_bytes());
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// Delete all but the `keep` most recently used binaries in `dir`.
fn evict(dir: &Path, keep: usize) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut files: Vec<(SystemTime, PathBuf)> = entries
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .filter(|e| !e.file_name().to_string_lossy().starts_with(".tmp-"))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    files.sort_by_key(|f| std::cmp::Reverse(f.0));
    for (_, path) in files.into_iter().skip(keep) {
        let _ = std::fs::remove_file(path);
    }
}

fn compile_failure(tests: &[TestCase], message: Message, info: CompileInfo) -> Report {
    Report {
        verdict: Verdict::CompilationError,
        tests: Vec::new(),
        total_tests: u32::try_from(tests.len()).unwrap_or(u32::MAX),
        first_failure: None,
        compile_output: Some(message),
        time_us: 0,
        memory_kb: 0,
        compile: Some(info),
    }
}

/// What the judge needs from the sandbox outcome.
#[derive(Debug, Clone)]
struct RunFacts {
    termination: Termination,
    exit_code: u32,
    memory_limit_hit: bool,
    memory_kb: u32,
    stderr: String,
    time_limit_ms: u32,
}

fn assemble(
    checker: &Checker,
    tests: &[TestCase],
    records: Vec<Result<Record, String>>,
    facts: &RunFacts,
) -> Report {
    let total_tests = u32::try_from(tests.len()).unwrap_or(u32::MAX);
    let mut report = Report {
        verdict: Verdict::InternalError,
        tests: Vec::new(),
        total_tests,
        first_failure: None,
        compile_output: None,
        time_us: 0,
        memory_kb: facts.memory_kb,
        compile: None,
    };

    let (mut ready, mut done, mut load_error) = (false, false, None);
    let mut results: Vec<TestRecord> = Vec::new();
    for r in records {
        match r {
            Ok(Record::CompileError { code, message }) => {
                report.verdict = Verdict::CompilationError;
                report.compile_output = Some(Message::new(code, message.trim_end()));
                return report;
            }
            Ok(Record::LoadError { message }) => load_error = Some(message.trim_end().to_string()),
            Ok(Record::Ready) => ready = true,
            Ok(Record::Result(t)) => results.push(t),
            Ok(Record::Done) => done = true,
            Err(e) => {
                let m = Message::new(
                    MessageCode::BadHarnessRecord,
                    format!("bad harness record: {e}"),
                );
                return internal(report, tests, m);
            }
        }
    }
    if results.iter().enumerate().any(|(i, r)| r.test != i) || results.len() > tests.len() {
        let m = Message::new(
            MessageCode::BadHarnessRecord,
            "harness records out of order",
        );
        return internal(report, tests, m);
    }

    for (rec, tc) in results.into_iter().zip(tests) {
        let (verdict, message) = match rec.status {
            Status::Ok if rec.stdout_truncated => (
                Verdict::OutputLimitExceeded,
                Some(Message::new(
                    MessageCode::TooMuchOutput,
                    "printed more output than allowed for one test",
                )),
            ),
            Status::Ok => match checker.check(&tc.expected, &rec.value) {
                CheckOutcome::Ok => (Verdict::Accepted, None),
                CheckOutcome::Wrong(m) => (Verdict::WrongAnswer, Some(m)),
            },
            Status::Error => (
                Verdict::RuntimeError,
                Some(Message::new(
                    rec.error_code.unwrap_or(MessageCode::Exception),
                    rec.error.as_deref().unwrap_or_default().trim_end(),
                )),
            ),
            Status::Memory => (
                Verdict::MemoryLimitExceeded,
                Some(Message::new(
                    MessageCode::OutOfMemory,
                    "the program ran out of memory",
                )),
            ),
        };
        let actual = (rec.status == Status::Ok).then_some(rec.value);
        report.tests.push(outcome(
            tc,
            verdict,
            rec.time_us,
            facts,
            message,
            actual,
            rec.stdout,
        ));
    }

    let ran = report.tests.len();
    if ran < tests.len() {
        if done {
            let m = Message::new(
                MessageCode::IncompleteResults,
                "harness finished without running every test",
            );
            return internal(report, tests, m);
        }
        // The batch stopped while tests[ran] was running (or while the module loaded).
        let stopped = stop_reason(ready, load_error, facts);
        let Some((verdict, message)) = stopped else {
            let m = Message::new(
                MessageCode::HarnessSilent,
                format!(
                    "the harness exited without reporting (exit code {:#x}){}",
                    facts.exit_code,
                    stderr_tail(&facts.stderr)
                ),
            );
            return internal(report, tests, m);
        };
        let time_us = if verdict == Verdict::TimeLimitExceeded {
            u64::from(facts.time_limit_ms) * 1000
        } else {
            0
        };
        let tc = &tests[ran];
        report.tests.push(outcome(
            tc,
            verdict,
            time_us,
            facts,
            Some(message),
            None,
            String::new(),
        ));
    }

    let results: Vec<TestResult> = report.tests.iter().map(|t| t.result.clone()).collect();
    report.verdict = overall_verdict(&results);
    report.time_us = results
        .iter()
        .map(|r| r.time_us)
        .fold(0, u64::saturating_add);
    report.first_failure = report
        .tests
        .iter()
        .zip(tests)
        .find(|(t, _)| !t.result.verdict.is_accepted())
        .map(|(t, tc)| Failure {
            kind: tc.kind,
            index: tc.index,
            verdict: t.result.verdict,
            input: tc.input.clone(),
            expected: tc.expected.clone(),
            actual: t.actual.clone(),
            message: t.result.message.clone(),
            stdout: t.stdout.clone(),
        });
    report
}

/// Why the run stopped before every test reported, or `None` if the harness itself
/// failed (an internal error, not the user's fault).
fn stop_reason(
    ready: bool,
    load_error: Option<String>,
    facts: &RunFacts,
) -> Option<(Verdict, Message)> {
    if let Some(m) = load_error {
        return Some((
            Verdict::RuntimeError,
            Message::new(MessageCode::LoadError, m),
        ));
    }
    match facts.termination {
        Termination::CpuTimeLimit | Termination::WallTimeLimit => {
            return Some((
                Verdict::TimeLimitExceeded,
                Message::new(
                    MessageCode::TimeLimit,
                    format!("time limit of {} ms exceeded", facts.time_limit_ms),
                ),
            ));
        }
        Termination::OutputLimit => {
            return Some((
                Verdict::OutputLimitExceeded,
                Message::new(MessageCode::OutputLimit, "output limit exceeded"),
            ));
        }
        Termination::Exited => {}
    }
    let oom = [
        "heap out of memory",
        "Fatal process out of memory",
        "MemoryError",
        "Allowed memory size",
        "memory allocation of",
    ];
    if facts.memory_limit_hit || oom.iter().any(|s| facts.stderr.contains(s)) {
        return Some((
            Verdict::MemoryLimitExceeded,
            Message::new(MessageCode::MemoryLimit, "memory limit exceeded"),
        ));
    }
    if !ready {
        return None;
    }
    Some((
        Verdict::RuntimeError,
        Message::new(
            MessageCode::ProcessExited,
            format!(
                "the program exited during this test (exit code {:#x}){}",
                facts.exit_code,
                stderr_tail(&facts.stderr)
            ),
        ),
    ))
}

fn outcome(
    tc: &TestCase,
    verdict: Verdict,
    time_us: u64,
    facts: &RunFacts,
    message: Option<Message>,
    actual: Option<Value>,
    stdout: String,
) -> TestOutcome {
    TestOutcome {
        result: TestResult {
            index: tc.index,
            kind: tc.kind,
            verdict,
            time_us,
            memory_kb: facts.memory_kb,
            message,
        },
        actual,
        stdout,
    }
}

/// Mark the run as an internal error, attributed to the first test that did not report.
fn internal(mut report: Report, tests: &[TestCase], message: Message) -> Report {
    report.verdict = Verdict::InternalError;
    if let Some(tc) = tests.get(report.tests.len()) {
        report.tests.push(TestOutcome {
            result: TestResult {
                index: tc.index,
                kind: tc.kind,
                verdict: Verdict::InternalError,
                time_us: 0,
                memory_kb: report.memory_kb,
                message: Some(message),
            },
            actual: None,
            stdout: String::new(),
        });
    }
    report
}

/// The last part of stderr, without known runtime noise, as "\n<text>".
fn stderr_tail(stderr: &str) -> String {
    const MAX: usize = 2048;
    // python-build-standalone cannot resolve its own folder in the sandbox (ADR 0006).
    let kept: Vec<&str> = stderr
        .lines()
        .filter(|l| !l.starts_with("Failed to find real location of"))
        .collect();
    let text = kept.join("\n");
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    let start = text.len().saturating_sub(MAX);
    let start = (start..text.len())
        .find(|&i| text.is_char_boundary(i))
        .unwrap_or(0);
    format!("\n{}", &text[start..])
}

/// A fresh per-run folder under `runs_dir`. kb-runner deletes it after a run; this
/// removes it if the run never started.
pub(crate) struct WorkDir {
    pub(crate) path: PathBuf,
    id: u64,
}

impl WorkDir {
    pub(crate) fn create(runs_dir: &Path) -> Result<WorkDir, JudgeError> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::fs::create_dir_all(runs_dir)?;
        let path = std::path::absolute(runs_dir)?.join(format!("judge-{}-{n}", std::process::id()));
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir(&path)?;
        Ok(WorkDir { path, id: n })
    }

    fn id(&self) -> u64 {
        self.id
    }

    /// This folder's absolute path as it appears in messages: as a `file:///` URL (Node)
    /// and as a plain path, each with a trailing separator.
    fn prefixes(&self) -> Vec<String> {
        let Ok(canon) = std::fs::canonicalize(&self.path).or_else(|_| {
            // Already deleted by the runner: rebuild the canonical form from the parent.
            let parent = self.path.parent().unwrap_or(&self.path);
            std::fs::canonicalize(parent).map(|p| p.join(self.path.file_name().unwrap_or_default()))
        }) else {
            return Vec::new();
        };
        let plain = canon
            .to_string_lossy()
            .trim_start_matches(r"\\?\")
            .to_string();
        let sep = std::path::MAIN_SEPARATOR;
        let url = format!(
            "file:///{}/",
            plain.replace('\\', "/").trim_start_matches('/')
        );
        vec![url, format!("{plain}{sep}")]
    }

    /// Strip this folder's absolute path from one message.
    fn clean(&self, message: &mut Message) {
        strip_prefixes(message, &self.prefixes());
    }

    /// Strip this folder's absolute path from messages so tracebacks read `solution.ts:3`.
    fn clean_messages(&self, report: &mut Report) {
        let prefixes = self.prefixes();
        let messages = report
            .compile_output
            .iter_mut()
            .chain(
                report
                    .tests
                    .iter_mut()
                    .filter_map(|t| t.result.message.as_mut()),
            )
            .chain(
                report
                    .first_failure
                    .iter_mut()
                    .filter_map(|f| f.message.as_mut()),
            );
        for m in messages {
            strip_prefixes(m, &prefixes);
        }
    }
}

fn strip_prefixes(m: &mut Message, prefixes: &[String]) {
    for p in prefixes {
        if m.text.contains(p.as_str()) {
            m.text = m.text.replace(p.as_str(), "");
        }
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// A per-run marker for harness records. Not a secret: it only keeps the user's own
/// output from being mistaken for a record.
fn nonce() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut out = String::from("KB");
    for salt in 0..2u8 {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u8(salt);
        h.write_u128(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos()),
        );
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::TestKind;
    use serde_json::json;

    fn tests3() -> Vec<TestCase> {
        (1..=3)
            .map(|i| TestCase {
                kind: if i < 3 {
                    TestKind::Public
                } else {
                    TestKind::Hidden
                },
                index: i,
                input: json!({"n": i}).as_object().unwrap().clone(),
                expected: json!(i * 10),
            })
            .collect()
    }

    fn facts(termination: Termination, exit_code: u32) -> RunFacts {
        RunFacts {
            termination,
            exit_code,
            memory_limit_hit: false,
            memory_kb: 1000,
            stderr: String::new(),
            time_limit_ms: 2000,
        }
    }

    fn result(test: usize, value: Value) -> Result<Record, String> {
        Ok(Record::Result(TestRecord {
            test,
            status: Status::Ok,
            value,
            error_code: None,
            error: None,
            time_us: 1_400,
            stdout: String::new(),
            stdout_truncated: false,
        }))
    }

    fn verdicts(r: &Report) -> Vec<Verdict> {
        r.tests.iter().map(|t| t.result.verdict).collect()
    }

    #[test]
    fn all_correct_is_accepted() {
        let recs = vec![
            Ok(Record::Ready),
            result(0, json!(10)),
            result(1, json!(20)),
            result(2, json!(30)),
            Ok(Record::Done),
        ];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            recs,
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::Accepted);
        assert_eq!(r.tests.len(), 3);
        assert_eq!(r.time_us, 4_200);
        assert_eq!(r.first_failure, None);
    }

    #[test]
    fn first_wrong_answer_is_reported_with_its_input() {
        let recs = vec![
            Ok(Record::Ready),
            result(0, json!(10)),
            result(1, json!(21)),
            result(2, json!(0)),
            Ok(Record::Done),
        ];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            recs,
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::WrongAnswer);
        assert_eq!(
            verdicts(&r),
            [
                Verdict::Accepted,
                Verdict::WrongAnswer,
                Verdict::WrongAnswer
            ]
        );
        let f = r.first_failure.unwrap();
        assert_eq!((f.kind, f.index), (TestKind::Public, 2));
        assert_eq!(f.input, *json!({"n": 2}).as_object().unwrap());
        assert_eq!(f.actual, Some(json!(21)));
    }

    #[test]
    fn time_limit_marks_the_running_test() {
        let recs = vec![Ok(Record::Ready), result(0, json!(10))];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            recs,
            &facts(Termination::CpuTimeLimit, 1),
        );
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
        assert_eq!(
            verdicts(&r),
            [Verdict::Accepted, Verdict::TimeLimitExceeded]
        );
        assert_eq!(r.tests[1].result.time_us, 2_000_000);
        assert_eq!(
            r.tests[1].result.message.as_ref().unwrap().code,
            MessageCode::TimeLimit
        );
    }

    #[test]
    fn crash_during_a_test_is_a_runtime_error() {
        let mut f = facts(Termination::Exited, 0xC00000FD);
        f.stderr = "Failed to find real location of python.exe\nstack overflow".into();
        let r = assemble(&Checker::Exact, &tests3(), vec![Ok(Record::Ready)], &f);
        assert_eq!(r.verdict, Verdict::RuntimeError);
        let msg = r.tests[0].result.message.clone().unwrap();
        assert_eq!(msg.code, MessageCode::ProcessExited);
        assert!(
            msg.text.contains("0xc00000fd") && msg.text.ends_with("\nstack overflow"),
            "{msg:?}"
        );
    }

    #[test]
    fn memory_limit_during_a_test() {
        let mut f = facts(Termination::Exited, 1);
        f.memory_limit_hit = true;
        let r = assemble(&Checker::Exact, &tests3(), vec![Ok(Record::Ready)], &f);
        assert_eq!(r.verdict, Verdict::MemoryLimitExceeded);
        assert_eq!(
            r.tests[0].result.message.as_ref().unwrap().code,
            MessageCode::MemoryLimit
        );
    }

    #[test]
    fn compile_and_load_errors() {
        let ce = vec![Ok(Record::CompileError {
            code: MessageCode::SyntaxError,
            message: "bad syntax\n".into(),
        })];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            ce,
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::CompilationError);
        assert_eq!(
            r.compile_output,
            Some(Message::new(MessageCode::SyntaxError, "bad syntax"))
        );
        assert!(r.tests.is_empty());

        let le = vec![Ok(Record::LoadError {
            message: "NameError".into(),
        })];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            le,
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert_eq!(
            r.first_failure.unwrap().message,
            Some(Message::new(MessageCode::LoadError, "NameError"))
        );
    }

    #[test]
    fn printing_too_much_is_output_limit() {
        let mut rec = result(0, json!(10));
        if let Ok(Record::Result(t)) = &mut rec {
            t.stdout_truncated = true;
        }
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            vec![Ok(Record::Ready), rec],
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.tests[0].result.verdict, Verdict::OutputLimitExceeded);
    }

    #[test]
    fn harness_failures_are_internal_errors() {
        // Exited before saying anything.
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            vec![],
            &facts(Termination::Exited, 1),
        );
        assert_eq!(r.verdict, Verdict::InternalError);
        // Claimed to be done with tests missing.
        let recs = vec![Ok(Record::Ready), result(0, json!(10)), Ok(Record::Done)];
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            recs,
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::InternalError);
        // Garbage record.
        let r = assemble(
            &Checker::Exact,
            &tests3(),
            vec![Err("x".into())],
            &facts(Termination::Exited, 0),
        );
        assert_eq!(r.verdict, Verdict::InternalError);
    }

    #[test]
    fn cache_key_tracks_every_input() {
        let compiler = std::env::current_exe().unwrap();
        let args = vec!["-O".to_string()];
        let files = vec![("main.rs".to_string(), "fn main() {}".to_string())];
        let key = cache_key(&compiler, &args, &files).unwrap();
        assert_eq!(key.len(), 64);
        assert_eq!(key, cache_key(&compiler, &args, &files).unwrap(), "stable");
        let edited = vec![("main.rs".to_string(), "fn main() { }".to_string())];
        assert_ne!(key, cache_key(&compiler, &args, &edited).unwrap());
        assert_ne!(key, cache_key(&compiler, &[], &files).unwrap());
        // Field boundaries are length-prefixed: moving text between fields changes the key.
        let a = vec![("ab".to_string(), "c".to_string())];
        let b = vec![("a".to_string(), "bc".to_string())];
        assert_ne!(
            cache_key(&compiler, &args, &a).unwrap(),
            cache_key(&compiler, &args, &b).unwrap()
        );
    }

    #[test]
    fn eviction_keeps_the_most_recently_used() {
        let dir = std::env::temp_dir().join(format!("kbj-evict-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = SystemTime::now() - Duration::from_secs(1000);
        for i in 0..5u64 {
            let f = dir.join(format!("{i}.exe"));
            std::fs::write(&f, "").unwrap();
            let file = std::fs::File::options().write(true).open(&f).unwrap();
            file.set_modified(base + Duration::from_secs(i * 10))
                .unwrap();
        }
        std::fs::write(dir.join(".tmp-in-progress"), "").unwrap();
        evict(&dir, 2);
        let mut left: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(left, [".tmp-in-progress", "3.exe", "4.exe"]);
    }

    #[test]
    fn nonces_differ() {
        let (a, b) = (nonce(), nonce());
        assert_ne!(a, b);
        assert_eq!(a.len(), 34);
    }
}
