//! Program problems (ADR 0028): the user's file is the whole program. It is checked for
//! syntax (or compiled) once, then run once per test in the sandbox with the test's
//! stdin; its stdout is compared with the expected text by the problem's text checker.
//!
//! There is no harness: the runner already feeds stdin and caps output (kb-runner is
//! unchanged). Each run's CPU cap is the language's usual cap
//! ([`crate::limits::cpu_cap_ms`]), which includes its startup allowance, so a program
//! problem's `time_limit_ms` is a per-test base.

use std::path::Path;

use kb_core::{
    CompileInfo, Failure, JudgeEvent, Language, Limits, Message, MessageCode, Report, TestCase,
    TestOutcome, TestResult, Verdict, overall_verdict,
};
use kb_runner::{SandboxSpec, Termination};
use serde_json::Value;

use crate::JudgeError;
use crate::checker::{CheckOutcome, normalize_text};
use crate::judge::{
    Compiled, JudgeEnv, WorkDir, compile_cached, compile_failure, looks_out_of_memory, stderr_tail,
};
use crate::languages::{LanguageDef, definition, resolve_program};
use crate::limits;
use crate::problem::{Problem, STDIN_PARAM};

/// The most a program may write to stdout and stderr together in one test.
pub const PROGRAM_OUTPUT_BYTES: usize = 8 * 1024 * 1024;
/// The most text of one test's stdout, stderr, input or expected output kept in a report
/// (stored in user.db and sent to the UI). The checker always sees the whole output.
pub const STORED_TEXT_BYTES: usize = 64 * 1024;
/// Appended to a text cut at [`STORED_TEXT_BYTES`].
pub const TRUNCATED_MARK: &str = "\n… (truncated)";
/// Limits of the one syntax-check run (Python, TypeScript, PHP).
const CHECK_LIMITS: Limits = Limits {
    cpu_time_ms: 10_000,
    memory_mb: 512,
    max_processes: 16,
    max_output_kb: 1024,
};

/// `text` cut to [`STORED_TEXT_BYTES`] (at a character boundary) with [`TRUNCATED_MARK`].
pub fn stored_text(text: &str) -> String {
    if text.len() <= STORED_TEXT_BYTES {
        return text.to_string();
    }
    let mut end = STORED_TEXT_BYTES;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}{TRUNCATED_MARK}", &text[..end])
}

/// Whether a stored text was cut.
pub fn was_truncated(text: &str) -> bool {
    text.ends_with(TRUNCATED_MARK)
}

/// The work-folder files for a program: the user's file, and for TypeScript a
/// `package.json` that makes `.ts` files ES modules.
fn program_files(def: &LanguageDef, source: &str) -> Vec<(String, String)> {
    let mut files = vec![(def.solution_file(), source.to_string())];
    if def.language == Language::TypeScript {
        files.push(("package.json".into(), "{\"type\": \"module\"}\n".into()));
    }
    files
}

fn strip_text(text: &str, work: &WorkDir) -> String {
    let mut out = text.to_string();
    for p in work.prefixes() {
        if out.contains(p.as_str()) {
            out = out.replace(p.as_str(), "");
        }
    }
    out
}

/// The TypeScript syntax check (`node -e`, `FILE` replaced by the quoted file name). The
/// run treats `solution.ts` as an ES module (the `package.json` above) and strips its
/// types (strip-only mode, so `enum` and other non-erasable syntax are errors), so the
/// check does the same: `stripTypeScriptTypes`, then `SourceTextModule`, which only
/// parses (nothing is linked or evaluated). A strip error is printed with the file name
/// and without Node's internal frames; a module parse error is rethrown so Node prints
/// it with the source line and caret.
const TS_CHECK: &str = r#"const { readFileSync } = require("node:fs");
const { stripTypeScriptTypes } = require("node:module");
const { SourceTextModule } = require("node:vm");
const file = FILE;
let js;
try {
  js = stripTypeScriptTypes(readFileSync(file, "utf8"), { mode: "strip" });
} catch (e) {
  const text = String((e && e.stack) || e).split("\n").filter((l) => !l.startsWith("    at ")).join("\n");
  process.stderr.write((text.startsWith(":") ? file : "") + text + "\n");
  process.exit(1);
}
new SourceTextModule(js, { identifier: file });
"#;

/// Program and arguments that check the file's syntax without running it.
fn syntax_check(
    def: &LanguageDef,
    toolchain: &Path,
) -> Result<Option<(std::path::PathBuf, Vec<String>)>, JudgeError> {
    let file = def.solution_file();
    let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    Ok(match def.language {
        Language::Python => {
            let code = format!(
                "import sys, traceback\ntry:\n    compile(open({file:?}, 'rb').read(), {file:?}, 'exec')\nexcept SyntaxError as e:\n    sys.stderr.write(''.join(traceback.format_exception_only(e)))\n    sys.exit(1)\n"
            );
            let mut args = words(&["-I", "-X", "utf8", "-c"]);
            args.push(code);
            Some((resolve_program(toolchain, "python")?, args))
        }
        Language::TypeScript => {
            // Not `node --check`: it parses a `.ts` file as CommonJS without stripping
            // types, so annotations are false syntax errors. TS_CHECK treats the file as
            // the run does: Node's own type stripping, then an ES-module parse.
            let mut args = words(&[
                "--preserve-symlinks",
                "--preserve-symlinks-main",
                "--no-warnings",
                "--experimental-vm-modules",
                "--stack-trace-limit=1",
                "-e",
            ]);
            args.push(TS_CHECK.replace("FILE", &format!("{file:?}")));
            Some((resolve_program(toolchain, "node")?, args))
        }
        Language::Php => {
            let mut args = words(&["-n", "-l"]);
            args.push(file);
            Some((resolve_program(toolchain, "php")?, args))
        }
        _ => None,
    })
}

/// A syntax error (or a check that could not finish) as a compilation error, else `None`.
fn check_syntax(
    def: &LanguageDef,
    toolchain: &Path,
    files: &[(String, String)],
    env: &JudgeEnv,
) -> Result<Option<(Message, CompileInfo)>, JudgeError> {
    let Some((program, args)) = syntax_check(def, toolchain)? else {
        return Ok(None);
    };
    let work = WorkDir::create(&env.runs_dir)?;
    for (name, contents) in files {
        std::fs::write(work.path.join(name), contents)?;
    }
    let mut spec = SandboxSpec::new(&env.sandbox_root, program, &work.path, &CHECK_LIMITS);
    spec.args = args;
    spec.read_only_dirs = vec![toolchain.to_path_buf()];
    let outcome = crate::sandbox::run(&spec)?;
    let info = CompileInfo {
        cached: false,
        time_us: u64::try_from(outcome.wall_time.as_micros()).unwrap_or(u64::MAX),
    };
    if outcome.termination == Termination::Exited && outcome.exit_code == 0 {
        return Ok(None);
    }
    let mut text = String::from_utf8_lossy(&outcome.stderr).into_owned();
    text.push_str(&String::from_utf8_lossy(&outcome.stdout));
    let text = strip_text(text.trim(), &work);
    let message = if outcome.termination == Termination::Exited {
        Message::new(MessageCode::SyntaxError, stderr_tail(&text).trim_start())
    } else {
        Message::new(
            MessageCode::CompileLimit,
            format!("the syntax check did not finish: {:?}", outcome.termination),
        )
    };
    Ok(Some((message, info)))
}

/// Judge a program problem: see the module docs. Reports `Compiling` (Rust), `Running`,
/// one `Checked` per test as it finishes, then `Finished`.
pub(crate) fn judge_program(
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
    let files = program_files(&def, source);
    let total = u32::try_from(tests.len()).unwrap_or(u32::MAX);
    let finish = |report: Report, on_event: &mut dyn FnMut(JudgeEvent)| {
        on_event(JudgeEvent::Finished {
            verdict: report.verdict,
        });
        Ok(report)
    };

    // Compile once (Rust, cached), or check the syntax once (interpreted languages).
    let (binary, compile) = if def.compile_limits.is_some() {
        on_event(JudgeEvent::Compiling);
        let command = def.program_compile_command(toolchain)?;
        match compile_cached(&def, command, toolchain, &files, "", env)? {
            Compiled::Binary(path, info) => (Some(path), Some(info)),
            Compiled::Failed(message, info) => {
                return finish(compile_failure(tests, message, info), on_event);
            }
        }
    } else {
        if let Some((message, info)) = check_syntax(&def, toolchain, &files, env)? {
            return finish(compile_failure(tests, message, info), on_event);
        }
        (None, None)
    };
    let (program, args) = def.program_command(toolchain, problem.memory_mb, binary.as_deref())?;
    let read_only = match &binary {
        Some(b) => b.parent().unwrap_or(&env.cache_dir).to_path_buf(),
        None => toolchain.to_path_buf(),
    };
    let mut limits = Limits::default_for(language);
    limits.memory_mb = problem.memory_mb;
    // Per test: the base limit scaled for this language and machine, plus the language's
    // startup allowance (ADR 0017, 0028).
    limits.cpu_time_ms = limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale);
    let shown_limit_ms = limits::effective_limit_ms(problem.time_limit_ms, &def, env.time_scale);

    on_event(JudgeEvent::Running { total });
    let mut report = Report {
        verdict: Verdict::Accepted,
        tests: Vec::new(),
        total_tests: total,
        first_failure: None,
        compile_output: None,
        time_us: 0,
        cpu_us: 0,
        memory_kb: 0,
        compile,
    };
    let mut first_failure = None;
    for (i, tc) in tests.iter().enumerate() {
        let stdin = match tc.input.get(STDIN_PARAM) {
            Some(Value::String(s)) => s.clone(),
            _ => {
                return Err(JudgeError::Problem(format!(
                    "{}: test {} has no stdin text",
                    problem.id, tc.index
                )));
            }
        };
        let work = WorkDir::create(&env.runs_dir)?;
        if binary.is_none() {
            for (name, contents) in &files {
                std::fs::write(work.path.join(name), contents)?;
            }
        }
        let mut spec = SandboxSpec::new(&env.sandbox_root, program.clone(), &work.path, &limits);
        spec.args = args.clone();
        spec.read_only_dirs = vec![read_only.clone()];
        spec.stdin = stdin.into_bytes();
        spec.max_output_bytes = PROGRAM_OUTPUT_BYTES;
        spec.wall_time = spec.cpu_time * env.wall_factor.max(1);
        let outcome = crate::sandbox::run(&spec)?;

        let cpu_us = u64::try_from((outcome.user_time + outcome.kernel_time).as_micros())
            .unwrap_or(u64::MAX);
        let memory_kb = u32::try_from(outcome.peak_memory_bytes.min(spec.memory_bytes) / 1024)
            .unwrap_or(u32::MAX);
        let stdout = String::from_utf8_lossy(&outcome.stdout).into_owned();
        let stderr = normalize_text(&strip_text(
            &String::from_utf8_lossy(&outcome.stderr),
            &work,
        ));

        let (verdict, message, stop) = match outcome.termination {
            Termination::CpuTimeLimit => (
                Verdict::TimeLimitExceeded,
                Some(Message::new(
                    MessageCode::TimeLimit,
                    format!("time limit of {shown_limit_ms} ms exceeded"),
                )),
                true,
            ),
            Termination::WallTimeLimit => (
                Verdict::TimeLimitExceeded,
                Some(Message::new(
                    MessageCode::WallTimeLimit,
                    format!(
                        "stopped by the wall-clock limit before using its {shown_limit_ms} ms of CPU time"
                    ),
                )),
                true,
            ),
            Termination::OutputLimit => (
                Verdict::OutputLimitExceeded,
                Some(Message::new(
                    MessageCode::OutputLimit,
                    format!(
                        "output limit exceeded: more than {} MB",
                        PROGRAM_OUTPUT_BYTES / (1024 * 1024)
                    ),
                )),
                false,
            ),
            Termination::Exited if outcome.memory_limit_hit || looks_out_of_memory(&stderr) => (
                Verdict::MemoryLimitExceeded,
                Some(Message::new(
                    MessageCode::MemoryLimit,
                    "memory limit exceeded",
                )),
                true,
            ),
            Termination::Exited if outcome.exit_code != 0 => (
                Verdict::RuntimeError,
                Some(Message::new(
                    MessageCode::ProcessExited,
                    format!(
                        "the program exited with code {}{}",
                        exit_code_text(outcome.exit_code),
                        stderr_tail(&stderr)
                    ),
                )),
                false,
            ),
            Termination::Exited => match problem
                .checker
                .check(&tc.expected, &Value::String(stdout.clone()))
            {
                CheckOutcome::Ok => (Verdict::Accepted, None, false),
                CheckOutcome::Wrong(m) => (Verdict::WrongAnswer, Some(m), false),
            },
        };
        let time_us = if verdict == Verdict::TimeLimitExceeded {
            u64::from(shown_limit_ms) * 1000
        } else {
            cpu_us
        };
        let ran = outcome.termination == Termination::Exited;
        // Stored and shown with LF line ends (Windows programs often write CRLF; the
        // checker ignores the difference too).
        let shown_stdout = if env.full_program_output {
            normalize_text(&stdout)
        } else {
            stored_text(&normalize_text(&stdout))
        };
        let test = TestOutcome {
            result: TestResult {
                index: tc.index,
                kind: tc.kind,
                verdict,
                time_us,
                memory_kb,
                message,
            },
            actual: ran.then(|| Value::String(shown_stdout.clone())),
            stdout: shown_stdout,
            stderr: (!stderr.is_empty()).then(|| stored_text(&stderr)),
        };
        if first_failure.is_none() && !verdict.is_accepted() {
            first_failure = Some(Failure {
                kind: tc.kind,
                index: tc.index,
                verdict,
                input: stored_input(&tc.input),
                expected: match &tc.expected {
                    Value::String(s) => Value::String(stored_text(s)),
                    other => other.clone(),
                },
                actual: test.actual.clone(),
                message: test.result.message.clone(),
                stdout: test.stdout.clone(),
                stderr: test.stderr.clone(),
            });
        }
        report.time_us = report.time_us.saturating_add(time_us);
        report.cpu_us = report.cpu_us.saturating_add(cpu_us);
        report.memory_kb = report.memory_kb.max(memory_kb);
        on_event(JudgeEvent::Checked {
            kind: tc.kind,
            index: tc.index,
            verdict,
            done: u32::try_from(i + 1).unwrap_or(u32::MAX),
            total,
        });
        report.tests.push(test);
        // A time or memory limit ends the batch, as in function mode: the remaining tests
        // would each cost the full limit.
        if stop {
            break;
        }
    }
    let results: Vec<TestResult> = report.tests.iter().map(|t| t.result.clone()).collect();
    report.verdict = overall_verdict(&results);
    report.first_failure = first_failure;
    finish(report, on_event)
}

/// A test input with its stdin text cut for storage.
fn stored_input(input: &serde_json::Map<String, Value>) -> serde_json::Map<String, Value> {
    input
        .iter()
        .map(|(k, v)| {
            let v = match v {
                Value::String(s) => Value::String(stored_text(s)),
                other => other.clone(),
            };
            (k.clone(), v)
        })
        .collect()
}

/// An exit code as people read it: small codes in decimal, Windows status codes in hex.
fn exit_code_text(code: u32) -> String {
    if code < 256 {
        code.to_string()
    } else {
        format!("{code:#x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_text_cuts_at_a_character_boundary() {
        assert_eq!(stored_text("abc"), "abc");
        assert!(!was_truncated("abc"));
        let long = "ñ".repeat(STORED_TEXT_BYTES); // 2 bytes each
        let cut = stored_text(&long);
        assert!(was_truncated(&cut));
        assert!(cut.len() <= STORED_TEXT_BYTES + TRUNCATED_MARK.len());
        assert!(
            cut.trim_end_matches(TRUNCATED_MARK)
                .chars()
                .all(|c| c == 'ñ')
        );
    }

    #[test]
    fn exit_codes_read_naturally() {
        assert_eq!(exit_code_text(1), "1");
        assert_eq!(exit_code_text(0xC000_0005), "0xc0000005");
    }
}
