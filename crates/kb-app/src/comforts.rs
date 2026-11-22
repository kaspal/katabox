//! Tool comforts (expansion M4, ADR 0034): the personal snippet library and the search
//! behind "Search everything" (problems, Learn topics and snippets; the UI adds its
//! settings and the user guide). Everything stays local.

use std::sync::atomic::{AtomicU64, Ordering};

use kb_core::Language;
use kb_store::snippets::SnippetRow;
use kb_store::{LearnTopicSummary, ProblemFilter, ProblemSummary, UiLang};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{App, AppError, AppErrorCode, Result};

pub const SNIPPET_NAME_MAX: usize = 80;
pub const SNIPPET_BODY_MAX: usize = 64 * 1024;
pub const SNIPPET_TAGS_MAX: usize = 10;
pub const SNIPPET_TAG_MAX: usize = 24;
/// Results per kind in "Search everything".
pub const SEARCH_LIMIT: usize = 10;

/// A snippet to save: a new one (`uid` none) or an edit.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SnippetInput {
    pub uid: Option<String>,
    pub language: Language,
    pub name: String,
    pub tags: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SearchResults {
    pub problems: Vec<ProblemSummary>,
    pub topics: Vec<LearnTopicSummary>,
    pub snippets: Vec<SnippetRow>,
}

fn bad(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, m.into())
}

pub(crate) fn valid_uid(uid: &str) -> bool {
    uid.len() == 16
        && uid
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The tags, trimmed and lower-cased, without duplicates; refused when invalid.
pub(crate) fn check_snippet(
    name: &str,
    tags: &[String],
    body: &str,
) -> Result<(String, Vec<String>)> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > SNIPPET_NAME_MAX {
        return Err(bad(format!(
            "a snippet name has 1 to {SNIPPET_NAME_MAX} characters"
        )));
    }
    if body.len() > SNIPPET_BODY_MAX {
        return Err(bad("a snippet is at most 64 KB"));
    }
    let mut out: Vec<String> = Vec::new();
    for t in tags {
        let t = t.trim().to_lowercase();
        if t.is_empty() {
            continue;
        }
        if t.chars().count() > SNIPPET_TAG_MAX
            || !t
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(bad(format!(
                "tag {t:?}: letters, digits, - and _ only, at most {SNIPPET_TAG_MAX}"
            )));
        }
        if !out.contains(&t) {
            out.push(t);
        }
    }
    if out.len() > SNIPPET_TAGS_MAX {
        return Err(bad(format!("at most {SNIPPET_TAGS_MAX} tags")));
    }
    Ok((name.to_string(), out))
}

static NEXT: AtomicU64 = AtomicU64::new(0);

impl App {
    pub async fn snippets(&self) -> Result<Vec<SnippetRow>> {
        Ok(self.store.user.snippets().await?)
    }

    /// Save a snippet (a new id for a new one) and return it.
    pub async fn save_snippet(&self, input: &SnippetInput) -> Result<SnippetRow> {
        if !kb_judge::languages::SUPPORTED.contains(&input.language) {
            return Err(bad("snippets are for the judge languages"));
        }
        let (name, tags) = check_snippet(&input.name, &input.tags, &input.body)?;
        let now = self.now_ms();
        let existing = self.store.user.snippets().await?;
        let (uid, created_at) = match &input.uid {
            Some(uid) => {
                let old = existing
                    .iter()
                    .find(|s| &s.uid == uid)
                    .ok_or_else(|| AppError::new(AppErrorCode::NotFound, "no such snippet"))?;
                (uid.clone(), old.created_at)
            }
            None => {
                let install = self.store.user.install_id().await?;
                let n = NEXT.fetch_add(1, Ordering::Relaxed);
                let d = Sha256::digest(format!("katabox-snippet|{install}|{now}|{n}").as_bytes());
                (d.iter().take(8).map(|b| format!("{b:02x}")).collect(), now)
            }
        };
        let row = SnippetRow {
            uid,
            language: input.language.id().into(),
            name,
            tags,
            body: input.body.clone(),
            created_at,
            updated_at: now,
        };
        self.store.user.put_snippet(&row).await?;
        Ok(self
            .store
            .user
            .snippets()
            .await?
            .into_iter()
            .find(|s| s.uid == row.uid)
            .unwrap_or(row))
    }

    pub async fn delete_snippet(&self, uid: &str) -> Result<()> {
        Ok(self.store.user.delete_snippet(uid).await?)
    }

    /// "Search everything": problems (full-text), Learn topics and snippets.
    pub async fn search_all(&self, query: &str, lang: UiLang) -> Result<SearchResults> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(SearchResults {
                problems: vec![],
                topics: vec![],
                snippets: vec![],
            });
        }
        let mut problems = self
            .store
            .list_problems(&ProblemFilter {
                lang: Some(lang),
                query: Some(q.to_string()),
                ..Default::default()
            })
            .await?;
        problems.truncate(SEARCH_LIMIT);
        let mut topics = self.store.content.search_learn(q, lang).await?;
        topics.truncate(SEARCH_LIMIT);
        let snippets = self
            .store
            .user
            .search_snippets(q, SEARCH_LIMIT as u32)
            .await?;
        Ok(SearchResults {
            problems,
            topics,
            snippets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snippet_checks() {
        let (name, tags) = check_snippet(
            " BFS ",
            &[" Graphs ".into(), "graphs".into(), "".into()],
            "x",
        )
        .unwrap();
        assert_eq!((name.as_str(), tags), ("BFS", vec!["graphs".to_string()]));
        assert!(check_snippet("", &[], "x").is_err());
        assert!(check_snippet(&"n".repeat(81), &[], "x").is_err());
        assert!(check_snippet("ok", &["bad tag".into()], "x").is_err());
        assert!(check_snippet("ok", &[], &"x".repeat(64 * 1024 + 1)).is_err());
        let many: Vec<String> = (0..11).map(|i| format!("t{i}")).collect();
        assert!(check_snippet("ok", &many, "x").is_err());
        assert!(valid_uid("0123456789abcdef"));
        assert!(!valid_uid("0123456789ABCDEF"));
        assert!(!valid_uid("../../etc/passwd"));
    }
}
