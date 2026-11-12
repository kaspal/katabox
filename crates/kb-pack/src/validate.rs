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

/// Every sandbox check (crate::checks) on every problem; prints one line per check.
pub fn run_references(dir: &Path, root: &Path) -> ExitCode {
    let env = match crate::checks::env_for(root) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let dirs = match problem_dirs(dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read {}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
    };
    println!();
    // Limits are judged at 1.0, the reference machine (env_for). This machine's factor
    // says how far its timings are from that (ADR 0017).
    match kb_judge::calibration::calibrate(&env) {
        Ok(c) => {
            println!(
                "machine factor {:.2} ({} benchmark {} ms; reference {} ms); limits judged at 1.0",
                c.factor,
                c.runtime.id(),
                c.measured_us / 1000,
                c.reference_us / 1000
            );
            if !(0.8..=1.25).contains(&c.factor) {
                println!(
                    "warning: this machine is not close to the reference machine; speed results may differ there"
                );
            }
        }
        Err(e) => println!("machine factor unknown: {e}"),
    }
    let mut checks = 0;
    let mut failed = 0;
    for p in &dirs {
        let findings = crate::checks::check_problem(p, &env);
        checks += findings.len();
        failed += crate::checks::print_findings(&findings);
    }
    println!(
        "
{} problem(s), {checks} check(s), {failed} failed",
        dirs.len()
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub fn validate_all(dir: &Path) -> ExitCode {
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
    // Problem tooling (ADR 0016): gen.py and brute.py drive the stress test together.
    if dir.join("gen.py").exists() != dir.join("solutions/brute.py").exists() {
        errs.push("gen.py and solutions/brute.py come as a pair".into());
    }
    // The weak-test guard runs in Python and Rust (ADR 0017).
    if dir.join("solutions/naive.py").exists() != dir.join("solutions/naive.rs").exists() {
        errs.push("solutions/naive.py and solutions/naive.rs come as a pair".into());
    }
    for (file, def) in [
        ("gen.py", "def generate("),
        ("validate.py", "def validate("),
    ] {
        if let Ok(src) = fs::read_to_string(dir.join(file)) {
            if !src.lines().any(|l| l.trim_start().starts_with(def)) {
                errs.push(format!("{file} must define {def}...)"));
            }
        }
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
        let mut files = vec![
            dir.join("solutions")
                .join(format!("ref.{}", lang.extension())),
        ];
        // brute.py, naive.py and naive.rs are solutions of the same function.
        let extra: &[&str] = match lang {
            kb_core::Language::Python => &["brute.py", "naive.py"],
            kb_core::Language::Rust => &["naive.rs"],
            _ => &[],
        };
        files.extend(
            extra
                .iter()
                .map(|f| dir.join("solutions").join(f))
                .filter(|f| f.exists()),
        );
        for file in files {
            let src = fs::read_to_string(&file).unwrap_or_default();
            if !src.lines().any(|l| normalize(l) == line) {
                errs.push(format!(
                    "{} does not declare the stub's signature: {line}",
                    file.display()
                ));
            }
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

    #[test]
    fn naive_py_needs_naive_rs() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let base = std::env::temp_dir().join(format!("kbp-naive-pair-{}", std::process::id()));
        let dir = base.join("two-sum");
        let _ = fs::remove_dir_all(&base);
        copy_dir(&src, &dir);
        let (_, ok) = validate_problem_meta(&dir);
        fs::remove_file(dir.join("solutions/naive.rs")).unwrap();
        let (_, errs) = validate_problem_meta(&dir);
        fs::remove_dir_all(&base).unwrap();
        assert!(ok.is_empty(), "{ok:?}");
        assert!(
            errs.iter().any(|e| e.contains("naive.rs come as a pair")),
            "{errs:?}"
        );
    }

    #[test]
    fn a_mismatched_naive_rs_is_reported() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let dir = std::env::temp_dir().join(format!("kbp-naive-sig-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        copy_dir(&src, &dir);
        fs::write(
            dir.join("solutions/naive.rs"),
            "pub fn two_sum(nums: &[i64], target: i64) -> Vec<i64> { vec![] }\n",
        )
        .unwrap();
        let mut errs = Vec::new();
        check_reference_signatures(&dir, &mut errs);
        fs::remove_dir_all(&dir).unwrap();
        assert!(errs.len() == 1 && errs[0].contains("naive.rs"), "{errs:?}");
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
