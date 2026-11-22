//! Reading exercises (expansion M7, ADR 0035): `content/reading/<set>/` holds `set.toml`,
//! `items.json` and each item's code as `<n>.py` and `<n>.ts`. Structure is checked on
//! every validate; `validate --learn --run` also runs every answer in the sandbox:
//! - print items run as programs in both languages and must print exactly the answer;
//! - complexity items count Python line events (`sys.settrace`) of `f(n)` at five sizes,
//!   and the profiler's fit must name the stated time class.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use kb_core::profile::ComplexityClass;
use kb_core::{Language, ProblemMode, TestCase, TestKind};
use kb_judge::JudgeEnv;
use kb_judge::problem::Problem;
use kb_store::reading::{
    PackReadingItem, PackReadingSet, ReadingItemData, ReadingKind, SPACE_CLASSES, TIME_CLASSES,
    normalize_output,
};
use serde::Deserialize;
use serde_json::{Map, Value};

/// Lines per snippet: something to read in one go.
const MAX_LINES: usize = 30;
const RUN_TIME_MS: u32 = 2000;
const RUN_MEMORY_MB: u32 = 256;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SetToml {
    id: String,
    order: u32,
    kind: ReadingKind,
    title: String,
    title_es: String,
    blurb: String,
    blurb_es: String,
}

/// `content/reading` next to a `content/problems` folder.
pub fn reading_dir_for(problems: &Path) -> PathBuf {
    problems
        .parent()
        .map(|p| p.join("reading"))
        .unwrap_or_else(|| PathBuf::from("reading"))
}

fn kebab(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-')
        && !s.ends_with('-')
}

/// The class names `estimate` returns, as items write them.
pub fn class_name(c: ComplexityClass) -> &'static str {
    match c {
        ComplexityClass::Constant => "O(1)",
        ComplexityClass::Logarithmic => "O(log n)",
        ComplexityClass::Linear => "O(n)",
        ComplexityClass::Linearithmic => "O(n log n)",
        ComplexityClass::Quadratic => "O(n²)",
        ComplexityClass::Exponential => "O(2ⁿ)",
    }
}

/// The sizes a time answer is checked at, unless the item names its own.
fn default_sizes(time: &str) -> Vec<u32> {
    match time {
        "O(1)" | "O(log n)" => vec![1 << 6, 1 << 9, 1 << 12, 1 << 15, 1 << 18],
        "O(n²)" => vec![20, 40, 80, 160, 320],
        "O(2ⁿ)" => vec![8, 10, 12, 14, 16],
        _ => vec![200, 400, 800, 1600, 3200],
    }
}

/// A line that can dominate the cost: a loop, or a call of a function defined in the file.
fn is_costly_line(line: &str, code: &str, python: bool) -> bool {
    let t = line.trim_start();
    let looped = if python {
        t.starts_with("for ") || t.starts_with("while ") || t.contains(" for ")
    } else {
        t.starts_with("for ")
            || t.starts_with("for(")
            || t.starts_with("while")
            || t.contains(".forEach(")
            || t.contains(".map(")
            || t.contains(".reduce(")
    };
    if looped {
        return true;
    }
    // A call of a function defined in the snippet (recursion, or a helper with a loop).
    let defined: Vec<&str> = code
        .lines()
        .filter_map(|l| {
            let l = l.trim_start();
            let rest = if python {
                l.strip_prefix("def ")
            } else {
                l.strip_prefix("function ")
            }?;
            rest.split('(').next()
        })
        .collect();
    let is_def = if python {
        t.starts_with("def ")
    } else {
        t.starts_with("function ")
    };
    !is_def && defined.iter().any(|name| t.contains(&format!("{name}(")))
}

/// One set as read from disk.
#[derive(Debug, Clone)]
pub struct SetDir {
    pub dir: PathBuf,
    pub set: PackReadingSet,
}

fn check_item(kind: ReadingKind, it: &ReadingItemData, py: &str, ts: &str, errs: &mut Vec<String>) {
    let at = |m: &str| format!("item {}: {m}", it.n);
    for (what, v) in [
        ("prompt", &it.prompt),
        ("prompt_es", &it.prompt_es),
        ("explain", &it.explain),
        ("explain_es", &it.explain_es),
    ] {
        if v.trim().is_empty() {
            errs.push(at(&format!("empty {what}")));
        }
    }
    for (file, code) in [("py", py), ("ts", ts)] {
        let lines = code.lines().count();
        if code.trim().is_empty() || lines > MAX_LINES {
            errs.push(at(&format!(
                "{}.{file} must have 1-{MAX_LINES} lines",
                it.n
            )));
        }
    }
    match kind {
        ReadingKind::Print => {
            let both = it.answer_py.is_some() && it.answer_ts.is_some();
            let one = it.answer.is_some();
            if one == both || (!one && (it.answer_py.is_some() || it.answer_ts.is_some()) && !both)
            {
                errs.push(at("give `answer`, or both `answer_py` and `answer_ts`"));
            }
            if both && it.answer_py == it.answer_ts {
                errs.push(at("answer_py equals answer_ts: use `answer`"));
            }
            if !it.choices.is_empty() {
                let unique: BTreeSet<String> =
                    it.choices.iter().map(|c| normalize_output(c)).collect();
                if !(3..=4).contains(&it.choices.len()) || unique.len() != it.choices.len() {
                    errs.push(at("choices: 3-4 different outputs"));
                }
                match &it.answer {
                    Some(a) if unique.contains(&normalize_output(a)) => {}
                    Some(_) => errs.push(at("the answer is not among the choices")),
                    None => errs.push(at("choices need one `answer` for both languages")),
                }
            }
            if it.time.is_some()
                || it.space.is_some()
                || it.dominant.is_some()
                || !it.sizes.is_empty()
            {
                errs.push(at(
                    "time / space / dominant / sizes are for complexity items",
                ));
            }
        }
        ReadingKind::Complexity => {
            if !it
                .time
                .as_deref()
                .is_some_and(|t| TIME_CLASSES.contains(&t))
            {
                errs.push(at(&format!(
                    "time must be one of {}",
                    TIME_CLASSES.join(", ")
                )));
            }
            if !it
                .space
                .as_deref()
                .is_some_and(|s| SPACE_CLASSES.contains(&s))
            {
                errs.push(at(&format!(
                    "space must be one of {}",
                    SPACE_CLASSES.join(", ")
                )));
            }
            if it.answer.is_some()
                || it.answer_py.is_some()
                || it.answer_ts.is_some()
                || !it.choices.is_empty()
            {
                errs.push(at("answers and choices are for print items"));
            }
            if !py.lines().any(|l| l.starts_with("def f(n")) {
                errs.push(at(
                    "the Python version defines `def f(n…)` at the top level",
                ));
            }
            if !ts.lines().any(|l| l.starts_with("function f(n")) {
                errs.push(at(
                    "the TypeScript version defines `function f(n…)` at the top level",
                ));
            }
            if !it.sizes.is_empty()
                && (it.sizes.len() < 4 || it.sizes.windows(2).any(|w| w[0] >= w[1]))
            {
                errs.push(at("sizes: at least 4, increasing"));
            }
            match it.dominant {
                None => errs.push(at("missing dominant (the costly line in each version)")),
                Some(d) => {
                    for (code, line, python, file) in
                        [(py, d.python, true, "py"), (ts, d.typescript, false, "ts")]
                    {
                        match code.lines().nth(line.saturating_sub(1) as usize) {
                            Some(l) if line > 0 && is_costly_line(l, code, python) => {}
                            _ => errs.push(at(&format!(
                                "dominant line {line} of {}.{file} is not a loop or a call of a function in the snippet",
                                it.n
                            ))),
                        }
                    }
                }
            }
        }
    }
}

fn read_set(dir: &Path) -> Result<SetDir, Vec<String>> {
    let mut errs = Vec::new();
    let meta: SetToml = fs::read_to_string(dir.join("set.toml"))
        .map_err(|e| vec![format!("set.toml: {e}")])
        .and_then(|s| toml::from_str(&s).map_err(|e| vec![format!("set.toml: {e}")]))?;
    let folder = dir
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    if meta.id != folder || !kebab(&meta.id) {
        errs.push(format!(
            "id {:?} must be kebab-case and match the folder",
            meta.id
        ));
    }
    for (what, v) in [
        ("title", &meta.title),
        ("title_es", &meta.title_es),
        ("blurb", &meta.blurb),
        ("blurb_es", &meta.blurb_es),
    ] {
        if v.trim().is_empty() {
            errs.push(format!("set.toml: empty {what}"));
        }
    }
    let items: Vec<ReadingItemData> = match fs::read_to_string(dir.join("items.json"))
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
    {
        Ok(i) => i,
        Err(e) => return Err(vec![format!("items.json: {e}")]),
    };
    if items.is_empty() {
        errs.push("items.json: no items".into());
    }
    let mut out = Vec::new();
    for (k, it) in items.into_iter().enumerate() {
        if it.n != k as u32 + 1 {
            errs.push(format!("items.json: item {} must have n = {}", it.n, k + 1));
        }
        let read =
            |ext: &str| fs::read_to_string(dir.join(format!("{}.{ext}", it.n))).unwrap_or_default();
        let (py, ts) = (read("py"), read("ts"));
        check_item(meta.kind, &it, &py, &ts, &mut errs);
        out.push(PackReadingItem {
            code_py: py,
            code_ts: ts,
            data: it,
        });
    }
    if !errs.is_empty() {
        return Err(errs);
    }
    Ok(SetDir {
        dir: dir.to_path_buf(),
        set: PackReadingSet {
            id: meta.id,
            ord: meta.order,
            kind: meta.kind,
            title_en: meta.title,
            title_es: meta.title_es,
            blurb_en: meta.blurb,
            blurb_es: meta.blurb_es,
            items: out,
        },
    })
}

/// Every set under `root` (none when the folder is missing). `Err` lists every problem.
pub fn read_reading(root: &Path) -> Result<Vec<SetDir>, Vec<String>> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut dirs: Vec<PathBuf> = fs::read_dir(root)
        .map(|r| {
            r.flatten()
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect()
        })
        .unwrap_or_default();
    dirs.sort();
    let mut sets = Vec::new();
    let mut errors = Vec::new();
    for d in dirs {
        match read_set(&d) {
            Ok(s) => sets.push(s),
            Err(es) => errors.extend(es.into_iter().map(|e| format!("{}: {e}", d.display()))),
        }
    }
    let mut orders = BTreeSet::new();
    for s in &sets {
        if !orders.insert(s.set.ord) {
            errors.push(format!("{}: order {} used twice", s.set.id, s.set.ord));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    sets.sort_by_key(|s| s.set.ord);
    Ok(sets)
}

/// Run `program` once on empty stdin; its stdout, or why it did not finish.
fn run_program(program: &str, language: Language, env: &JudgeEnv) -> Result<String, String> {
    let mut input = Map::new();
    input.insert(
        kb_judge::problem::STDIN_PARAM.into(),
        Value::String(String::new()),
    );
    // Program tests expect text; the output is read back whatever the verdict.
    let test = TestCase {
        kind: TestKind::Custom,
        index: 1,
        input,
        expected: Value::String(String::new()),
    };
    let problem = Problem::from_parts(
        "reading-check",
        ProblemMode::Program,
        &Value::Null,
        "lines",
        RUN_TIME_MS,
        RUN_MEMORY_MB,
        vec![test.clone()],
    )
    .map_err(|e| e.to_string())?;
    let report =
        kb_judge::judge(&problem, &[test], language, program, env).map_err(|e| e.to_string())?;
    let t = report
        .tests
        .first()
        .ok_or_else(|| format!("{:?}", report.verdict))?;
    if !matches!(
        t.result.verdict,
        kb_core::Verdict::Accepted | kb_core::Verdict::WrongAnswer
    ) {
        let why = t.stderr.clone().unwrap_or_default();
        return Err(format!(
            "{:?} {}",
            t.result.verdict,
            why.lines().last().unwrap_or("")
        ));
    }
    Ok(match &t.actual {
        Some(Value::String(s)) => s.clone(),
        _ => t.stdout.clone(),
    })
}

/// The Python program that counts line events of `f(n)` at each size.
pub fn counting_program(code: &str, sizes: &[u32]) -> String {
    const DRIVER: &str = r#"

def _kb_count(n):
    hits = [0]

    def _kb_line(frame, event, arg):
        if event == "line":
            hits[0] += 1
        return _kb_line

    def _kb_call(frame, event, arg):
        return None if frame.f_code.co_name.startswith("_kb") else _kb_line

    sys.settrace(_kb_call)
    try:
        f(n)
    finally:
        sys.settrace(None)
    return hits[0]

print(" ".join(str(_kb_count(n)) for n in [SIZES]))
"#;
    let sizes: Vec<String> = sizes.iter().map(u32::to_string).collect();
    format!(
        "import sys\nsys.setrecursionlimit(100000)\n{}{}",
        code.trim_end(),
        DRIVER.replace("SIZES", &sizes.join(", "))
    )
}

/// Check one set's answers in the sandbox; prints one line per item, returns failures.
pub fn run_set(s: &SetDir, env: &JudgeEnv) -> usize {
    let mut failed = 0;
    for it in &s.set.items {
        let d = &it.data;
        let result: Result<String, String> = match s.set.kind {
            ReadingKind::Print => (|| {
                for (lang, code, ts) in [
                    (Language::Python, &it.code_py, false),
                    (Language::TypeScript, &it.code_ts, true),
                ] {
                    let got = run_program(code, lang, env)?;
                    let want = d.answer_for(ts).unwrap_or_default();
                    if normalize_output(&got) != normalize_output(want) {
                        return Err(format!(
                            "{} prints {:?}, the answer says {:?}",
                            lang.id(),
                            got,
                            want
                        ));
                    }
                }
                Ok("both print the answer".into())
            })(),
            ReadingKind::Complexity => (|| {
                let time = d.time.as_deref().unwrap_or_default();
                let sizes = if d.sizes.is_empty() {
                    default_sizes(time)
                } else {
                    d.sizes.clone()
                };
                let out = run_program(
                    &counting_program(&it.code_py, &sizes),
                    Language::Python,
                    env,
                )?;
                // The TypeScript version must at least run.
                run_program(
                    &format!("{}\nf(8);\n", it.code_ts.trim_end()),
                    Language::TypeScript,
                    env,
                )?;
                let counts: Vec<u32> = out
                    .split_whitespace()
                    .filter_map(|c| c.parse().ok())
                    .collect();
                if counts.len() != sizes.len() {
                    return Err(format!("unexpected counter output {out:?}"));
                }
                let points: Vec<(u32, u32)> =
                    sizes.iter().copied().zip(counts.iter().copied()).collect();
                let est = kb_judge::complexity::estimate(&points).ok_or("too few sizes")?;
                let fitted = class_name(est.class);
                if fitted != time {
                    return Err(format!(
                        "line counts {counts:?} at {sizes:?} fit {fitted}, the answer says {time}"
                    ));
                }
                Ok(format!("line counts fit {fitted}"))
            })(),
        };
        match result {
            Ok(m) => println!("ok    reading {}/{} {m}", s.set.id, d.n),
            Err(e) => {
                failed += 1;
                println!("FAIL  reading {}/{} {e}", s.set.id, d.n);
            }
        }
    }
    failed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn costly_lines_are_loops_or_calls_of_snippet_functions() {
        let py = "def f(n):\n    total = 0\n    for i in range(n):\n        total += g(i)\n    return total\n\ndef g(i):\n    return i\n";
        assert!(is_costly_line("    for i in range(n):", py, true));
        assert!(is_costly_line("        total += g(i)", py, true));
        assert!(!is_costly_line("    total = 0", py, true));
        assert!(!is_costly_line("def g(i):", py, true));
        let ts = "function f(n: number): number {\n  let i = 1;\n  while (i < n) i *= 2;\n  return i;\n}\n";
        assert!(is_costly_line("  while (i < n) i *= 2;", ts, false));
        assert!(!is_costly_line("  let i = 1;", ts, false));
    }

    #[test]
    fn the_counting_program_wraps_the_snippet() {
        let p = counting_program("def f(n):\n    return n\n", &[1, 2]);
        assert!(p.starts_with("import sys\n"));
        assert!(p.contains("def f(n):\n    return n\n\ndef _kb_count"));
        assert!(p.ends_with("for n in [1, 2]))\n"));
    }

    #[test]
    fn class_names_cover_the_answers() {
        for c in ComplexityClass::ALL {
            assert!(TIME_CLASSES.contains(&class_name(c)));
        }
    }
}
