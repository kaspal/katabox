//! `content.db`: the compiled problem pack. Written once by [`build`] (`kb-pack build`),
//! opened read-only by the app.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use kb_core::{Difficulty, Language, LanguageLimit, TestCase, TestKind};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Connection, Row, SqliteConnection};

use crate::user::ProblemStatus;
use crate::{Result, StoreError};

/// Bump when `sql/content.sql` changes; the app refuses other versions.
pub const CONTENT_SCHEMA_VERSION: i64 = 2;
const SCHEMA: &str = include_str!("../sql/content.sql");

/// A UI language. English is always present in a pack; others fall back to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum UiLang {
    En,
    Es,
}

impl UiLang {
    pub const ALL: [UiLang; 2] = [UiLang::En, UiLang::Es];

    pub fn code(self) -> &'static str {
        match self {
            UiLang::En => "en",
            UiLang::Es => "es",
        }
    }
}

impl FromStr for UiLang {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        UiLang::ALL
            .into_iter()
            .find(|l| l.code() == s)
            .ok_or_else(|| format!("unknown UI language {s:?}"))
    }
}

/// Identity of the pack being built.
#[derive(Debug, Clone)]
pub struct PackMeta {
    pub pack_id: String,
    pub pack_version: String,
}

/// One problem package, already read and validated (kb-pack does that).
#[derive(Debug, Clone)]
pub struct PackProblem {
    pub id: String,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
    pub pattern: String,
    pub rank: u8,
    pub profile: bool,
    /// `{"name": ..., "params": [{"name": ..., "type": ...}], "returns": ...}`.
    pub signature: Value,
    pub checker: String,
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    /// One per UI language; English is required.
    pub texts: Vec<PackText>,
    /// Public and hidden tests (`kind` and 1-based `index` as loaded by kb-judge).
    pub tests: Vec<TestCase>,
    pub solutions: Vec<(Language, String)>,
    /// `validate.py`, when the problem has one (ADR 0016).
    pub validator: Option<String>,
}

/// The texts of a problem in one UI language. A missing editorial or empty hint list
/// falls back to English at read time.
#[derive(Debug, Clone)]
pub struct PackText {
    pub lang: UiLang,
    pub title: String,
    pub statement: String,
    pub editorial: Option<String>,
    pub hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ContentMeta {
    pub pack_id: String,
    pub pack_version: String,
    /// UTC, ISO 8601 (`2026-11-11T09:30:00Z`).
    pub built_at: String,
}

/// Which problems to list. Every field is optional; `None` means "any".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProblemFilter {
    pub difficulty: Option<Difficulty>,
    pub tag: Option<String>,
    pub pattern: Option<String>,
    /// Applied by [`crate::Store::list_problems`] (status lives in user.db);
    /// [`ContentDb::list_problems`] ignores it.
    pub status: Option<StatusFilter>,
    /// Free text, matched against title, tags and statement in every language.
    pub query: Option<String>,
    /// Language of the returned titles (English fallback).
    pub lang: Option<UiLang>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum StatusFilter {
    /// Never submitted.
    Todo,
    Attempted,
    Solved,
}

impl StatusFilter {
    pub fn matches(self, status: Option<ProblemStatus>) -> bool {
        match self {
            StatusFilter::Todo => status.is_none(),
            StatusFilter::Attempted => status == Some(ProblemStatus::Attempted),
            StatusFilter::Solved => status == Some(ProblemStatus::Solved),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProblemSummary {
    pub id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub pattern: String,
    pub rank: u8,
    pub tags: Vec<String>,
    /// Filled by [`crate::Store::list_problems`]; `None` from [`ContentDb`] alone.
    pub status: Option<ProblemStatus>,
}

/// A public test as shown in the UI.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PublicTest {
    pub index: u32,
    pub input: Map<String, Value>,
    pub output: Value,
}

/// Everything the problem page shows, in one UI language with English fallback.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProblemView {
    pub id: String,
    /// The language requested; each text falls back to English when missing.
    pub lang: UiLang,
    pub title: String,
    pub statement: String,
    pub editorial: Option<String>,
    pub hints: Vec<String>,
    pub difficulty: Difficulty,
    pub pattern: String,
    pub rank: u8,
    pub tags: Vec<String>,
    pub signature: Value,
    /// The base time limit, set for Rust; each language scales it (ADR 0017).
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    pub public_tests: Vec<PublicTest>,
    /// Custom test inputs are checked by the problem's `validate.py` (needs Python).
    pub has_validator: bool,
    /// Each language's time limit at this machine's time scale. Filled by the app, which
    /// knows the languages and the calibration; empty as read from content.db.
    pub time_limits: Vec<LanguageLimit>,
    /// The time scale in `time_limits` (1.0 = the reference machine; ADR 0017).
    pub time_scale: f64,
}

/// What the judge needs: settings and every test (public first, then hidden).
#[derive(Debug, Clone, PartialEq)]
pub struct JudgeData {
    pub id: String,
    pub signature: Value,
    pub checker: String,
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    pub tests: Vec<TestCase>,
    /// `validate.py`, checking custom inputs (ADR 0016).
    pub validator: Option<String>,
}

/// Read-only handle to content.db.
#[derive(Debug, Clone)]
pub struct ContentDb {
    pool: SqlitePool,
}

impl ContentDb {
    /// Open read-only; refuses a file with another content schema version.
    pub async fn open(path: &Path) -> Result<ContentDb> {
        if !path.is_file() {
            return Err(StoreError::NotFound(format!("{}", path.display())));
        }
        let opts = SqliteConnectOptions::new()
            .filename(path)
            .read_only(true)
            .pragma("query_only", "ON");
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(opts)
            .await?;
        let found: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&pool)
            .await?;
        if found != CONTENT_SCHEMA_VERSION {
            pool.close().await;
            return Err(StoreError::ContentSchema {
                found,
                expected: CONTENT_SCHEMA_VERSION,
            });
        }
        Ok(ContentDb { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn meta(&self) -> Result<ContentMeta> {
        let rows = sqlx::query("SELECT key, value FROM content_meta")
            .fetch_all(&self.pool)
            .await?;
        let get = |k: &str| -> Result<String> {
            rows.iter()
                .find(|r| r.get::<String, _>("key") == k)
                .map(|r| r.get("value"))
                .ok_or_else(|| StoreError::Corrupt(format!("content_meta lacks {k}")))
        };
        Ok(ContentMeta {
            pack_id: get("pack_id")?,
            pack_version: get("pack_version")?,
            built_at: get("built_at")?,
        })
    }

    /// Problems matching `filter` (status ignored, see [`ProblemFilter::status`]),
    /// ordered by rank, then id.
    pub async fn list_problems(&self, filter: &ProblemFilter) -> Result<Vec<ProblemSummary>> {
        let lang = filter.lang.unwrap_or(UiLang::En);
        // Text with no words (only punctuation or spaces) does not filter.
        let fts = filter.query.as_deref().and_then(fts_query);
        let rows = sqlx::query(
            "SELECT p.id, p.difficulty, p.pattern, p.rank,
                    COALESCE(tl.title, te.title) AS title,
                    (SELECT json_group_array(tag) FROM
                        (SELECT tag FROM problem_tags t WHERE t.problem_id = p.id ORDER BY tag)
                    ) AS tags
             FROM problems p
             JOIN problem_texts te ON te.problem_id = p.id AND te.lang = 'en'
             LEFT JOIN problem_texts tl ON tl.problem_id = p.id AND tl.lang = ?1
             WHERE (?2 IS NULL OR p.difficulty = ?2)
               AND (?3 IS NULL OR p.pattern = ?3)
               AND (?4 IS NULL OR EXISTS
                    (SELECT 1 FROM problem_tags t WHERE t.problem_id = p.id AND t.tag = ?4))
               AND (?5 IS NULL OR p.id IN
                    (SELECT problem_id FROM problem_fts WHERE problem_fts MATCH ?5))
             ORDER BY p.rank, p.id",
        )
        .bind(lang.code())
        .bind(filter.difficulty.map(difficulty_code))
        .bind(filter.pattern.as_deref())
        .bind(filter.tag.as_deref())
        .bind(fts)
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|r| {
                Ok(ProblemSummary {
                    id: r.try_get("id")?,
                    title: r.try_get("title")?,
                    difficulty: parse_difficulty(&r.try_get::<String, _>("difficulty")?)?,
                    pattern: r.try_get("pattern")?,
                    rank: rank(r)?,
                    tags: serde_json::from_str(&r.try_get::<String, _>("tags")?)?,
                    status: None,
                })
            })
            .collect()
    }

    /// The problem page in `lang`, each text falling back to English.
    pub async fn get_problem(&self, id: &str, lang: UiLang) -> Result<ProblemView> {
        let p = sqlx::query("SELECT * FROM problems WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| StoreError::NotFound(format!("problem {id}")))?;
        let texts = sqlx::query(
            "SELECT lang, title, statement, editorial FROM problem_texts
             WHERE problem_id = ? AND lang IN (?, 'en')",
        )
        .bind(id)
        .bind(lang.code())
        .fetch_all(&self.pool)
        .await?;
        let pick = |want: &str| texts.iter().find(|t| t.get::<String, _>("lang") == want);
        let en = pick("en").ok_or_else(|| StoreError::Corrupt(format!("{id} has no en text")))?;
        let local = pick(lang.code());
        let text = |col: &str| -> Option<String> {
            local
                .and_then(|t| t.get::<Option<String>, _>(col))
                .or_else(|| en.get::<Option<String>, _>(col))
        };

        let mut hints = self.hints(id, lang).await?;
        if hints.is_empty() && lang != UiLang::En {
            hints = self.hints(id, UiLang::En).await?;
        }
        let tags: Vec<String> =
            sqlx::query_scalar("SELECT tag FROM problem_tags WHERE problem_id = ? ORDER BY tag")
                .bind(id)
                .fetch_all(&self.pool)
                .await?;
        let public_tests = sqlx::query(
            "SELECT idx, input, output FROM tests
             WHERE problem_id = ? AND kind = 'public' ORDER BY idx",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| {
            Ok(PublicTest {
                index: r.try_get("idx")?,
                input: serde_json::from_str(&r.try_get::<String, _>("input")?)?,
                output: serde_json::from_str(&r.try_get::<String, _>("output")?)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

        Ok(ProblemView {
            id: id.to_string(),
            lang,
            title: text("title").unwrap_or_default(),
            statement: text("statement").unwrap_or_default(),
            editorial: text("editorial"),
            hints,
            difficulty: parse_difficulty(&p.try_get::<String, _>("difficulty")?)?,
            pattern: p.try_get("pattern")?,
            rank: rank(&p)?,
            tags,
            signature: serde_json::from_str(&p.try_get::<String, _>("signature")?)?,
            time_limit_ms: p.try_get("time_limit_ms")?,
            memory_mb: p.try_get("memory_mb")?,
            public_tests,
            has_validator: p.try_get::<Option<String>, _>("validator")?.is_some(),
            time_limits: Vec::new(),
            time_scale: 1.0,
        })
    }

    async fn hints(&self, id: &str, lang: UiLang) -> Result<Vec<String>> {
        Ok(sqlx::query_scalar(
            "SELECT body FROM hints WHERE problem_id = ? AND lang = ? ORDER BY tier",
        )
        .bind(id)
        .bind(lang.code())
        .fetch_all(&self.pool)
        .await?)
    }

    /// The problem's `validate.py`, if it has one (custom tests; ADR 0016).
    pub async fn validator(&self, id: &str) -> Result<Option<String>> {
        let v: Option<Option<String>> =
            sqlx::query_scalar("SELECT validator FROM problems WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        v.ok_or_else(|| StoreError::NotFound(format!("problem {id}")))
    }

    /// Settings and all tests for judging `id` (public first, then hidden).
    pub async fn judge_data(&self, id: &str) -> Result<JudgeData> {
        let p = sqlx::query(
            "SELECT signature, checker, time_limit_ms, memory_mb, validator FROM problems WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| StoreError::NotFound(format!("problem {id}")))?;
        let tests = sqlx::query(
            "SELECT kind, idx, input, output FROM tests WHERE problem_id = ?
             ORDER BY CASE kind WHEN 'public' THEN 0 ELSE 1 END, idx",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| {
            Ok(TestCase {
                kind: parse_kind(&r.try_get::<String, _>("kind")?)?,
                index: r.try_get("idx")?,
                input: serde_json::from_str(&r.try_get::<String, _>("input")?)?,
                expected: serde_json::from_str(&r.try_get::<String, _>("output")?)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
        Ok(JudgeData {
            id: id.to_string(),
            signature: serde_json::from_str(&p.try_get::<String, _>("signature")?)?,
            checker: p.try_get("checker")?,
            time_limit_ms: p.try_get("time_limit_ms")?,
            memory_mb: p.try_get("memory_mb")?,
            tests,
            validator: p.try_get("validator")?,
        })
    }

    pub async fn reference_solution(&self, id: &str, language: Language) -> Result<Option<String>> {
        Ok(sqlx::query_scalar(
            "SELECT code FROM reference_solutions WHERE problem_id = ? AND language = ?",
        )
        .bind(id)
        .bind(language.id())
        .fetch_optional(&self.pool)
        .await?)
    }
}

/// Write `problems` into a new content.db at `out`. Builds into `<out>.tmp` and renames
/// it over `out` only when everything succeeded. Rows are inserted in a fixed order, so
/// two builds of the same input differ only in `content_meta.built_at`.
pub async fn build(out: &Path, meta: &PackMeta, problems: &[PackProblem]) -> Result<()> {
    check_pack(problems)?;
    if let Some(dir) = out.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = PathBuf::from(format!("{}.tmp", out.display()));
    remove_if_exists(&tmp)?;
    let result = write_pack(&tmp, meta, problems).await;
    match result {
        Ok(()) => {
            std::fs::rename(&tmp, out)?;
            Ok(())
        }
        Err(e) => {
            let _ = remove_if_exists(&tmp);
            Err(e)
        }
    }
}

fn check_pack(problems: &[PackProblem]) -> Result<()> {
    let mut seen = HashSet::new();
    for p in problems {
        if !seen.insert(p.id.as_str()) {
            return Err(StoreError::InvalidPack(format!(
                "duplicate problem id {}",
                p.id
            )));
        }
        if !p.texts.iter().any(|t| t.lang == UiLang::En) {
            return Err(StoreError::InvalidPack(format!(
                "{}: no English text",
                p.id
            )));
        }
        let mut langs = HashSet::new();
        if !p.texts.iter().all(|t| langs.insert(t.lang)) {
            return Err(StoreError::InvalidPack(format!(
                "{}: a language twice",
                p.id
            )));
        }
        if p.tests
            .iter()
            .any(|t| !matches!(t.kind, TestKind::Public | TestKind::Hidden))
        {
            return Err(StoreError::InvalidPack(format!(
                "{}: only public and hidden tests belong in a pack",
                p.id
            )));
        }
    }
    Ok(())
}

async fn write_pack(path: &Path, meta: &PackMeta, problems: &[PackProblem]) -> Result<()> {
    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let mut conn = SqliteConnection::connect_with(&opts).await?;
    sqlx::raw_sql(SCHEMA).execute(&mut conn).await?;
    sqlx::raw_sql(&format!("PRAGMA user_version = {CONTENT_SCHEMA_VERSION}"))
        .execute(&mut conn)
        .await?;

    let mut tx = conn.begin().await?;
    let built_at: String = sqlx::query_scalar("SELECT strftime('%Y-%m-%dT%H:%M:%SZ', 'now')")
        .fetch_one(&mut *tx)
        .await?;
    for (k, v) in [
        ("pack_id", meta.pack_id.as_str()),
        ("pack_version", meta.pack_version.as_str()),
        ("schema_version", &CONTENT_SCHEMA_VERSION.to_string()),
        ("built_at", &built_at),
    ] {
        sqlx::query("INSERT INTO content_meta (key, value) VALUES (?, ?)")
            .bind(k)
            .bind(v)
            .execute(&mut *tx)
            .await?;
    }

    let mut sorted: Vec<&PackProblem> = problems.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for p in sorted {
        insert_problem(&mut tx, p).await?;
    }
    tx.commit().await?;
    sqlx::raw_sql("VACUUM").execute(&mut conn).await?;
    conn.close().await?;
    Ok(())
}

async fn insert_problem(conn: &mut SqliteConnection, p: &PackProblem) -> Result<()> {
    sqlx::query(
        "INSERT INTO problems (id, difficulty, pattern, rank, signature, checker,
                               time_limit_ms, memory_mb, profile, validator)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&p.id)
    .bind(difficulty_code(p.difficulty))
    .bind(&p.pattern)
    .bind(i64::from(p.rank))
    .bind(serde_json::to_string(&p.signature)?)
    .bind(&p.checker)
    .bind(p.time_limit_ms)
    .bind(p.memory_mb)
    .bind(p.profile)
    .bind(&p.validator)
    .execute(&mut *conn)
    .await?;

    let mut tags = p.tags.clone();
    tags.sort();
    tags.dedup();
    for t in &tags {
        sqlx::query("INSERT INTO problem_tags (problem_id, tag) VALUES (?, ?)")
            .bind(&p.id)
            .bind(t)
            .execute(&mut *conn)
            .await?;
    }

    let mut texts: Vec<&PackText> = p.texts.iter().collect();
    texts.sort_by_key(|t| t.lang.code());
    for t in texts {
        sqlx::query(
            "INSERT INTO problem_texts (problem_id, lang, title, statement, editorial)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&p.id)
        .bind(t.lang.code())
        .bind(&t.title)
        .bind(&t.statement)
        .bind(t.editorial.as_deref())
        .execute(&mut *conn)
        .await?;
        for (i, h) in t.hints.iter().enumerate() {
            sqlx::query("INSERT INTO hints (problem_id, lang, tier, body) VALUES (?, ?, ?, ?)")
                .bind(&p.id)
                .bind(t.lang.code())
                .bind(i64::try_from(i + 1).unwrap_or(i64::MAX))
                .bind(h)
                .execute(&mut *conn)
                .await?;
        }
        sqlx::query(
            "INSERT INTO problem_fts (problem_id, lang, title, tags, statement)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&p.id)
        .bind(t.lang.code())
        .bind(&t.title)
        .bind(tags.join(" "))
        .bind(&t.statement)
        .execute(&mut *conn)
        .await?;
    }

    let mut tests: Vec<&TestCase> = p.tests.iter().collect();
    tests.sort_by_key(|t| (kind_code(t.kind), t.index));
    for t in tests {
        sqlx::query(
            "INSERT INTO tests (problem_id, kind, idx, input, output) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&p.id)
        .bind(kind_code(t.kind))
        .bind(t.index)
        .bind(serde_json::to_string(&t.input)?)
        .bind(serde_json::to_string(&t.expected)?)
        .execute(&mut *conn)
        .await?;
    }

    let mut solutions: Vec<&(Language, String)> = p.solutions.iter().collect();
    solutions.sort_by_key(|(l, _)| l.id());
    for (lang, code) in solutions {
        sqlx::query(
            "INSERT INTO reference_solutions (problem_id, language, code) VALUES (?, ?, ?)",
        )
        .bind(&p.id)
        .bind(lang.id())
        .bind(code)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

/// Turn free text into a safe FTS5 query: every word quoted (so FTS syntax in the input
/// is literal) and prefix-matched, all words required. `None` when there are no words.
pub(crate) fn fts_query(text: &str) -> Option<String> {
    let terms: Vec<String> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| format!("\"{w}\"*"))
        .collect();
    (!terms.is_empty()).then(|| terms.join(" "))
}

fn remove_if_exists(p: &Path) -> std::io::Result<()> {
    match std::fs::remove_file(p) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        r => r,
    }
}

fn rank(r: &SqliteRow) -> Result<u8> {
    let v: i64 = r.try_get("rank")?;
    u8::try_from(v).map_err(|_| StoreError::Corrupt(format!("rank {v}")))
}

pub(crate) fn difficulty_code(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "easy",
        Difficulty::Medium => "medium",
        Difficulty::Hard => "hard",
    }
}

fn parse_difficulty(s: &str) -> Result<Difficulty> {
    match s {
        "easy" => Ok(Difficulty::Easy),
        "medium" => Ok(Difficulty::Medium),
        "hard" => Ok(Difficulty::Hard),
        _ => Err(StoreError::Corrupt(format!("difficulty {s:?}"))),
    }
}

fn kind_code(k: TestKind) -> &'static str {
    match k {
        TestKind::Public => "public",
        TestKind::Hidden => "hidden",
        TestKind::Example => "example",
        TestKind::Custom => "custom",
    }
}

fn parse_kind(s: &str) -> Result<TestKind> {
    match s {
        "public" => Ok(TestKind::Public),
        "hidden" => Ok(TestKind::Hidden),
        _ => Err(StoreError::Corrupt(format!("test kind {s:?}"))),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::testutil::temp_dir;
    use serde_json::json;

    pub fn sample_problem(
        id: &str,
        difficulty: Difficulty,
        rank: u8,
        spanish: bool,
    ) -> PackProblem {
        let mut texts = vec![PackText {
            lang: UiLang::En,
            title: format!("Title of {id}"),
            statement: format!("Find the widest gap between numbers in {id}."),
            editorial: Some("Sort, then scan.".into()),
            hints: vec!["Sort first.".into(), "Compare neighbours.".into()],
        }];
        if spanish {
            texts.push(PackText {
                lang: UiLang::Es,
                title: format!("Título de {id}"),
                statement: format!("Encuentra la brecha más ancha entre números en {id}."),
                editorial: None,
                hints: vec![],
            });
        }
        let test = |kind, index, n: i64| TestCase {
            kind,
            index,
            input: json!({"nums": [n, n + 3]}).as_object().unwrap().clone(),
            expected: json!(3),
        };
        PackProblem {
            id: id.into(),
            difficulty,
            tags: vec!["sorting".into(), "array".into()],
            pattern: "sorting".into(),
            rank,
            profile: false,
            validator: (id == "gap-a").then(|| {
                "def validate(inp):
    return None
"
                .to_string()
            }),
            signature: json!({"name": "widestGap", "params": [{"name": "nums", "type": "int[]"}], "returns": "int"}),
            checker: "exact".into(),
            time_limit_ms: 2000,
            memory_mb: 256,
            texts,
            tests: vec![
                test(TestKind::Hidden, 1, 10),
                test(TestKind::Public, 2, 2),
                test(TestKind::Public, 1, 1),
            ],
            solutions: vec![(Language::Python, "def widest_gap(nums): ...\n".into())],
        }
    }

    pub async fn sample_db(name: &str) -> (PathBuf, ContentDb) {
        let dir = temp_dir(name);
        let path = dir.join("content.db");
        let meta = PackMeta {
            pack_id: "test".into(),
            pack_version: "1.0.0".into(),
        };
        let problems = [
            sample_problem("gap-b", Difficulty::Medium, 2, false),
            sample_problem("gap-a", Difficulty::Easy, 1, true),
        ];
        build(&path, &meta, &problems).await.unwrap();
        (dir, ContentDb::open(&path).await.unwrap())
    }

    #[tokio::test]
    async fn bundled_sqlite_has_fts5() {
        let mut conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();
        let on: i64 = sqlx::query_scalar("SELECT sqlite_compileoption_used('ENABLE_FTS5')")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(on, 1);
    }

    #[test]
    fn fts_query_quotes_every_word() {
        assert_eq!(fts_query("two sum").as_deref(), Some("\"two\"* \"sum\"*"));
        assert_eq!(
            fts_query("a\" OR b*").as_deref(),
            Some("\"a\"* \"OR\"* \"b\"*")
        );
        assert_eq!(fts_query("índices").as_deref(), Some("\"índices\"*"));
        assert_eq!(fts_query(" -- ()"), None);
    }

    #[tokio::test]
    async fn builds_and_reads_back() {
        let (_dir, db) = sample_db("build").await;
        let meta = db.meta().await.unwrap();
        assert_eq!(meta.pack_id, "test");
        assert_eq!(meta.pack_version, "1.0.0");
        assert!(meta.built_at.ends_with('Z'), "{}", meta.built_at);

        let all = db.list_problems(&ProblemFilter::default()).await.unwrap();
        let ids: Vec<&str> = all.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, ["gap-a", "gap-b"]); // by rank
        assert_eq!(all[0].tags, ["array", "sorting"]);
        assert_eq!(all[0].title, "Title of gap-a");

        let jd = db.judge_data("gap-a").await.unwrap();
        let order: Vec<(TestKind, u32)> = jd.tests.iter().map(|t| (t.kind, t.index)).collect();
        assert_eq!(
            order,
            [
                (TestKind::Public, 1),
                (TestKind::Public, 2),
                (TestKind::Hidden, 1)
            ]
        );
        assert_eq!(jd.checker, "exact");
        // validate.py travels with the problem (ADR 0016); gap-b has none.
        assert_eq!(
            jd.validator.as_deref(),
            Some(
                "def validate(inp):
    return None
"
            )
        );
        assert_eq!(db.judge_data("gap-b").await.unwrap().validator, None);
        assert_eq!(
            db.reference_solution("gap-a", Language::Python)
                .await
                .unwrap()
                .as_deref(),
            Some("def widest_gap(nums): ...\n")
        );
        assert_eq!(
            db.reference_solution("gap-a", Language::Rust)
                .await
                .unwrap(),
            None
        );
        assert!(matches!(
            db.get_problem("nope", UiLang::En).await,
            Err(StoreError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn spanish_with_english_fallback() {
        let (_dir, db) = sample_db("i18n").await;
        let es = db.get_problem("gap-a", UiLang::Es).await.unwrap();
        assert_eq!(es.title, "Título de gap-a");
        assert!(es.statement.starts_with("Encuentra"));
        assert_eq!(es.editorial.as_deref(), Some("Sort, then scan.")); // fallback
        assert_eq!(es.hints, ["Sort first.", "Compare neighbours."]); // fallback
        assert_eq!(es.public_tests.len(), 2);
        assert_eq!(es.public_tests[0].index, 1);

        // No Spanish text at all: everything in English.
        assert!(es.has_validator);
        let b = db.get_problem("gap-b", UiLang::Es).await.unwrap();
        assert!(!b.has_validator);
        assert_eq!(b.title, "Title of gap-b");
        let list = db
            .list_problems(&ProblemFilter {
                lang: Some(UiLang::Es),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(list[0].title, "Título de gap-a");
        assert_eq!(list[1].title, "Title of gap-b");
    }

    #[tokio::test]
    async fn filters_and_search() {
        let (_dir, db) = sample_db("filters").await;
        let ids = |f: ProblemFilter| {
            let db = db.clone();
            async move {
                db.list_problems(&f)
                    .await
                    .unwrap()
                    .into_iter()
                    .map(|p| p.id)
                    .collect::<Vec<_>>()
            }
        };
        let f = ProblemFilter::default;
        assert_eq!(
            ids(ProblemFilter {
                difficulty: Some(Difficulty::Medium),
                ..f()
            })
            .await,
            ["gap-b"]
        );
        assert_eq!(
            ids(ProblemFilter {
                tag: Some("array".into()),
                ..f()
            })
            .await
            .len(),
            2
        );
        assert!(
            ids(ProblemFilter {
                tag: Some("graph".into()),
                ..f()
            })
            .await
            .is_empty()
        );
        assert_eq!(
            ids(ProblemFilter {
                pattern: Some("sorting".into()),
                ..f()
            })
            .await
            .len(),
            2
        );
        // English words, prefixes, both problems.
        assert_eq!(
            ids(ProblemFilter {
                query: Some("wid gap".into()),
                ..f()
            })
            .await
            .len(),
            2
        );
        // Spanish, with and without accents: only gap-a has Spanish text.
        assert_eq!(
            ids(ProblemFilter {
                query: Some("brecha más".into()),
                ..f()
            })
            .await,
            ["gap-a"]
        );
        assert_eq!(
            ids(ProblemFilter {
                query: Some("numeros".into()),
                ..f()
            })
            .await,
            ["gap-a"]
        );
        // FTS syntax in user input is literal, not an error.
        assert!(
            ids(ProblemFilter {
                query: Some("\"NEAR(".into()),
                ..f()
            })
            .await
            .is_empty()
        );
        // Tag search through FTS.
        assert_eq!(
            ids(ProblemFilter {
                query: Some("sorting".into()),
                ..f()
            })
            .await
            .len(),
            2
        );
    }

    #[tokio::test]
    async fn refuses_other_schema_and_bad_packs() {
        let dir = temp_dir("schema");
        let path = dir.join("content.db");
        let mut conn = SqliteConnection::connect_with(
            &SqliteConnectOptions::new()
                .filename(&path)
                .create_if_missing(true),
        )
        .await
        .unwrap();
        sqlx::raw_sql("PRAGMA user_version = 99")
            .execute(&mut conn)
            .await
            .unwrap();
        conn.close().await.unwrap();
        assert!(matches!(
            ContentDb::open(&path).await,
            Err(StoreError::ContentSchema { found: 99, .. })
        ));

        let meta = PackMeta {
            pack_id: "t".into(),
            pack_version: "1".into(),
        };
        let out = dir.join("bad.db");
        let dup = [
            sample_problem("x", Difficulty::Easy, 1, false),
            sample_problem("x", Difficulty::Easy, 1, false),
        ];
        assert!(matches!(
            build(&out, &meta, &dup).await,
            Err(StoreError::InvalidPack(_))
        ));
        let mut no_en = sample_problem("y", Difficulty::Easy, 1, true);
        no_en.texts.retain(|t| t.lang != UiLang::En);
        assert!(build(&out, &meta, &[no_en]).await.is_err());
        assert!(!out.exists());
        assert!(!dir.join("bad.db.tmp").exists());
    }
}
