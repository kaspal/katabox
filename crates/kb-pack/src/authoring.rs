//! The in-app problem editor (expansion M13, ADR 0040): a form's draft, written as an
//! ordinary function-mode package that the same validation as packs checks, and the
//! validator's messages mapped back to the form's fields.
//!
//! The editor makes pack-quality problems: one reference solution is enough (built-in
//! content needs all four, plus the scripts kb-pack's sandbox checks use).

use std::fs;
use std::path::Path;

use kb_core::{Difficulty, Language};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The longest problem id the editor accepts: `<pack>--<id>` must stay within 64.
pub const MAX_DRAFT_ID: usize = 40;
/// At most this many tests of each kind in a draft.
pub const MAX_DRAFT_TESTS: usize = 60;
/// The largest text field (statement, editorial, a hint, the reference), in bytes.
pub const MAX_DRAFT_TEXT: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct DraftParam {
    pub name: String,
    /// A judge type such as `int[]` or `TreeNode`.
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct DraftTest {
    pub input: Map<String, Value>,
    pub output: Value,
}

/// Everything the editor's form holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct ProblemDraft {
    pub id: String,
    pub title: String,
    pub title_es: String,
    pub difficulty: Difficulty,
    pub pattern: String,
    pub tags: Vec<String>,
    pub rank: u8,
    pub function: String,
    pub params: Vec<DraftParam>,
    pub returns: String,
    pub checker: String,
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    pub statement: String,
    pub statement_es: String,
    /// Up to three tiers, from a nudge to almost the answer.
    pub hints: Vec<String>,
    pub hints_es: Vec<String>,
    pub editorial: String,
    pub editorial_es: String,
    pub public_tests: Vec<DraftTest>,
    pub hidden_tests: Vec<DraftTest>,
    pub reference_language: Language,
    pub reference: String,
}

impl ProblemDraft {
    /// A new, empty draft with sensible defaults.
    pub fn new(id: &str) -> ProblemDraft {
        ProblemDraft {
            id: id.to_string(),
            title: String::new(),
            title_es: String::new(),
            difficulty: Difficulty::Easy,
            pattern: "arrays-hashing".into(),
            tags: Vec::new(),
            rank: 5,
            function: "solve".into(),
            params: vec![DraftParam {
                name: "nums".into(),
                ty: "int[]".into(),
            }],
            returns: "int".into(),
            checker: "exact".into(),
            time_limit_ms: 1000,
            memory_mb: 256,
            statement: String::new(),
            statement_es: String::new(),
            hints: vec![String::new(); 3],
            hints_es: vec![String::new(); 3],
            editorial: String::new(),
            editorial_es: String::new(),
            public_tests: Vec::new(),
            hidden_tests: Vec::new(),
            reference_language: Language::Python,
            reference: String::new(),
        }
    }
}

/// One problem with the draft, tied to the form field it belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DraftIssue {
    /// A form field (`title`, `signature`, `public_tests`, …) or `general`.
    pub field: String,
    pub message: String,
}

fn issue(field: &str, message: impl Into<String>) -> DraftIssue {
    DraftIssue {
        field: field.into(),
        message: message.into(),
    }
}

/// `[a-z0-9]+(-[a-z0-9]+)*`, at most [`MAX_DRAFT_ID`] characters: a folder name on every
/// system, and never `--` (kept for follow-ups and packs).
pub fn valid_draft_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= MAX_DRAFT_ID
        && id.split('-').all(|p| {
            !p.is_empty()
                && p.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

/// The function-mode patterns a problem may use (SQL patterns are for SQL problems).
pub fn patterns() -> Vec<String> {
    crate::validate::function_patterns()
}

/// Checks the package validator can't phrase per field: empty texts and sizes.
pub fn draft_issues(d: &ProblemDraft) -> Vec<DraftIssue> {
    let mut out = Vec::new();
    if !valid_draft_id(&d.id) {
        out.push(issue(
            "id",
            format!(
                "lowercase letters, digits and single dashes, at most {MAX_DRAFT_ID} characters"
            ),
        ));
    }
    for (field, text) in [
        ("statement", &d.statement),
        ("statement_es", &d.statement_es),
        ("editorial", &d.editorial),
    ] {
        if text.trim().is_empty() {
            out.push(issue(field, "write this text"));
        }
    }
    if d.hints.iter().all(|h| h.trim().is_empty()) {
        out.push(issue("hints", "write at least one hint"));
    }
    if d.hints.len() > 3 || d.hints_es.len() > 3 {
        out.push(issue("hints", "at most three hint tiers"));
    }
    if d.reference.trim().is_empty() {
        out.push(issue("reference", "write the reference solution"));
    }
    if !kb_judge::languages::SUPPORTED.contains(&d.reference_language) {
        out.push(issue("reference", "pick a judge language"));
    }
    for (field, tests) in [
        ("public_tests", &d.public_tests),
        ("hidden_tests", &d.hidden_tests),
    ] {
        if tests.len() > MAX_DRAFT_TESTS {
            out.push(issue(field, format!("at most {MAX_DRAFT_TESTS} tests")));
        }
    }
    let too_long = [
        &d.statement,
        &d.statement_es,
        &d.editorial,
        &d.editorial_es,
        &d.reference,
    ]
    .into_iter()
    .chain(d.hints.iter())
    .chain(d.hints_es.iter())
    .any(|t| t.len() > MAX_DRAFT_TEXT);
    if too_long {
        out.push(issue("general", "a text is longer than 64 KB"));
    }
    out
}

/// The form field a validator message is about.
pub fn field_of(message: &str) -> &'static str {
    let m = message.to_ascii_lowercase();
    let has = |s: &str| m.contains(s);
    if has("title_es") {
        "title_es"
    } else if has("title") {
        "title"
    } else if has("tag") {
        "tags"
    } else if has("pattern") {
        "pattern"
    } else if has("rank") {
        "rank"
    } else if has("tests/public") || has("tests\\public") || has("public test") {
        "public_tests"
    } else if has("tests/hidden") || has("tests\\hidden") || has("hidden test") {
        "hidden_tests"
    } else if has("reference") || has("ref.") || has("stub") {
        "reference"
    } else if has("signature") || has("param") || has("returns") {
        "signature"
    } else if has("checker") {
        "checker"
    } else if has("limit") {
        "limits"
    } else if has("statement.es") {
        "statement_es"
    } else if has("statement") {
        "statement"
    } else if has("hints") {
        "hints"
    } else if has("editorial") {
        "editorial"
    } else {
        "general"
    }
}

#[derive(Serialize)]
struct TomlParam<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    ty: &'a str,
}

#[derive(Serialize)]
struct TomlSignature<'a> {
    name: &'a str,
    params: Vec<TomlParam<'a>>,
    returns: &'a str,
}

#[derive(Serialize)]
struct TomlJudge<'a> {
    checker: &'a str,
    time_limit_ms: u32,
    memory_mb: u32,
}

#[derive(Serialize)]
struct TomlProblem<'a> {
    id: &'a str,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    title_es: Option<&'a str>,
    difficulty: Difficulty,
    tags: &'a [String],
    pattern: &'a str,
    rank: u8,
    signature: TomlSignature<'a>,
    judge: TomlJudge<'a>,
}

fn join_hints(hints: &[String]) -> String {
    hints
        .iter()
        .map(|h| h.trim())
        .filter(|h| !h.is_empty())
        .collect::<Vec<_>>()
        .join("\n---\n")
        + "\n"
}

/// Write `d` as a package in `dir` (which must not exist yet; the caller owns it).
pub fn write_package(d: &ProblemDraft, dir: &Path) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let title_es = d.title_es.trim();
    let toml = toml::to_string(&TomlProblem {
        id: &d.id,
        title: d.title.trim(),
        title_es: (!title_es.is_empty()).then_some(title_es),
        difficulty: d.difficulty,
        tags: &d.tags,
        pattern: &d.pattern,
        rank: d.rank,
        signature: TomlSignature {
            name: &d.function,
            params: d
                .params
                .iter()
                .map(|p| TomlParam {
                    name: &p.name,
                    ty: &p.ty,
                })
                .collect(),
            returns: &d.returns,
        },
        judge: TomlJudge {
            checker: &d.checker,
            time_limit_ms: d.time_limit_ms,
            memory_mb: d.memory_mb,
        },
    })
    .map_err(|e| e.to_string())?;
    fs::create_dir_all(dir.join("solutions")).map_err(io)?;
    fs::create_dir_all(dir.join("tests/public")).map_err(io)?;
    fs::create_dir_all(dir.join("tests/hidden")).map_err(io)?;
    let write = |name: &str, text: &str| fs::write(dir.join(name), text).map_err(io);
    write("problem.toml", &toml)?;
    write("statement.md", &d.statement)?;
    write("statement.es.md", &d.statement_es)?;
    write("hints.md", &join_hints(&d.hints))?;
    if d.hints_es.iter().any(|h| !h.trim().is_empty()) {
        write("hints.es.md", &join_hints(&d.hints_es))?;
    }
    write("editorial.md", &d.editorial)?;
    if !d.editorial_es.trim().is_empty() {
        write("editorial.es.md", &d.editorial_es)?;
    }
    for (kind, tests) in [("public", &d.public_tests), ("hidden", &d.hidden_tests)] {
        for (i, t) in tests.iter().enumerate() {
            let json = serde_json::to_string(&serde_json::json!({
                "input": t.input,
                "output": t.output,
            }))
            .map_err(|e| e.to_string())?;
            write(&format!("tests/{kind}/{}.json", i + 1), &json)?;
        }
    }
    write(
        &format!("solutions/ref.{}", d.reference_language.extension()),
        &d.reference,
    )?;
    Ok(())
}

fn identifier(s: &str) -> bool {
    s.chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The starter code for the draft's signature in its reference language: the exact
/// declaration the reference must keep.
pub fn starter(d: &ProblemDraft) -> Result<String, String> {
    if !identifier(&d.function) || d.params.iter().any(|p| !identifier(&p.name)) {
        return Err("the function and parameter names must be identifiers".into());
    }
    let mut params = Vec::new();
    for p in &d.params {
        params.push(kb_judge::Param {
            name: p.name.clone(),
            ty: p
                .ty
                .parse()
                .map_err(|e| format!("param {:?}: {e}", p.name))?,
        });
    }
    let sig = kb_judge::Signature {
        name: d.function.clone(),
        params,
        returns: d.returns.parse().map_err(|e| format!("returns: {e}"))?,
    };
    kb_judge::starter_code(d.reference_language, &sig).map_err(|e| e.to_string())
}

/// Every issue: the draft's own checks, then (when it could be written) the package
/// validation with a pack's rules, each mapped to its field.
pub fn check_draft(d: &ProblemDraft, package: &Path) -> Vec<DraftIssue> {
    let mut out = draft_issues(d);
    if valid_draft_id(&d.id) {
        let (_, errs) = crate::validate_pack_problem_dir(package);
        for e in errs {
            // Already said per field above, in the form's words.
            if (e.contains("missing statement") || e.contains("missing editorial"))
                && out
                    .iter()
                    .any(|i| i.field.starts_with("statement") || i.field == "editorial")
            {
                continue;
            }
            out.push(issue(field_of(&e), e));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> std::path::PathBuf {
        let d =
            std::env::temp_dir().join(format!("kb-pack-authoring-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    /// A complete draft of an original problem: the largest value in a list.
    pub(crate) fn sample() -> ProblemDraft {
        let t = |nums: &[i64], out: i64| DraftTest {
            input: serde_json::json!({ "nums": nums })
                .as_object()
                .unwrap()
                .clone(),
            output: out.into(),
        };
        ProblemDraft {
            title: "Tallest Tower".into(),
            title_es: "La torre más alta".into(),
            tags: vec!["array".into()],
            function: "tallest".into(),
            statement: "Given tower heights `nums`, return the tallest.".into(),
            statement_es: "Dadas las alturas `nums`, devuelve la mayor.".into(),
            hints: vec![
                "Look at each tower once.".into(),
                String::new(),
                String::new(),
            ],
            editorial: "Keep the best value seen so far: O(n) time, O(1) space.".into(),
            public_tests: vec![t(&[1, 5, 3], 5), t(&[7], 7)],
            hidden_tests: vec![t(&[-4, -2, -9], -2), t(&[2, 2, 2], 2)],
            reference: "def tallest(nums: list[int]) -> int:\n    return max(nums)\n".into(),
            ..ProblemDraft::new("tallest-tower")
        }
    }

    #[test]
    fn a_complete_draft_writes_a_valid_pack_package() {
        let d = sample();
        let dir = temp("ok").join(&d.id);
        write_package(&d, &dir).unwrap();
        assert_eq!(check_draft(&d, &dir), vec![]);
        let toml = fs::read_to_string(dir.join("problem.toml")).unwrap();
        assert!(toml.contains("title_es = \"La torre más alta\""), "{toml}");
        assert!(dir.join("solutions/ref.py").is_file());
        assert!(!dir.join("solutions/ref.rs").exists());
        assert!(!dir.join("hints.es.md").exists());
        assert_eq!(
            fs::read_to_string(dir.join("hints.md")).unwrap(),
            "Look at each tower once.\n"
        );
        // Built-in content still needs every reference.
        let (_, builtin) = crate::validate_problem_dir(&dir);
        assert!(builtin.iter().any(|e| e.contains("ref.rs")), "{builtin:?}");
        // The package builds into a pack database with its one reference.
        let pack = crate::build::read_package_for_pack(&dir).unwrap();
        assert_eq!(pack.solutions.len(), 1);
    }

    #[test]
    fn issues_map_to_fields() {
        let mut d = sample();
        d.title = String::new();
        d.statement_es = String::new();
        d.tags.clear();
        d.params[0].ty = "integer[]".into();
        d.public_tests[0].output = "five".into();
        d.reference = "def wrong_name(nums):\n    return 0\n".into();
        let dir = temp("bad").join(&d.id);
        write_package(&d, &dir).unwrap();
        let issues = check_draft(&d, &dir);
        let fields: std::collections::BTreeSet<&str> =
            issues.iter().map(|i| i.field.as_str()).collect();
        for f in ["title", "statement_es", "tags", "signature"] {
            assert!(fields.contains(f), "{f}: {issues:#?}");
        }
        assert!(!fields.contains("general"), "{issues:#?}");
        // A test whose output doesn't fit the return type, with a fixed signature.
        let mut d = sample();
        d.public_tests[0].output = "five".into();
        d.reference = "def wrong_name(nums):\n    return 0\n".into();
        let dir = temp("bad2").join(&d.id);
        write_package(&d, &dir).unwrap();
        let fields: Vec<String> = check_draft(&d, &dir).into_iter().map(|i| i.field).collect();
        assert!(fields.contains(&"public_tests".to_string()), "{fields:?}");
    }

    #[test]
    fn ids_and_draft_checks() {
        for ok in ["a", "tallest-tower", "two-sum-2"] {
            assert!(valid_draft_id(ok), "{ok}");
        }
        for bad in ["", "A", "a--b", "-a", "a-", "a_b", "../x", &"a".repeat(41)] {
            assert!(!valid_draft_id(bad), "{bad}");
        }
        let d = ProblemDraft::new("x");
        let fields: Vec<String> = draft_issues(&d).into_iter().map(|i| i.field).collect();
        for f in [
            "statement",
            "statement_es",
            "editorial",
            "hints",
            "reference",
        ] {
            assert!(fields.contains(&f.to_string()), "{f}: {fields:?}");
        }
        assert_eq!(field_of("missing reference solution x"), "reference");
        assert_eq!(field_of("title_es is empty"), "title_es");
        assert_eq!(
            field_of("tests/hidden/3.json: output: expected int"),
            "hidden_tests"
        );
        assert_eq!(field_of("something else"), "general");
        assert!(patterns().contains(&"two-pointers".to_string()));
        let mut d = sample();
        d.reference_language = Language::TypeScript;
        let ts = starter(&d).unwrap();
        assert!(
            ts.contains("export function tallest(nums: number[]): number"),
            "{ts}"
        );
        d.function = "bad name".into();
        assert!(starter(&d).is_err());
        assert!(!patterns().iter().any(|p| p.starts_with("sql-")));
    }
}
