//! Dev command for the judge (English only; not part of the app UI).
//!
//! ```text
//! kb-judge run <problem> --lang <python|typescript> <file> [--public] [--json]
//!              [--root <dir>] [--problems <dir>]
//! kb-judge stub <problem> --lang <python|typescript>
//! kb-judge harness <problem> --lang <python|typescript>
//! ```
//!
//! `<problem>` is a folder with `problem.toml`, or an id under `--problems` (default
//! `content/problems`). `--root` is the sandbox root with the toolchains (default
//! `target/katabox`, created by `scripts/fetch-python-toolchain.{ps1,sh}`).
//! `run` judges every test (public, then hidden) unless `--public`; it exits 0 on
//! Accepted, 1 on any other verdict, 2 on errors.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use kb_core::{Language, Message, Verdict};
use kb_judge::{JudgeEnv, Problem, Report, harness, judge, starter_code};

const USAGE: &str = "usage:
  kb-judge run <problem> --lang <python|typescript> <file> [--public] [--json] [--root <dir>] [--problems <dir>]
  kb-judge stub <problem> --lang <python|typescript>
  kb-judge harness <problem> --lang <python|typescript>";

fn main() -> ExitCode {
    match real_main() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("kb-judge: {e}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn real_main() -> Result<ExitCode, String> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().ok_or("missing command")?;
    let mut lang: Option<Language> = None;
    let mut root = PathBuf::from("target/katabox");
    let mut problems = PathBuf::from("content/problems");
    let (mut public_only, mut json) = (false, false);
    let mut positional: Vec<String> = Vec::new();
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} needs a value"));
        match a.as_str() {
            "--lang" => lang = Some(value("--lang")?.parse().map_err(|e| format!("{e}"))?),
            "--root" => root = value("--root")?.into(),
            "--problems" => problems = value("--problems")?.into(),
            "--public" => public_only = true,
            "--json" => json = true,
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => positional.push(a),
        }
    }
    let lang = lang.ok_or("--lang is required")?;
    let problem_arg = positional.first().ok_or("missing <problem>")?;
    let problem = load_problem(problem_arg, &problems)?;

    match cmd.as_str() {
        "stub" => {
            print!(
                "{}",
                starter_code(lang, &problem.signature).map_err(|e| e.to_string())?
            );
            Ok(ExitCode::SUCCESS)
        }
        "harness" => {
            print!(
                "{}",
                harness(lang, &problem.signature).map_err(|e| e.to_string())?
            );
            Ok(ExitCode::SUCCESS)
        }
        "run" => {
            let file = positional.get(1).ok_or("missing <file>")?;
            let source = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
            let root = std::path::absolute(&root).map_err(|e| e.to_string())?;
            let tests: Vec<_> = if public_only {
                problem.public_tests().cloned().collect()
            } else {
                problem.tests.clone()
            };
            let env = JudgeEnv::dev(root);
            let report = judge(&problem, &tests, lang, &source, &env).map_err(|e| e.to_string())?;
            if json {
                let out = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
                println!("{out}");
            } else {
                print_report(&problem, lang, &report);
            }
            Ok(if report.verdict == Verdict::Accepted {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        other => Err(format!("unknown command {other}")),
    }
}

fn load_problem(arg: &str, problems: &Path) -> Result<Problem, String> {
    let direct = Path::new(arg);
    let dir = if direct.join("problem.toml").is_file() {
        direct.to_path_buf()
    } else {
        problems.join(arg)
    };
    Problem::load(&dir).map_err(|e| e.to_string())
}

fn print_report(problem: &Problem, lang: Language, r: &Report) {
    println!("{} [{lang}] {} test(s)", problem.id, r.total_tests);
    if let Some(out) = &r.compile_output {
        println!("\n[{}]\n{}", code(out), out.text);
    }
    for t in &r.tests {
        let t = &t.result;
        let kind = format!("{:?}", t.kind).to_lowercase();
        let msg = t
            .message
            .as_ref()
            .map(|m| format!("[{}] {}", code(m), m.text.lines().last().unwrap_or("")))
            .unwrap_or_default();
        println!(
            "  {kind:<6} {:>2}  {:<3} {:>10} ms  {msg}",
            t.index,
            t.verdict.code(),
            ms(t.time_us)
        );
    }
    let passed = r
        .tests
        .iter()
        .filter(|t| t.result.verdict.is_accepted())
        .count();
    println!(
        "\n{:?} ({}): {passed}/{} passed, {} ms, {} KiB peak",
        r.verdict,
        r.verdict.code(),
        r.total_tests,
        ms(r.time_us),
        r.memory_kb
    );
    if let Some(f) = &r.first_failure {
        let kind = format!("{:?}", f.kind).to_lowercase();
        println!("\nFirst failing test: {kind} {}", f.index);
        println!("  input:    {}", serde_json::Value::Object(f.input.clone()));
        println!("  expected: {}", f.expected);
        if let Some(a) = &f.actual {
            println!("  actual:   {a}");
        }
        if let Some(m) = &f.message {
            println!(
                "  message [{}]:\n    {}",
                code(m),
                m.text.replace('\n', "\n    ")
            );
        }
        if !f.stdout.is_empty() {
            println!(
                "  stdout:\n    {}",
                f.stdout.trim_end().replace('\n', "\n    ")
            );
        }
    }
}

/// Microseconds as milliseconds with three decimals, e.g. `0.042`.
fn ms(us: u64) -> String {
    format!("{}.{:03}", us / 1000, us % 1000)
}

/// The message code as it appears in JSON, e.g. `output_mismatch`.
fn code(m: &Message) -> String {
    serde_json::to_value(m.code)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}
