//! `kb-pack validate`: structure, metadata, test files, and that each reference solution
//! matches its generated stub; `--run` also judges every reference solution.

use kb_core::{Difficulty, Language};
use kb_store::blob::TestBytes;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PATTERNS: &[&str] = &[
    "fundamentals",
    "arrays-hashing",
    "sorting",
    "two-pointers",
    "sliding-window",
    "stack",
    "binary-search",
    "range-queries",
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
    "geometry",
    "bit-manipulation",
    "strings",
    "union-find",
    "design",
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
    /// The complexity profiler's sizes (ADR 0024), with `profile = true`.
    pub profile_sizes: Option<Vec<u32>>,
    /// A function (the default) or a whole program reading stdin (ADR 0028).
    #[serde(default)]
    pub mode: kb_core::ProblemMode,
    /// Required for a function problem; a program problem has none.
    pub signature: Option<Signature>,
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
    let mut total = TestBytes::default();
    for p in &dirs {
        let (difficulty, mut errors) = validate_problem_meta(p);
        let size = if errors.is_empty() {
            match test_size(p) {
                Ok(b) => {
                    errors.extend(budget_error(b));
                    total.json += b.json;
                    total.stored += b.stored;
                    Some(b)
                }
                Err(e) => {
                    errors.push(e);
                    None
                }
            }
        } else {
            None
        };
        if errors.is_empty() {
            println!(
                "ok    {} ({:?})  tests {}",
                p.display(),
                difficulty.unwrap_or(Difficulty::Easy),
                size.map(size_text).unwrap_or_default()
            );
        } else {
            failed += 1;
            println!("FAIL  {}", p.display());
            for e in errors {
                println!("        - {e}");
            }
            if let Some(b) = size {
                println!("        tests {}", size_text(b));
            }
        }
    }
    println!(
        "\n{} problem(s), {} failed; tests {} KB as JSON, {} KB compressed",
        dirs.len(),
        failed,
        kb(total.json),
        kb(total.stored)
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Most a problem's tests may take in content.db, compressed (ADR 0018).
pub const TEST_BUDGET_BYTES: u64 = 512 * 1024;

/// A problem's test bytes, as JSON and as stored in content.db (zstd per test, the same
/// function the build uses).
pub fn test_size(dir: &Path) -> Result<TestBytes, String> {
    let problem = kb_judge::Problem::load(dir).map_err(|e| e.to_string())?;
    kb_store::blob::test_bytes(&problem.tests).map_err(|e| e.to_string())
}

/// Why `b` is over the budget, if it is.
pub fn budget_error(b: TestBytes) -> Option<String> {
    (b.stored > TEST_BUDGET_BYTES).then(|| {
        format!(
            "tests take {} KB compressed, over the {} KB budget (ADR 0018): use smaller values or fewer max-size tests",
            kb(b.stored),
            kb(TEST_BUDGET_BYTES)
        )
    })
}

fn kb(bytes: u64) -> u64 {
    bytes.div_ceil(1024)
}

fn size_text(b: TestBytes) -> String {
    format!(
        "{} KB → {} KB compressed ({}% of {} KB)",
        kb(b.json),
        kb(b.stored),
        b.stored * 100 / TEST_BUDGET_BYTES,
        kb(TEST_BUDGET_BYTES)
    )
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
    let program = meta.mode == kb_core::ProblemMode::Program;
    match (&meta.signature, program) {
        (Some(sig), false) => {
            if sig.name.is_empty() || sig.returns.is_empty() {
                errs.push("signature needs a name and a return type".into());
            }
            for p in &sig.params {
                if p.name.is_empty() || p.ty.is_empty() {
                    errs.push("every param needs a name and a type".into());
                } else if let Err(e) = p.ty.parse::<kb_judge::Type>() {
                    errs.push(format!("param {:?}: {e}", p.name));
                }
            }
            if let Err(e) = sig.returns.parse::<kb_judge::Type>() {
                errs.push(format!("returns: {e}"));
            }
        }
        (None, false) => errs.push("missing [signature]".into()),
        (Some(_), true) => {
            errs.push("a program problem has no [signature] (its program reads stdin)".into())
        }
        (None, true) => {}
    }
    match meta.judge.checker.parse::<kb_judge::Checker>() {
        Err(e) => errs.push(e),
        Ok(c) if c.is_text() != program => errs.push(if program {
            "a program problem needs a text checker (tokens | tokens-float:<eps> | lines)".into()
        } else {
            "text checkers (tokens, lines) are for program problems".into()
        }),
        Ok(_) => {}
    }
    if program && meta.profile {
        errs.push("the complexity profiler is not available for program problems yet".into());
    }
    if meta.judge.time_limit_ms == 0 || meta.judge.memory_mb == 0 {
        errs.push("judge limits must be positive".into());
    }
    if meta.profile && !dir.join("gen.py").exists() {
        errs.push("profile = true requires gen.py".into());
    }
    match (meta.profile, &meta.profile_sizes) {
        (true, None) => errs.push("profile = true requires profile_sizes".into()),
        (false, Some(_)) => errs.push("profile_sizes needs profile = true".into()),
        (true, Some(sizes)) => {
            if let Some(e) = crate::profile::sizes_error(sizes) {
                errs.push(e);
            } else if let Some(e) = crate::profile::files_error(dir, sizes) {
                errs.push(e);
            }
        }
        (false, None) => {
            if dir.join("profile").exists() {
                errs.push("profile/ needs profile = true".into());
            }
        }
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
        if let Ok(src) = fs::read_to_string(dir.join(file))
            && !src.lines().any(|l| l.trim_start().starts_with(def))
        {
            errs.push(format!("{file} must define {def}...)"));
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

    let (public, hidden) = if program {
        let public = check_program_tests(&dir.join("tests/public"), &mut errs);
        let hidden = check_program_tests(&dir.join("tests/hidden"), &mut errs);
        check_program_statements(dir, &public, &mut errs);
        (public.len(), hidden.len())
    } else {
        // Each param with its type, when it parses (a bad type is reported above).
        let empty = Signature {
            name: String::new(),
            params: Vec::new(),
            returns: String::new(),
        };
        let sig = meta.signature.as_ref().unwrap_or(&empty);
        let params: Vec<(&str, Option<kb_judge::Type>)> = sig
            .params
            .iter()
            .map(|p| (p.name.as_str(), p.ty.parse().ok()))
            .collect();
        let returns: Option<kb_judge::Type> = sig.returns.parse().ok();
        let public = check_tests(
            &dir.join("tests/public"),
            &params,
            returns.as_ref(),
            &mut errs,
        );
        let hidden = check_tests(
            &dir.join("tests/hidden"),
            &params,
            returns.as_ref(),
            &mut errs,
        );
        (public, hidden)
    };
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
    // A program problem's references are whole programs: no declaration line to match.
    if errs.is_empty() && !program {
        check_reference_signatures(dir, &mut errs);
    }
    (Some(meta.difficulty), errs)
}

/// A program problem's tests (ADR 0028), in file order: `{"stdin": text, "stdout":
/// text}`, no NUL characters, and every integer token within ±(2⁵³ − 1) (the numbers a
/// JavaScript solution parses with `Number`). Returns the (stdin, stdout) pairs.
fn check_program_tests(dir: &Path, errs: &mut Vec<String>) -> Vec<(String, String)> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<std::path::PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    files.sort_by_key(|p| {
        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        (stem.parse::<u64>().unwrap_or(u64::MAX), stem.to_string())
    });
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ProgramTest {
        stdin: String,
        stdout: String,
    }
    let mut out = Vec::new();
    for p in files {
        let t: ProgramTest = match fs::read_to_string(&p)
            .map_err(|e| e.to_string())
            .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
        {
            Ok(t) => t,
            Err(e) => {
                errs.push(format!(
                    "{}: {e} (a program test is {{\"stdin\", \"stdout\"}})",
                    p.display()
                ));
                continue;
            }
        };
        for (what, text) in [("stdin", &t.stdin), ("stdout", &t.stdout)] {
            if text.contains('\0') {
                errs.push(format!("{}: {what} contains a NUL character", p.display()));
            }
            if let Some(tok) = unsafe_integer_token(text) {
                errs.push(format!(
                    "{}: {what} has {tok}, outside ±(2⁵³ − 1), where JavaScript loses precision",
                    p.display()
                ));
            }
        }
        out.push((t.stdin, t.stdout));
    }
    out
}

/// The first whitespace-separated integer token beyond ±(2⁵³ − 1), if any.
fn unsafe_integer_token(text: &str) -> Option<&str> {
    text.split_ascii_whitespace().find(|tok| {
        let digits = tok.strip_prefix(['-', '+']).unwrap_or(tok);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return false; // not an integer token
        }
        let value = digits.trim_start_matches('0');
        // Over 16 digits is always beyond 2⁵³ − 1 (16 digits); otherwise compare.
        value.len() > 16
            || value
                .parse::<u64>()
                .is_ok_and(|n| n > MAX_SAFE_INTEGER as u64)
    })
}

/// A program problem's statements (ADR 0028): the input and output format headings, and
/// examples equal to the public tests. Each `### Example N` (Spanish `### Ejemplo N`)
/// holds two code fences, the input and the output, equal to public test N's stdin and
/// stdout up to the final newline.
fn check_program_statements(dir: &Path, public: &[(String, String)], errs: &mut Vec<String>) {
    for (file, example, headings) in [
        (
            "statement.md",
            "Example",
            ["## Input format", "## Output format"],
        ),
        (
            "statement.es.md",
            "Ejemplo",
            ["## Formato de entrada", "## Formato de salida"],
        ),
    ] {
        let Ok(text) = fs::read_to_string(dir.join(file)) else {
            continue; // reported as missing above
        };
        let text = text.replace("\r\n", "\n");
        for h in headings {
            if !text.lines().any(|l| l.trim_end() == h) {
                errs.push(format!("{file}: missing the {h:?} heading"));
            }
        }
        let examples = program_examples(&text, example);
        if examples.is_empty() {
            errs.push(format!("{file}: no \"### {example} N\" sections"));
        }
        for (n, fences) in &examples {
            let Some((stdin, stdout)) = public.get(n.wrapping_sub(1)) else {
                errs.push(format!("{file}: {example} {n} has no public test {n}"));
                continue;
            };
            let same = |a: &str, b: &str| {
                a.replace("\r\n", "\n").trim_end_matches('\n')
                    == b.replace("\r\n", "\n").trim_end_matches('\n')
            };
            match fences.as_slice() {
                [input, output, ..] => {
                    if !same(input, stdin) {
                        errs.push(format!(
                            "{file}: {example} {n}'s input differs from public test {n}'s stdin"
                        ));
                    }
                    if !same(output, stdout) {
                        errs.push(format!(
                            "{file}: {example} {n}'s output differs from public test {n}'s stdout"
                        ));
                    }
                }
                _ => errs.push(format!(
                    "{file}: {example} {n} needs an input fence and an output fence"
                )),
            }
        }
    }
}

/// `### <word> N` sections and the code fences in each, in order.
fn program_examples(text: &str, word: &str) -> Vec<(usize, Vec<String>)> {
    let mut out: Vec<(usize, Vec<String>)> = Vec::new();
    let mut fence: Option<String> = None;
    for line in text.lines() {
        if let Some(body) = fence.as_mut() {
            if line.trim_start().starts_with("```") {
                if let Some((_, fences)) = out.last_mut() {
                    fences.push(std::mem::take(body));
                }
                fence = None;
            } else {
                body.push_str(line);
                body.push('\n');
            }
            continue;
        }
        if line.trim_start().starts_with("```") {
            fence = Some(String::new());
            continue;
        }
        if line.starts_with('#') {
            let n = line
                .strip_prefix("### ")
                .and_then(|h| h.strip_prefix(word))
                .and_then(|h| h.trim().parse::<usize>().ok());
            match n {
                Some(n) => out.push((n, Vec::new())),
                // Another heading ends the example; later fences are not part of it.
                None if !out.is_empty() => out.push((usize::MAX, Vec::new())),
                None => {}
            }
        }
    }
    out.retain(|(n, _)| *n != usize::MAX);
    out
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
        // The function's declaration line; node definitions above it may differ.
        let line = match kb_judge::signature_line(lang, &problem.signature) {
            Ok(l) => normalize(&l),
            Err(e) => {
                errs.push(format!("{lang} stub: {e}"));
                continue;
            }
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

fn normalize(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Each test is `{"input": {<param>: ...}, "output": ...}`, every value of its declared
/// type (ADR 0025: lists and trees in their canonical JSON). Returns the count.
fn check_tests(
    dir: &Path,
    params: &[(&str, Option<kb_judge::Type>)],
    returns: Option<&kb_judge::Type>,
    errs: &mut Vec<String>,
) -> usize {
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
        // JSON numbers beyond ±(2⁵³ − 1) lose precision in JavaScript (JSON.parse).
        if let Some(n) = unsafe_integer(&v) {
            errs.push(format!(
                "{}: {n} is outside ±(2⁵³ − 1), where JavaScript loses precision",
                p.display()
            ));
        }
        let Some(input) = v.get("input").and_then(|i| i.as_object()) else {
            errs.push(format!("{}: missing object \"input\"", p.display()));
            continue;
        };
        match (v.get("output"), returns) {
            (None, _) => errs.push(format!("{}: missing \"output\"", p.display())),
            (Some(out), Some(ty)) => {
                if let Err(e) = ty.check_value(out) {
                    errs.push(format!("{}: output: {e}", p.display()));
                }
            }
            (Some(_), None) => {}
        }
        for (name, ty) in params {
            match input.get(*name) {
                None => errs.push(format!("{}: input lacks param {name:?}", p.display())),
                Some(value) => {
                    if let Some(Err(e)) = ty.as_ref().map(|t| t.check_value(value)) {
                        errs.push(format!("{}: input {name}: {e}", p.display()));
                    }
                }
            }
        }
    }
    n
}

/// The largest integer every language reads exactly from JSON (JavaScript's
/// `Number.MAX_SAFE_INTEGER`).
pub const MAX_SAFE_INTEGER: i64 = (1 << 53) - 1;

/// The first number in `value` that is an integer outside ±(2⁵³ − 1), if any.
fn unsafe_integer(value: &serde_json::Value) -> Option<String> {
    use serde_json::Value;
    match value {
        Value::Number(n) => {
            let ok = match (n.as_i64(), n.as_u64(), n.as_f64()) {
                (Some(i), _, _) => i.abs() <= MAX_SAFE_INTEGER,
                (None, Some(_), _) => false,
                // A float with an integral value beyond 2⁵³ is the same trap.
                (None, None, Some(f)) => f.fract() != 0.0 || f.abs() <= MAX_SAFE_INTEGER as f64,
                _ => true,
            };
            (!ok).then(|| n.to_string())
        }
        Value::Array(items) => items.iter().find_map(unsafe_integer),
        Value::Object(map) => map.values().find_map(unsafe_integer),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_whitespace() {
        assert_eq!(
            normalize("  def  f(a:  int) -> int:  "),
            "def f(a: int) -> int:"
        );
    }

    #[test]
    fn integers_stay_within_javascript_safe_range() {
        use serde_json::json;
        assert_eq!(
            unsafe_integer(&json!({"input": {"a": [1, -5, 9007199254740991i64]}})),
            None
        );
        assert_eq!(
            unsafe_integer(&json!({"input": {"a": [[9007199254740992i64]]}})).as_deref(),
            Some("9007199254740992")
        );
        assert!(unsafe_integer(&json!({"output": -9007199254740993i64})).is_some());
        assert!(unsafe_integer(&json!({"output": 18446744073709551615u64})).is_some());
        assert!(unsafe_integer(&json!({"output": 1e20})).is_some());
        assert_eq!(unsafe_integer(&json!({"output": 0.5})), None);
    }

    #[test]
    fn test_values_are_typed() {
        let dir = std::env::temp_dir().join(format!("kbp-typed-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let write = |name: &str, text: &str| fs::write(dir.join(name), text).unwrap();
        write(
            "1.json",
            r#"{"input": {"root": [1, null, 2], "k": 3}, "output": [4]}"#,
        );
        write(
            "2.json",
            r#"{"input": {"root": [1, 2, null], "k": 3}, "output": [4]}"#,
        );
        write(
            "3.json",
            r#"{"input": {"root": [1], "k": "x"}, "output": [null]}"#,
        );
        let params = [
            ("root", Some("TreeNode".parse().unwrap())),
            ("k", Some(kb_judge::Type::Int)),
        ];
        let returns: kb_judge::Type = "ListNode".parse().unwrap();
        let mut errs = Vec::new();
        assert_eq!(check_tests(&dir, &params, Some(&returns), &mut errs), 3);
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(errs.len(), 3, "{errs:?}");
        assert!(
            errs.iter()
                .any(|e| e.contains("2.json: input root") && e.contains("trailing"))
        );
        assert!(
            errs.iter()
                .any(|e| e.contains("3.json: input k: expected an int"))
        );
        assert!(
            errs.iter()
                .any(|e| e.contains("3.json: output: expected a ListNode"))
        );
    }

    #[test]
    fn profile_keys_go_together() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let root = std::env::temp_dir().join(format!("kbp-profile-meta-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let dir = root.join("two-sum");
        copy_dir(&src, &dir);
        let toml = fs::read_to_string(dir.join("problem.toml")).unwrap();
        let errs = |text: &str| {
            fs::write(dir.join("problem.toml"), text).unwrap();
            validate_problem_meta(&dir).1
        };
        assert_eq!(errs(&toml), Vec::<String>::new());
        let sizes_line = toml
            .lines()
            .find(|l| l.starts_with("profile_sizes"))
            .unwrap();
        let no_sizes = toml.replace(
            &format!(
                "{sizes_line}
"
            ),
            "",
        );
        assert!(errs(&no_sizes).contains(&"profile = true requires profile_sizes".to_string()));
        let off = no_sizes.replace(
            "profile = true
",
            "",
        );
        assert!(errs(&off).contains(&"profile/ needs profile = true".to_string()));
        let sizes_only = toml.replace(
            "profile = true
",
            "",
        );
        assert!(errs(&sizes_only).contains(&"profile_sizes needs profile = true".to_string()));
        let bad_sizes = toml.replace(sizes_line, "profile_sizes = [300, 100, 1000, 3000]");
        assert!(
            errs(&bad_sizes)
                .iter()
                .any(|e| e.contains("strictly increasing"))
        );
        fs::remove_dir_all(&root).unwrap();
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

    #[test]
    fn tests_over_the_budget_are_reported() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let base = std::env::temp_dir().join(format!("kbp-budget-{}", std::process::id()));
        let dir = base.join("two-sum");
        let _ = fs::remove_dir_all(&base);
        copy_dir(&src, &dir);
        let today = test_size(&dir).unwrap();
        assert!(today.stored < TEST_BUDGET_BYTES && today.stored * 2 < today.json);
        assert_eq!(budget_error(today), None);
        // Another max-size test of random numbers: they barely compress.
        let mut x: u64 = 0x2545_f491_4f6c_dd1d;
        let nums: Vec<i64> = (0..100_000)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                (x % 2_000_000_001) as i64 - 1_000_000_000
            })
            .collect();
        let big = serde_json::json!({"input": {"nums": nums, "target": 0}, "output": [0, 1]});
        fs::write(dir.join("tests/hidden/99.json"), big.to_string()).unwrap();
        let more = test_size(&dir).unwrap();
        fs::remove_dir_all(&base).unwrap();
        assert!(more.stored > TEST_BUDGET_BYTES, "{more:?}");
        let e = budget_error(more).unwrap();
        assert!(e.contains("over the 512 KB budget"), "{e}");
        assert_eq!(
            size_text(TestBytes {
                json: 2048,
                stored: 256 * 1024
            }),
            "2 KB → 256 KB compressed (50% of 512 KB)"
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

/// Every tag used in `content/problems` has a display name in both UI languages
/// (`tag.names` in apps/desktop/src/i18n/locales), so the UI never shows a raw id.
#[cfg(test)]
mod tag_names {
    use std::path::Path;

    #[test]
    fn every_content_tag_has_a_name_in_both_locales() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let names = |lang: &str| -> serde_json::Map<String, serde_json::Value> {
            let path = repo.join(format!("apps/desktop/src/i18n/locales/{lang}.json"));
            let v: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            v["tag"]["names"].as_object().cloned().unwrap_or_default()
        };
        let (en, es) = (names("en"), names("es"));
        let mut missing = Vec::new();
        for dir in super::problem_dirs(&repo.join("content/problems")).unwrap() {
            let p = kb_judge::Problem::load(&dir).unwrap();
            let toml: toml::Value =
                toml::from_str(&std::fs::read_to_string(dir.join("problem.toml")).unwrap())
                    .unwrap();
            for tag in toml["tags"].as_array().unwrap() {
                let tag = tag.as_str().unwrap();
                if !en.contains_key(tag) || !es.contains_key(tag) {
                    missing.push(format!("{} uses {tag}", p.id));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "add these tags to tag.names in en.json and es.json: {missing:?}"
        );
    }
}
