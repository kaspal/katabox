//! `kb-pack validate [DIR]` — checks every problem folder under DIR
//! (default `content/problems`) against the package format in the plan.
//!
//! Running reference solutions against tests and stress testing arrive with
//! kb-runner; this first version checks structure, metadata and test files.

use kb_core::{Difficulty, Language};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PATTERNS: &[&str] = &[
    "arrays-hashing",
    "two-pointers",
    "sliding-window",
    "stack",
    "binary-search",
    "linked-list",
    "trees",
    "tries",
    "heap",
    "backtracking",
    "graphs",
    "dynamic-programming",
    "greedy",
    "intervals",
    "math",
    "bit-manipulation",
    "strings",
    "union-find",
    "design",
    "sorting",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProblemToml {
    id: String,
    title: String,
    difficulty: Difficulty,
    tags: Vec<String>,
    pattern: String,
    rank: u8,
    #[serde(default)]
    profile: bool,
    signature: Signature,
    judge: Judge,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Signature {
    name: String,
    params: Vec<Param>,
    returns: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Param {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Judge {
    checker: String,
    time_limit_ms: u32,
    memory_mb: u32,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("validate") => {
            let dir = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(|| "content/problems".into());
            validate_all(&dir)
        }
        _ => {
            eprintln!("usage: kb-pack validate [DIR]");
            ExitCode::from(2)
        }
    }
}

fn validate_all(dir: &Path) -> ExitCode {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("cannot read {}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    let mut failed = 0;
    for p in &dirs {
        let (difficulty, errors) = validate_problem_meta(p);
        if errors.is_empty() {
            println!(
                "ok    {} ({:?})",
                p.display(),
                difficulty.unwrap_or(Difficulty::Easy)
            );
        } else {
            failed += 1;
            println!("FAIL  {}", p.display());
            for e in errors {
                println!("        - {e}");
            }
        }
    }
    println!("\n{} problem(s), {} failed", dirs.len(), failed);
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Returns the parsed difficulty (when readable) and the list of errors.
fn validate_problem_meta(dir: &Path) -> (Option<Difficulty>, Vec<String>) {
    let mut errs = Vec::new();
    let folder = dir.file_name().and_then(|s| s.to_str()).unwrap_or_default();

    let meta: ProblemToml = match fs::read_to_string(dir.join("problem.toml"))
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str(&s).map_err(|e| e.to_string()))
    {
        Ok(m) => m,
        Err(e) => return (None, vec![format!("problem.toml: {e}")]),
    };

    if meta.id != folder {
        errs.push(format!(
            "id {:?} must match folder name {:?}",
            meta.id, folder
        ));
    }
    if meta.title.trim().is_empty() {
        errs.push("title is empty".into());
    }
    if meta.tags.is_empty() {
        errs.push("at least one tag is required".into());
    }
    if !PATTERNS.contains(&meta.pattern.as_str()) {
        errs.push(format!("unknown pattern {:?}", meta.pattern));
    }
    if !(1..=10).contains(&meta.rank) {
        errs.push(format!("rank must be 1-10, got {}", meta.rank));
    }
    if meta.signature.name.is_empty() || meta.signature.returns.is_empty() {
        errs.push("signature needs a name and a return type".into());
    }
    for p in &meta.signature.params {
        if p.name.is_empty() || p.ty.is_empty() {
            errs.push("every param needs a name and a type".into());
        } else if let Err(e) = p.ty.parse::<kb_judge::Type>() {
            errs.push(format!("param {:?}: {e}", p.name));
        }
    }
    if let Err(e) = meta.signature.returns.parse::<kb_judge::Type>() {
        errs.push(format!("returns: {e}"));
    }
    if let Err(e) = meta.judge.checker.parse::<kb_judge::Checker>() {
        errs.push(e);
    }
    if meta.judge.time_limit_ms == 0 || meta.judge.memory_mb == 0 {
        errs.push("judge limits must be positive".into());
    }
    if meta.profile && !dir.join("gen.py").exists() {
        errs.push("profile = true requires gen.py".into());
    }

    for f in [
        "statement.md",
        "statement.es.md",
        "hints.md",
        "editorial.md",
    ] {
        if !dir.join(f).is_file() {
            errs.push(format!("missing {f}"));
        }
    }

    let params: Vec<&str> = meta
        .signature
        .params
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    let public = check_tests(&dir.join("tests/public"), &params, &mut errs);
    let hidden = check_tests(&dir.join("tests/hidden"), &params, &mut errs);
    if public == 0 {
        errs.push("needs at least one public test".into());
    }
    if hidden == 0 {
        errs.push("needs at least one hidden test".into());
    }

    for lang in Language::ALL {
        if lang == Language::JavaScript {
            continue; // the TypeScript reference covers JS (types stripped)
        }
        let f = dir
            .join("solutions")
            .join(format!("ref.{}", lang.extension()));
        if !f.is_file() {
            errs.push(format!("missing reference solution {}", f.display()));
        }
    }
    (Some(meta.difficulty), errs)
}

/// Each test is `{"input": {<param>: ...}, "output": ...}`. Returns the count.
fn check_tests(dir: &Path, params: &[&str], errs: &mut Vec<String>) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    let mut n = 0;
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        n += 1;
        let v: serde_json::Value = match fs::read_to_string(&p)
            .map_err(|e| e.to_string())
            .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
        {
            Ok(v) => v,
            Err(e) => {
                errs.push(format!("{}: {e}", p.display()));
                continue;
            }
        };
        let Some(input) = v.get("input").and_then(|i| i.as_object()) else {
            errs.push(format!("{}: missing object \"input\"", p.display()));
            continue;
        };
        if v.get("output").is_none() {
            errs.push(format!("{}: missing \"output\"", p.display()));
        }
        for name in params {
            if !input.contains_key(*name) {
                errs.push(format!("{}: input lacks param {name:?}", p.display()));
            }
        }
    }
    n
}
