//! user.db queries for the personal snippet library (expansion M4, ADR 0034). The
//! validation rules live in kb-app.

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, StoreError, UserDb, now_ms};

/// One snippet (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SnippetRow {
    /// 16 hex characters, random.
    pub uid: String,
    /// The language id (`python`, `typescript`, …).
    pub language: String,
    pub name: String,
    pub tags: Vec<String>,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64,
}

fn snippet_row(r: &SqliteRow) -> Result<SnippetRow> {
    Ok(SnippetRow {
        uid: r.try_get("uid")?,
        language: r.try_get("language")?,
        name: r.try_get("name")?,
        tags: serde_json::from_str(&r.try_get::<String, _>("tags")?)?,
        body: r.try_get("body")?,
        created_at: r.try_get("created_at")?,
        updated_at: r.try_get("updated_at")?,
    })
}

impl UserDb {
    /// Every snippet, by language then name.
    pub async fn snippets(&self) -> Result<Vec<SnippetRow>> {
        sqlx::query("SELECT * FROM snippets ORDER BY language, name COLLATE NOCASE, uid")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(snippet_row)
            .collect()
    }

    /// Insert or replace a snippet by its uid (the caller checks the values).
    pub async fn put_snippet(&self, s: &SnippetRow) -> Result<()> {
        sqlx::query(
            "INSERT INTO snippets (uid, language, name, tags, body, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT (uid) DO UPDATE SET language = excluded.language, name = excluded.name,
               tags = excluded.tags, body = excluded.body, updated_at = excluded.updated_at",
        )
        .bind(&s.uid)
        .bind(&s.language)
        .bind(&s.name)
        .bind(serde_json::to_string(&s.tags)?)
        .bind(&s.body)
        .bind(s.created_at)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_snippet(&self, uid: &str) -> Result<()> {
        let n = sqlx::query("DELETE FROM snippets WHERE uid = ?")
            .bind(uid)
            .execute(&self.pool)
            .await?
            .rows_affected();
        if n == 0 {
            return Err(StoreError::NotFound(format!("snippet {uid}")));
        }
        Ok(())
    }

    /// Snippets whose name, tags or body contain `query` (any case), at most `limit`.
    pub async fn search_snippets(&self, query: &str, limit: u32) -> Result<Vec<SnippetRow>> {
        let like = format!(
            "%{}%",
            query
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        );
        sqlx::query(
            "SELECT * FROM snippets WHERE name LIKE ?1 ESCAPE '\\' OR tags LIKE ?1 ESCAPE '\\'
               OR body LIKE ?1 ESCAPE '\\'
             ORDER BY (name LIKE ?1 ESCAPE '\\') DESC, name COLLATE NOCASE LIMIT ?2",
        )
        .bind(like)
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(snippet_row)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;

    fn snip(uid: &str, name: &str, body: &str) -> SnippetRow {
        SnippetRow {
            uid: uid.into(),
            language: "python".into(),
            name: name.into(),
            tags: vec!["graphs".into()],
            body: body.into(),
            created_at: 1,
            updated_at: 1,
        }
    }

    #[tokio::test]
    async fn snippets_crud_and_search() {
        let dir = temp_dir("snippets");
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        db.put_snippet(&snip("a1", "BFS queue", "from collections import deque"))
            .await
            .unwrap();
        db.put_snippet(&snip("b2", "binary search", "lo, hi = 0, n"))
            .await
            .unwrap();
        db.put_snippet(&snip(
            "a1",
            "BFS template",
            "from collections import deque\n",
        ))
        .await
        .unwrap();
        let all = db.snippets().await.unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "BFS template", "replaced by uid");
        assert_eq!(db.search_snippets("deque", 10).await.unwrap().len(), 1);
        assert_eq!(
            db.search_snippets("GRAPHS", 10).await.unwrap().len(),
            2,
            "tags, any case"
        );
        assert!(
            db.search_snippets("100%", 10).await.unwrap().is_empty(),
            "% is literal"
        );
        db.delete_snippet("b2").await.unwrap();
        assert!(db.delete_snippet("b2").await.is_err());
        assert_eq!(db.snippets().await.unwrap().len(), 1);
        db.close().await;
    }
}
