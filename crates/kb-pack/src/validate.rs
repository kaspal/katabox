//! `kb-pack validate`: structure, metadata, test files, and that each reference solution
//! matches its generated stub; `--run` also judges every reference solution.

use kb_core::{Difficulty, Language};
use serde::{Deserialize, Serialize};
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
pub(crate) struct ProblemToml {
    pub id: String,
    pub title: String,
    /// Spanish title; Spanish falls back to `title` when absent.
    pub title_es: Option<String>,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
    pub pattern: String,
    pub rank: u8,
    #[serde(default)]
    pub profile: bool,
    pub signature: Signature,
    pub judge: Judge,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Signature {
    pub name: String,
    pub params: Vec<Param>,
    pub returns: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Judge {
    pub checker: String,
    pub time_limit_ms: u32,
    pub memory_mb: u32,
}

/// Read and parse `dir/problem.toml`.
pub(crate) fn read_problem_toml(dir: &Path) -> Result<ProblemToml, String> {
    fs::read_to_string(dir.join("problem.toml"))
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str(&s).map_err(|e| e.to_string()))
}

/// Problem folders under `dir`, sorted.
pub(crate) fn problem_dirs(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    Ok(dirs)
}

/// Judge every reference solution in every supported language; all must be Accepted.
pub(crate) fn run_references(dir: &Path, root: &Path) -> ExitCode {
    let root = match std::path::absolute(root) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: {e}", root.display());
            return ExitCode::FAILURE;
        }
    };
    let env = kb_judge::JudgeEnv::dev(root);
    let dirs = match problem_dirs(dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read {}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
    };
    let (mut runs, mut failed) = (0, 0);
    println!();
    for p in &dirs {
        let problem = match kb_judge::Problem::load(p) {
            Ok(pr) => pr,
            Err(e) => {
                failed += 1;
                println!("FAIL  {}: {e}", p.display());
                continue;
            }
        };
        for lang in kb_judge::languages::SUPPORTED {
            runs += 1;
            let name = lang.id();
            let file = p
                .join("solutions")
                .join(format!("ref.{}", lang.extension()));
            let outcome = fs::read_to_string(&file)
                .map_err(|e| e.to_string())
                .and_then(|src| {
                    kb_judge::judge(&problem, &problem.tests, lang, &src, &env)
                        .map_err(|e| e.to_string())
                });
            match outcome {
                Ok(r) if r.verdict.is_accepted() => {
                    println!(
                        "run   {} {name:<10} AC  {}/{} tests, {} us",
                        problem.id, r.total_tests, r.total_tests, r.time_us
                    );
                }
                Ok(r) => {
                    failed += 1;
                    let why = r
                        .first_failure
                        .as_ref()
                        .and_then(|f| f.message.as_ref())
                        .or(r.compile_output.as_ref())
                        .map(|m| {
                            m.text
                                .lines()
                                .take(8)
                                .collect::<Vec<_>>()
                                .join("\n        ")
                        })
                        .unwrap_or_default();
                    println!(
                        "FAIL  {} {name:<10} {}\n        {why}",
                        problem.id,
                        r.verdict.code()
                    );
                }
                Err(e) => {
                    failed += 1;
                    println!("FAIL  {} {name:<10} {e}", problem.id);
                }
            }
        }
    }
    println!("\n{runs} reference run(s), {failed} failed");
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub(crate) fn validate_all(dir: &Path) -> ExitCode {
    let dirs = match problem_dirs(dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read {}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
    };

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

    let meta = match read_problem_toml(dir) {
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
    if meta
        .title_es
        .as_deref()
        .is_some_and(|t| t.trim().is_empty())
    {
        errs.push("title_es is empty (remove it to fall back to title)".into());
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
    if errs.is_empty() {
        check_reference_signatures(dir, &mut errs);
    }
    (Some(meta.difficulty), errs)
}

/// Every reference solution must declare the function exactly as its generated stub
/// does (whitespace aside), so the stub users start from matches what is judged.
fn check_reference_signatures(dir: &Path, errs: &mut Vec<String>) {
    let problem = match kb_judge::Problem::load(dir) {
        Ok(p) => p,
        Err(e) => {
            errs.push(e.to_string());
            return;
        }
    };
    for lang in kb_judge::languages::SUPPORTED {
        let stub = match kb_judge::starter_code(lang, &problem.signature) {
            Ok(s) => s,
            Err(e) => {
                errs.push(format!("{lang} stub: {e}"));
                continue;
            }
        };
        let Some(line) = signature_line(&stub) else {
            continue;
        };
        let file = dir
            .join("solutions")
            .join(format!("ref.{}", lang.extension()));
        let src = fs::read_to_string(&file).unwrap_or_default();
        if !src.lines().any(|l| normalize(l) == line) {
            errs.push(format!(
                "{} does not declare the stub's signature: {line}",
                file.display()
            ));
        }
    }
}

/// The stub's declaration line (skipping `<?php` and blank lines), whitespace-normalized.
fn signature_line(stub: &str) -> Option<String> {
    stub.lines()
        .map(normalize)
        .find(|l| !l.is_empty() && l != "<?php")
}

fn normalize(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn signature_line_skips_php_open_tag() {
        assert_eq!(
            signature_line("<?php\nfunction f(int $a): int {\n\n}\n").as_deref(),
            Some("function f(int $a): int {")
        );
        assert_eq!(
            signature_line("pub fn f(a: i64) -> i64 {\n    todo!()\n}\n").as_deref(),
            Some("pub fn f(a: i64) -> i64 {")
        );
        assert_eq!(
            normalize("  def  f(a:  int) -> int:  "),
            "def f(a: int) -> int:"
        );
    }

    #[test]
    fn two_sum_references_match_their_stubs() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let mut errs = Vec::new();
        check_reference_signatures(&dir, &mut errs);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn a_mismatched_reference_is_reported() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let dir = std::env::temp_dir().join(format!("kbp-mismatch-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        copy_dir(&src, &dir);
        fs::write(
            dir.join("solutions/ref.rs"),
            "pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<usize> { vec![] }\n",
        )
        .unwrap();
        let mut errs = Vec::new();
        check_reference_signatures(&dir, &mut errs);
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(errs.len(), 1, "{errs:?}");
        assert!(
            errs[0].contains("ref.rs") && errs[0].contains("-> Vec<i64>"),
            "{errs:?}"
        );
    }

    pub(crate) fn copy_dir(from: &Path, to: &Path) {
        fs::create_dir_all(to).unwrap();
        for e in fs::read_dir(from).unwrap().flatten() {
            let target = to.join(e.file_name());
            if e.path().is_dir() {
                copy_dir(&e.path(), &target);
            } else {
                fs::copy(e.path(), target).unwrap();
            }
        }
    }
}
