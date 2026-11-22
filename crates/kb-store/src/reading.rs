//! Reading exercises (expansion M7, ADR 0035): short Python and TypeScript snippets to read,
//! in sets. A "print" item asks what the code prints; a "complexity" item asks for its
//! time and space growth. kb-pack checks every answer in the sandbox before a build; the
//! app reveals answers only after the user has answered (kb-app checks them).

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{Row, SqliteConnection};

use crate::content::{ContentDb, UiLang};
use crate::{Result, StoreError};

/// The growth classes a complexity item may answer with (the ones the profiler's fit
/// tells apart, so kb-pack can check time answers).
pub const TIME_CLASSES: [&str; 6] = ["O(1)", "O(log n)", "O(n)", "O(n log n)", "O(n²)", "O(2ⁿ)"];
/// The space answers.
pub const SPACE_CLASSES: [&str; 4] = ["O(1)", "O(log n)", "O(n)", "O(n²)"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ReadingKind {
    /// What does this print?
    Print,
    /// What is its time and space complexity?
    Complexity,
}

impl ReadingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ReadingKind::Print => "print",
            ReadingKind::Complexity => "complexity",
        }
    }

    fn parse(s: &str) -> Result<Self> {
        match s {
            "print" => Ok(ReadingKind::Print),
            "complexity" => Ok(ReadingKind::Complexity),
            other => Err(StoreError::Corrupt(format!("reading kind {other:?}"))),
        }
    }
}

/// The 1-based line that dominates the cost, in each language's version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DominantLine {
    pub python: u32,
    pub typescript: u32,
}

/// One item as `items.json` holds it (and content.db stores it, as JSON).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingItemData {
    pub n: u32,
    pub prompt: String,
    pub prompt_es: String,
    pub explain: String,
    pub explain_es: String,
    /// Print items: the exact output in both languages...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// ...or per language, when they differ on purpose (then the answer is typed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_py: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_ts: Option<String>,
    /// Print items: 3-4 outputs to choose from (one is the answer); empty = typed answer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<String>,
    /// Complexity items: one of [`TIME_CLASSES`] and of [`SPACE_CLASSES`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dominant: Option<DominantLine>,
    /// Complexity items: the sizes kb-pack checks the time answer at (default by class).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sizes: Vec<u32>,
}

impl ReadingItemData {
    /// The expected output in `typescript` or Python.
    pub fn answer_for(&self, typescript: bool) -> Option<&str> {
        let own = if typescript {
            &self.answer_ts
        } else {
            &self.answer_py
        };
        own.as_deref().or(self.answer.as_deref())
    }
}

/// One item to write.
#[derive(Debug, Clone)]
pub struct PackReadingItem {
    pub code_py: String,
    pub code_ts: String,
    pub data: ReadingItemData,
}

/// One set to write.
#[derive(Debug, Clone)]
pub struct PackReadingSet {
    pub id: String,
    pub ord: u32,
    pub kind: ReadingKind,
    pub title_en: String,
    pub title_es: String,
    pub blurb_en: String,
    pub blurb_es: String,
    pub items: Vec<PackReadingItem>,
}

/// Print answers compare line by line, ignoring trailing spaces and trailing blank lines.
pub fn normalize_output(s: &str) -> String {
    let lines: Vec<&str> = s.lines().map(str::trim_end).collect();
    let end = lines
        .iter()
        .rposition(|l| !l.is_empty())
        .map_or(0, |i| i + 1);
    lines[..end].join("\n")
}

pub(crate) fn check_reading(
    sets: &[PackReadingSet],
    links: &[(String, String)],
    topics: &HashSet<&str>,
) -> Result<()> {
    let bad = |m: String| Err(StoreError::InvalidPack(m));
    let mut ids = HashSet::new();
    let mut items = HashSet::new();
    for s in sets {
        if !ids.insert(s.id.as_str()) {
            return bad(format!("reading set {} twice", s.id));
        }
        for (k, it) in s.items.iter().enumerate() {
            if it.data.n != k as u32 + 1 {
                return bad(format!(
                    "reading set {}: items must be numbered 1, 2, …",
                    s.id
                ));
            }
            items.insert(format!("{}/{}", s.id, it.data.n));
        }
    }
    for (topic, item) in links {
        if !topics.contains(topic.as_str()) {
            return bad(format!("reading link from unknown topic {topic}"));
        }
        if !items.contains(item) {
            return bad(format!("{topic}: unknown reading item {item}"));
        }
    }
    Ok(())
}

pub(crate) async fn insert_reading(
    conn: &mut SqliteConnection,
    sets: &[PackReadingSet],
    links: &[(String, String)],
) -> Result<()> {
    let mut sets: Vec<&PackReadingSet> = sets.iter().collect();
    sets.sort_by(|a, b| a.id.cmp(&b.id));
    for s in sets {
        sqlx::query(
            "INSERT INTO reading_sets (id, ord, kind, title_en, title_es, blurb_en, blurb_es)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&s.id)
        .bind(s.ord)
        .bind(s.kind.as_str())
        .bind(&s.title_en)
        .bind(&s.title_es)
        .bind(&s.blurb_en)
        .bind(&s.blurb_es)
        .execute(&mut *conn)
        .await?;
        for it in &s.items {
            sqlx::query(
                "INSERT INTO reading_items (set_id, n, code_py, code_ts, data)
                 VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&s.id)
            .bind(it.data.n)
            .bind(&it.code_py)
            .bind(&it.code_ts)
            .bind(serde_json::to_string(&it.data).map_err(|e| StoreError::Corrupt(e.to_string()))?)
            .execute(&mut *conn)
            .await?;
        }
    }
    for (k, (topic, item)) in links.iter().enumerate() {
        sqlx::query("INSERT INTO reading_links (topic_id, item_id, ord) VALUES (?, ?, ?)")
            .bind(topic)
            .bind(item)
            .bind(k as u32)
            .execute(&mut *conn)
            .await?;
    }
    Ok(())
}

/// A set in the list, in one UI language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingSetSummary {
    pub id: String,
    pub kind: ReadingKind,
    pub title: String,
    pub blurb: String,
    pub items: u32,
}

/// An item as the UI shows it before answering: never the answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingItemView {
    /// `<set>/<n>`.
    pub id: String,
    pub n: u32,
    pub prompt: String,
    pub code_python: String,
    pub code_typescript: String,
    /// Print items with choices; empty means the output is typed.
    pub choices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingSetView {
    pub id: String,
    pub kind: ReadingKind,
    pub title: String,
    pub blurb: String,
    pub items: Vec<ReadingItemView>,
}

/// A reading item linked from a Learn topic's Practice tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingLink {
    pub id: String,
    pub set: String,
    pub n: u32,
    pub kind: ReadingKind,
    pub set_title: String,
}

/// An item with its answer, for kb-app's answer check.
#[derive(Debug, Clone)]
pub struct ReadingItemFull {
    pub kind: ReadingKind,
    pub data: ReadingItemData,
}

fn pick(lang: UiLang, en: String, es: String) -> String {
    if lang == UiLang::Es && !es.is_empty() {
        es
    } else {
        en
    }
}

impl ContentDb {
    pub async fn reading_sets(&self, lang: UiLang) -> Result<Vec<ReadingSetSummary>> {
        let rows = sqlx::query(
            "SELECT s.id, s.kind, s.title_en, s.title_es, s.blurb_en, s.blurb_es,
                    (SELECT COUNT(*) FROM reading_items i WHERE i.set_id = s.id) AS items
             FROM reading_sets s ORDER BY s.ord",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|r| {
                Ok(ReadingSetSummary {
                    id: r.try_get("id")?,
                    kind: ReadingKind::parse(&r.try_get::<String, _>("kind")?)?,
                    title: pick(lang, r.try_get("title_en")?, r.try_get("title_es")?),
                    blurb: pick(lang, r.try_get("blurb_en")?, r.try_get("blurb_es")?),
                    items: r.try_get::<i64, _>("items")? as u32,
                })
            })
            .collect()
    }

    pub async fn reading_set(&self, id: &str, lang: UiLang) -> Result<ReadingSetView> {
        let s = sqlx::query(
            "SELECT id, kind, title_en, title_es, blurb_en, blurb_es FROM reading_sets WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| StoreError::NotFound(format!("reading set {id}")))?;
        let rows = sqlx::query(
            "SELECT n, code_py, code_ts, data FROM reading_items WHERE set_id = ? ORDER BY n",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;
        let mut items = Vec::new();
        for r in rows {
            let data: ReadingItemData = serde_json::from_str(&r.try_get::<String, _>("data")?)
                .map_err(|e| StoreError::Corrupt(e.to_string()))?;
            items.push(ReadingItemView {
                id: format!("{id}/{}", data.n),
                n: data.n,
                prompt: pick(lang, data.prompt.clone(), data.prompt_es.clone()),
                code_python: r.try_get("code_py")?,
                code_typescript: r.try_get("code_ts")?,
                choices: data.choices.clone(),
            });
        }
        Ok(ReadingSetView {
            id: s.try_get("id")?,
            kind: ReadingKind::parse(&s.try_get::<String, _>("kind")?)?,
            title: pick(lang, s.try_get("title_en")?, s.try_get("title_es")?),
            blurb: pick(lang, s.try_get("blurb_en")?, s.try_get("blurb_es")?),
            items,
        })
    }

    /// One item with its answer (`<set>/<n>`).
    pub async fn reading_item(&self, item: &str) -> Result<ReadingItemFull> {
        let (set, n) = item
            .split_once('/')
            .and_then(|(s, n)| Some((s, n.parse::<u32>().ok()?)))
            .ok_or_else(|| StoreError::NotFound(format!("reading item {item}")))?;
        let r = sqlx::query(
            "SELECT s.kind, i.data FROM reading_items i JOIN reading_sets s ON s.id = i.set_id
             WHERE i.set_id = ? AND i.n = ?",
        )
        .bind(set)
        .bind(n)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| StoreError::NotFound(format!("reading item {item}")))?;
        Ok(ReadingItemFull {
            kind: ReadingKind::parse(&r.try_get::<String, _>("kind")?)?,
            data: serde_json::from_str(&r.try_get::<String, _>("data")?)
                .map_err(|e| StoreError::Corrupt(e.to_string()))?,
        })
    }

    /// The reading items a Learn topic links to, in order.
    pub async fn reading_links(&self, topic: &str, lang: UiLang) -> Result<Vec<ReadingLink>> {
        let rows = sqlx::query(
            "SELECT l.item_id, s.id AS set_id, s.kind, s.title_en, s.title_es
             FROM reading_links l
             JOIN reading_sets s ON s.id = substr(l.item_id, 1, instr(l.item_id, '/') - 1)
             WHERE l.topic_id = ? ORDER BY l.ord",
        )
        .bind(topic)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|r| {
                let id: String = r.try_get("item_id")?;
                let n = id
                    .rsplit('/')
                    .next()
                    .and_then(|n| n.parse().ok())
                    .unwrap_or(0);
                Ok(ReadingLink {
                    n,
                    set: r.try_get("set_id")?,
                    kind: ReadingKind::parse(&r.try_get::<String, _>("kind")?)?,
                    set_title: pick(lang, r.try_get("title_en")?, r.try_get("title_es")?),
                    id,
                })
            })
            .collect()
    }
}
