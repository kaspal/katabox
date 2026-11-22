//! The mistake journal and reflections (expansion M3, ADR 0034). After a failed Submit
//! KataBox tags the failure where it can tell (time, memory, the runtime error's kind,
//! and the shape of the failing input: empty, one element, the largest value, negative,
//! zero, duplicates); the user can change the tags and add a note. After Accepted, an
//! optional reflection ("why it works" and a complexity pick) is shown again in review
//! mode, katas and the Notes tab. Everything stays in user.db.

use std::collections::{HashMap, HashSet};

use kb_core::{Report, TestCase, Verdict};
use kb_judge::{Signature, Type};
use kb_store::UiLang;
use kb_store::journal::{MistakeRow, ReflectionRow};
use serde::Serialize;
use serde_json::Value;

use crate::{App, AppError, AppErrorCode, Result};

/// Every tag, in display order. Users pick from the same set.
pub const TAGS: [&str; 21] = [
    "wa",
    "tle",
    "mle",
    "ole",
    "compile",
    "runtime:index",
    "runtime:key",
    "runtime:null",
    "runtime:zero_division",
    "runtime:recursion",
    "runtime:overflow",
    "runtime:type",
    "runtime:other",
    "edge:empty",
    "edge:single",
    "edge:max",
    "edge:negative",
    "edge:zero",
    "edge:duplicates",
    "shrunk",
    "logic",
];

/// The longest note, in characters.
pub const NOTE_MAX: usize = 2000;
/// The longest "why it works", in characters.
pub const WHY_MAX: usize = 500;
/// The complexity classes a reflection may pick.
pub const CLASSES: [&str; 9] = [
    "O(1)",
    "O(log n)",
    "O(n)",
    "O(n log n)",
    "O(n^2)",
    "O(n^3)",
    "O(2^n)",
    "O(n!)",
    "other",
];

/// The kind of a runtime error, from its message and stderr (Python, TypeScript, PHP
/// and Rust wordings).
pub fn runtime_kind(text: &str) -> &'static str {
    let has = |s: &str| text.contains(s);
    if has("RecursionError")
        || has("Maximum call stack")
        || has("stack overflow")
        || has("Maximum function nesting")
    {
        "runtime:recursion"
    } else if has("ZeroDivisionError")
        || has("DivisionByZeroError")
        || has("divide by zero")
        || has("by zero")
    {
        "runtime:zero_division"
    } else if has("IndexError")
        || has("index out of bounds")
        || has("Undefined array key")
        || has("Undefined offset")
        || has("RangeError: Invalid array length")
    {
        "runtime:index"
    } else if has("KeyError") {
        "runtime:key"
    } else if has("NoneType")
        || has("of undefined")
        || has("of null")
        || has("on a `None` value")
        || has("on null")
        || has("is not a function")
    {
        "runtime:null"
    } else if has("overflow") {
        "runtime:overflow"
    } else if has("TypeError") || has("ValueError") {
        "runtime:type"
    } else {
        "runtime:other"
    }
}

/// The largest absolute integer and the longest array or string seen for one parameter
/// across a problem's tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParamMax {
    pub abs_int: u64,
    pub len: usize,
}

fn scan(v: &Value, m: &mut ParamMax) {
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                m.abs_int = m.abs_int.max(i.unsigned_abs());
            }
        }
        Value::String(s) => m.len = m.len.max(s.chars().count()),
        Value::Array(a) => {
            m.len = m.len.max(a.len());
            for x in a {
                scan(x, m);
            }
        }
        _ => {}
    }
}

/// Per parameter, the largest values the problem's tests use.
pub fn param_maxes(tests: &[TestCase]) -> HashMap<String, ParamMax> {
    let mut out: HashMap<String, ParamMax> = HashMap::new();
    for t in tests {
        for (k, v) in &t.input {
            scan(v, out.entry(k.clone()).or_default());
        }
    }
    out
}

fn any_int(v: &Value, f: &dyn Fn(i64) -> bool) -> bool {
    match v {
        Value::Number(n) => n.as_i64().is_some_and(f),
        Value::Array(a) => a.iter().any(|x| any_int(x, f)),
        _ => false,
    }
}

fn has_duplicates(a: &[Value]) -> bool {
    let mut seen = HashSet::new();
    a.iter()
        .filter(|v| !v.is_array() && !v.is_object() && !v.is_null())
        .any(|v| !seen.insert(v.to_string()))
}

/// The edge-case tags of a failing input, from its parameters' shapes. Lists and
/// trees travel as arrays (ADR 0025).
pub fn edge_tags(
    input: &serde_json::Map<String, Value>,
    sig: &Signature,
    maxes: &HashMap<String, ParamMax>,
) -> Vec<&'static str> {
    let mut tags: Vec<&'static str> = Vec::new();
    let mut add = |t: &'static str| {
        if !tags.contains(&t) {
            tags.push(t);
        }
    };
    for p in &sig.params {
        let Some(v) = input.get(&p.name) else {
            continue;
        };
        let max = maxes.get(&p.name).copied().unwrap_or_default();
        match v {
            Value::Array(a) => {
                if a.is_empty() {
                    add("edge:empty");
                } else if a.len() == 1 {
                    add("edge:single");
                }
                if matches!(p.ty, Type::Array(_)) && has_duplicates(a) {
                    add("edge:duplicates");
                }
                if max.len > 1 && a.len() == max.len {
                    add("edge:max");
                }
            }
            Value::String(s) => {
                let n = s.chars().count();
                if n == 0 {
                    add("edge:empty");
                } else if n == 1 {
                    add("edge:single");
                }
                if max.len > 1 && n == max.len {
                    add("edge:max");
                }
            }
            Value::Number(n) if n.as_i64() == Some(0) => add("edge:zero"),
            _ => {}
        }
        if any_int(v, &|i| i < 0) {
            add("edge:negative");
        }
        if max.abs_int > 1 && any_int(v, &|i| i.unsigned_abs() == max.abs_int) {
            add("edge:max");
        }
    }
    tags
}

/// A function problem's signature and its parameters' largest test values.
type ProblemShape = (Signature, HashMap<String, ParamMax>);

/// The automatic tags of a failed report. `sig` and `maxes` are for function problems
/// (edge cases come from the failing input).
pub fn auto_tags(
    report: &Report,
    sig: Option<&Signature>,
    maxes: &HashMap<String, ParamMax>,
) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    match report.verdict {
        Verdict::Accepted => return tags,
        Verdict::WrongAnswer => tags.push("wa".into()),
        Verdict::TimeLimitExceeded => tags.push("tle".into()),
        Verdict::MemoryLimitExceeded => tags.push("mle".into()),
        Verdict::OutputLimitExceeded => tags.push("ole".into()),
        Verdict::CompilationError => tags.push("compile".into()),
        Verdict::RuntimeError => {
            let f = report.first_failure.as_ref();
            let text = format!(
                "{} {}",
                f.and_then(|f| f.message.as_ref())
                    .map_or("", |m| m.text.as_str()),
                f.and_then(|f| f.stderr.as_deref()).unwrap_or("")
            );
            tags.push(runtime_kind(&text).into());
        }
        Verdict::InternalError => {}
    }
    if let (Some(sig), Some(f)) = (sig, report.first_failure.as_ref())
        && !matches!(
            report.verdict,
            Verdict::CompilationError | Verdict::InternalError
        )
    {
        tags.extend(
            edge_tags(&f.input, sig, maxes)
                .into_iter()
                .map(String::from),
        );
    }
    tags
}

// --- Views and the App --------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MistakeExample {
    pub problem_id: String,
    pub title: String,
    pub language: String,
    pub submitted_at: i64,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MistakeTagCount {
    pub tag: String,
    pub count: u32,
    /// The latest few, newest first.
    pub examples: Vec<MistakeExample>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct JournalOverview {
    /// Failed submissions in the journal.
    pub failures: u32,
    /// Tags by count (then display order); only tags in use.
    pub tags: Vec<MistakeTagCount>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MistakeView {
    pub auto_tags: Vec<String>,
    pub tags: Vec<String>,
    pub note: String,
}

fn bad(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, m.into())
}

fn check_tags(tags: &[String]) -> Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for t in tags {
        if !TAGS.contains(&t.as_str()) {
            return Err(bad(format!("unknown tag {t:?}")));
        }
        if !out.contains(t) {
            out.push(t.clone());
        }
    }
    Ok(out)
}

impl App {
    /// Tag every failed submission that has no journal entry yet (new failures, and
    /// those made before the journal existed).
    pub(crate) async fn tag_new_failures(&self) -> Result<()> {
        let open = self.store.user.untagged_failures().await?;
        let mut cache: HashMap<String, Option<ProblemShape>> = HashMap::new();
        for f in open {
            if !cache.contains_key(&f.problem_id) {
                let info = match self.problem(&f.problem_id).await {
                    Ok(p) if p.mode == kb_core::ProblemMode::Function => {
                        let maxes = param_maxes(&p.tests);
                        Some((p.signature, maxes))
                    }
                    _ => None,
                };
                cache.insert(f.problem_id.clone(), info);
            }
            let tags = match serde_json::from_value::<Report>(f.report.clone()) {
                Ok(report) => {
                    let info = cache.get(&f.problem_id).and_then(Option::as_ref);
                    let empty = HashMap::new();
                    auto_tags(&report, info.map(|i| &i.0), info.map_or(&empty, |i| &i.1))
                }
                Err(_) => Vec::new(),
            };
            self.store
                .user
                .add_mistake(&f.problem_id, &f.language, f.submitted_at, &tags)
                .await?;
        }
        Ok(())
    }

    /// Progress → Mistakes: counts per tag with the latest examples.
    pub async fn journal_overview(&self, lang: UiLang) -> Result<JournalOverview> {
        self.tag_new_failures().await?;
        let rows = self.store.user.mistakes().await?;
        let titles: HashMap<String, String> = self
            .all_problems(lang)
            .await?
            .into_iter()
            .map(|p| (p.id, p.title))
            .collect();
        let mut tags: Vec<MistakeTagCount> = Vec::new();
        for m in &rows {
            for t in &m.tags {
                let e = match tags.iter_mut().position(|x| &x.tag == t) {
                    Some(i) => &mut tags[i],
                    None => {
                        tags.push(MistakeTagCount {
                            tag: t.clone(),
                            count: 0,
                            examples: Vec::new(),
                        });
                        tags.last_mut().expect("just pushed")
                    }
                };
                e.count += 1;
                if e.examples.len() < 3 {
                    e.examples.push(MistakeExample {
                        problem_id: m.problem_id.clone(),
                        title: titles
                            .get(&m.problem_id)
                            .cloned()
                            .unwrap_or_else(|| m.problem_id.clone()),
                        language: m.language.clone(),
                        submitted_at: m.submitted_at,
                        note: m.note.clone(),
                    });
                }
            }
        }
        let order = |t: &str| TAGS.iter().position(|x| *x == t).unwrap_or(usize::MAX);
        tags.sort_by(|a, b| {
            b.count
                .cmp(&a.count)
                .then_with(|| order(&a.tag).cmp(&order(&b.tag)))
        });
        Ok(JournalOverview {
            failures: u32::try_from(rows.len()).unwrap_or(u32::MAX),
            tags,
        })
    }

    async fn mistake_key(&self, submission: i64) -> Result<(String, String, i64, bool)> {
        let s = self.store.user.get_submission(submission).await?;
        self.ensure_unlocked(&s.summary.problem_id).await?;
        Ok((
            s.summary.problem_id,
            s.summary.language.id().to_string(),
            s.summary.created_at,
            s.summary.verdict.is_accepted(),
        ))
    }

    /// A submission's journal entry (`None` for an Accepted one).
    pub async fn submission_mistake(&self, submission: i64) -> Result<Option<MistakeView>> {
        let (p, l, at, ok) = self.mistake_key(submission).await?;
        if ok {
            return Ok(None);
        }
        self.tag_new_failures().await?;
        Ok(self
            .store
            .user
            .mistake(&p, &l, at)
            .await?
            .map(|m| MistakeView {
                auto_tags: m.auto_tags,
                tags: m.tags,
                note: m.note,
            }))
    }

    /// The user's tags and note for a failed submission.
    pub async fn set_submission_mistake(
        &self,
        submission: i64,
        tags: &[String],
        note: &str,
    ) -> Result<()> {
        let (p, l, at, ok) = self.mistake_key(submission).await?;
        if ok {
            return Err(bad("an Accepted submission has no mistakes to tag"));
        }
        if note.chars().count() > NOTE_MAX {
            return Err(bad(format!("a note has at most {NOTE_MAX} characters")));
        }
        let tags = check_tags(tags)?;
        self.tag_new_failures().await?;
        Ok(self.store.user.set_mistake(&p, &l, at, &tags, note).await?)
    }

    /// The shrinker found a smaller failing input for this submission's failure.
    pub async fn tag_shrunk(&self, submission: i64) -> Result<()> {
        let (p, l, at, ok) = self.mistake_key(submission).await?;
        if ok {
            return Ok(());
        }
        self.tag_new_failures().await?;
        Ok(self
            .store
            .user
            .add_mistake_tag(&p, &l, at, "shrunk")
            .await?)
    }

    /// The problem's reflection. During a kata it stays hidden until the problem is
    /// solved in that kata.
    pub async fn reflection(&self, problem: &str) -> Result<Option<ReflectionRow>> {
        if let Some(k) = self.running_kata().await?
            && k.problem_ids.iter().any(|p| p == problem)
        {
            let solved = self.store.user.kata_attempts().await?.iter().any(|a| {
                a.session_started_at == k.started_at
                    && a.problem_id == problem
                    && a.solved_at.is_some()
            });
            if !solved {
                return Err(crate::kata::locked());
            }
        }
        if self.in_interview(problem).await? {
            return Err(crate::interview::locked());
        }
        Ok(self.store.user.reflection(problem).await?)
    }

    pub async fn set_reflection(
        &self,
        problem: &str,
        why: &str,
        time_class: Option<&str>,
        space_class: Option<&str>,
    ) -> Result<()> {
        if why.chars().count() > WHY_MAX {
            return Err(bad(format!("at most {WHY_MAX} characters")));
        }
        for c in [time_class, space_class].into_iter().flatten() {
            if !CLASSES.contains(&c) {
                return Err(bad(format!("unknown complexity {c:?}")));
            }
        }
        Ok(self
            .store
            .user
            .set_reflection(problem, why.trim(), time_class, space_class)
            .await?)
    }
}

/// Merge-free helper for tests and export checks: is `row` a valid journal entry?
pub(crate) fn valid_mistake(row: &MistakeRow) -> bool {
    row.tags
        .iter()
        .chain(&row.auto_tags)
        .all(|t| TAGS.contains(&t.as_str()))
        && row.note.chars().count() <= NOTE_MAX
}

pub(crate) fn valid_reflection(row: &ReflectionRow) -> bool {
    row.why.chars().count() <= WHY_MAX
        && [&row.time_class, &row.space_class]
            .into_iter()
            .flatten()
            .all(|c| CLASSES.contains(&c.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::{Failure, Message, MessageCode, TestKind};
    use serde_json::json;

    fn sig() -> Signature {
        Signature::from_json(&json!({
            "name": "f",
            "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}, {"name": "s", "type": "string"}],
            "returns": "int"
        }))
        .unwrap()
    }

    fn report(
        verdict: Verdict,
        input: Value,
        stderr: Option<&str>,
        message: Option<&str>,
    ) -> Report {
        Report {
            verdict,
            tests: vec![],
            total_tests: 3,
            first_failure: Some(Failure {
                kind: TestKind::Hidden,
                index: 1,
                verdict,
                input: input.as_object().unwrap().clone(),
                expected: json!(0),
                actual: None,
                message: message.map(|m| Message::new(MessageCode::Exception, m)),
                stdout: String::new(),
                stderr: stderr.map(str::to_string),
            }),
            compile_output: None,
            time_us: 0,
            cpu_us: 0,
            memory_kb: 0,
            compile: None,
        }
    }

    fn maxes() -> HashMap<String, ParamMax> {
        let t = |input: Value| TestCase {
            kind: TestKind::Hidden,
            index: 1,
            input: input.as_object().unwrap().clone(),
            expected: json!(0),
        };
        param_maxes(&[
            t(json!({"nums": [1, 2, 3], "k": 5, "s": "abcd"})),
            t(json!({"nums": [1000000000, -4, 7, 7, 1], "k": 2, "s": "ab"})),
        ])
    }

    #[test]
    fn maxes_scan_every_test() {
        let m = maxes();
        assert_eq!(
            m["nums"],
            ParamMax {
                abs_int: 1_000_000_000,
                len: 5
            }
        );
        assert_eq!(m["s"], ParamMax { abs_int: 0, len: 4 });
    }

    #[test]
    fn edge_cases_from_the_failing_input() {
        let s = sig();
        let m = maxes();
        let tags = |input: Value| {
            auto_tags(
                &report(Verdict::WrongAnswer, input, None, None),
                Some(&s),
                &m,
            )
        };
        assert_eq!(
            tags(json!({"nums": [], "k": 3, "s": "x"})),
            ["wa", "edge:empty", "edge:single"]
        );
        assert_eq!(
            tags(json!({"nums": [5], "k": 0, "s": "abc"})),
            ["wa", "edge:single", "edge:zero"]
        );
        assert_eq!(
            tags(json!({"nums": [2, -1, 2], "k": 1, "s": "abcd"})),
            ["wa", "edge:duplicates", "edge:negative", "edge:max"]
        );
        assert_eq!(
            tags(json!({"nums": [3, 1000000000], "k": 1, "s": "ab"})),
            ["wa", "edge:max"]
        );
        assert_eq!(tags(json!({"nums": [3, 4], "k": 1, "s": "ab"})), ["wa"]);
        // No signature (a program problem): the verdict only.
        assert_eq!(
            auto_tags(
                &report(Verdict::TimeLimitExceeded, json!({"stdin": ""}), None, None),
                None,
                &m
            ),
            ["tle"]
        );
    }

    #[test]
    fn runtime_errors_by_language() {
        let cases = [
            (
                "Traceback ...\nIndexError: list index out of range",
                "runtime:index",
            ),
            ("KeyError: 'x'", "runtime:key"),
            (
                "TypeError: 'NoneType' object is not subscriptable",
                "runtime:null",
            ),
            (
                "ZeroDivisionError: division by zero",
                "runtime:zero_division",
            ),
            (
                "RecursionError: maximum recursion depth exceeded",
                "runtime:recursion",
            ),
            (
                "RangeError: Maximum call stack size exceeded",
                "runtime:recursion",
            ),
            (
                "TypeError: Cannot read properties of undefined (reading 'next')",
                "runtime:null",
            ),
            (
                "thread 'main' panicked at src/solution.rs:4:9:\nindex out of bounds: the len is 3",
                "runtime:index",
            ),
            ("attempt to add with overflow", "runtime:overflow"),
            (
                "called `Option::unwrap()` on a `None` value",
                "runtime:null",
            ),
            (
                "PHP Fatal error:  Uncaught DivisionByZeroError: Modulo by zero",
                "runtime:zero_division",
            ),
            ("Warning: Undefined array key 5", "runtime:index"),
            ("ValueError: invalid literal for int()", "runtime:type"),
            ("something odd", "runtime:other"),
        ];
        for (text, want) in cases {
            assert_eq!(runtime_kind(text), want, "{text}");
            let r = report(
                Verdict::RuntimeError,
                json!({"nums": [1, 2], "k": 1, "s": "ab"}),
                Some(text),
                None,
            );
            assert_eq!(auto_tags(&r, None, &HashMap::new()), [want]);
        }
        // The message text counts too.
        let r = report(
            Verdict::RuntimeError,
            json!({"nums": [1, 2], "k": 1, "s": "ab"}),
            None,
            Some("KeyError: 3"),
        );
        assert_eq!(auto_tags(&r, None, &HashMap::new()), ["runtime:key"]);
    }

    #[test]
    fn tags_and_reflections_are_checked() {
        assert_eq!(
            check_tags(&["wa".into(), "wa".into(), "logic".into()]).unwrap(),
            ["wa", "logic"]
        );
        assert!(check_tags(&["<script>".into()]).is_err());
        assert!(valid_reflection(&ReflectionRow {
            problem_id: "p".into(),
            why: "x".into(),
            time_class: Some("O(n)".into()),
            space_class: None,
            updated_at: 0,
        }));
        assert!(!valid_reflection(&ReflectionRow {
            problem_id: "p".into(),
            why: "x".into(),
            time_class: Some("O(fast)".into()),
            space_class: None,
            updated_at: 0,
        }));
    }
}
