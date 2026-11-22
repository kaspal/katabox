//! user.db queries for the mistake journal and reflections (expansion M3, ADR 0034).
//! The tagging rules live in kb-app; this module only reads and writes rows.

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, UserDb, now_ms};

/// One failed Submit's journal entry (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MistakeRow {
    pub problem_id: String,
    /// The language id, as stored on the submission.
    pub language: String,
    /// The submission's `created_at` (UTC ms).
    pub submitted_at: i64,
    pub auto_tags: Vec<String>,
    pub tags: Vec<String>,
    pub note: String,
    pub updated_at: i64,
}

/// A reflection after Accepted (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReflectionRow {
    pub problem_id: String,
    pub why: String,
    pub time_class: Option<String>,
    pub space_class: Option<String>,
    pub updated_at: i64,
}

/// A failed submission without a journal entry yet.
#[derive(Debug, Clone, PartialEq)]
pub struct UntaggedFailure {
    pub problem_id: String,
    pub language: String,
    pub submitted_at: i64,
    pub report: serde_json::Value,
}

fn mistake_row(r: &SqliteRow) -> Result<MistakeRow> {
    Ok(MistakeRow {
        problem_id: r.try_get("problem_id")?,
        language: r.try_get("language")?,
        submitted_at: r.try_get("submitted_at")?,
        auto_tags: serde_json::from_str(&r.try_get::<String, _>("auto_tags")?)?,
        tags: serde_json::from_str(&r.try_get::<String, _>("tags")?)?,
        note: r.try_get("note")?,
        updated_at: r.try_get("updated_at")?,
    })
}

fn reflection_row(r: &SqliteRow) -> Result<ReflectionRow> {
    Ok(ReflectionRow {
        problem_id: r.try_get("problem_id")?,
        why: r.try_get("why")?,
        time_class: r.try_get("time_class")?,
        space_class: r.try_get("space_class")?,
        updated_at: r.try_get("updated_at")?,
    })
}

impl UserDb {
    /// Failed submissions that have no journal entry, oldest first.
    pub async fn untagged_failures(&self) -> Result<Vec<UntaggedFailure>> {
        let rows = sqlx::query(
            "SELECT s.problem_id, s.language, s.created_at, s.report FROM submissions s
             WHERE s.verdict <> 'accepted' AND NOT EXISTS (
               SELECT 1 FROM mistakes m WHERE m.problem_id = s.problem_id
                 AND m.language = s.language AND m.submitted_at = s.created_at)
             ORDER BY s.created_at, s.id",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|r| {
                Ok(UntaggedFailure {
                    problem_id: r.try_get("problem_id")?,
                    language: r.try_get("language")?,
                    submitted_at: r.try_get("created_at")?,
                    report: serde_json::from_str(&r.try_get::<String, _>("report")?)?,
                })
            })
            .collect()
    }

    /// Store the automatic tags of a failure (the user's tags start as these). A
    /// second call for the same submission changes nothing.
    pub async fn add_mistake(
        &self,
        problem: &str,
        language: &str,
        submitted_at: i64,
        auto_tags: &[String],
    ) -> Result<()> {
        let tags = serde_json::to_string(auto_tags)?;
        sqlx::query(
            "INSERT OR IGNORE INTO mistakes (problem_id, language, submitted_at, auto_tags, tags, note, updated_at)
             VALUES (?, ?, ?, ?, ?, '', ?)",
        )
        .bind(problem)
        .bind(language)
        .bind(submitted_at)
        .bind(&tags)
        .bind(&tags)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mistake(
        &self,
        problem: &str,
        language: &str,
        submitted_at: i64,
    ) -> Result<Option<MistakeRow>> {
        sqlx::query(
            "SELECT * FROM mistakes WHERE problem_id = ? AND language = ? AND submitted_at = ?",
        )
        .bind(problem)
        .bind(language)
        .bind(submitted_at)
        .fetch_optional(&self.pool)
        .await?
        .as_ref()
        .map(mistake_row)
        .transpose()
    }

    /// Every journal entry, newest first.
    pub async fn mistakes(&self) -> Result<Vec<MistakeRow>> {
        sqlx::query("SELECT * FROM mistakes ORDER BY submitted_at DESC, problem_id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(mistake_row)
            .collect()
    }

    /// The user's tags and note for a failure.
    pub async fn set_mistake(
        &self,
        problem: &str,
        language: &str,
        submitted_at: i64,
        tags: &[String],
        note: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE mistakes SET tags = ?, note = ?, updated_at = ?
             WHERE problem_id = ? AND language = ? AND submitted_at = ?",
        )
        .bind(serde_json::to_string(tags)?)
        .bind(note)
        .bind(now_ms())
        .bind(problem)
        .bind(language)
        .bind(submitted_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Add one tag (to both lists) unless present: KataBox found it later (the shrinker).
    pub async fn add_mistake_tag(
        &self,
        problem: &str,
        language: &str,
        submitted_at: i64,
        tag: &str,
    ) -> Result<()> {
        let Some(m) = self.mistake(problem, language, submitted_at).await? else {
            return Ok(());
        };
        let mut auto = m.auto_tags.clone();
        let mut tags = m.tags.clone();
        for v in [&mut auto, &mut tags] {
            if !v.iter().any(|t| t == tag) {
                v.push(tag.to_string());
            }
        }
        sqlx::query(
            "UPDATE mistakes SET auto_tags = ?, tags = ?, updated_at = ?
             WHERE problem_id = ? AND language = ? AND submitted_at = ?",
        )
        .bind(serde_json::to_string(&auto)?)
        .bind(serde_json::to_string(&tags)?)
        .bind(now_ms())
        .bind(problem)
        .bind(language)
        .bind(submitted_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn reflection(&self, problem: &str) -> Result<Option<ReflectionRow>> {
        sqlx::query("SELECT * FROM reflections WHERE problem_id = ?")
            .bind(problem)
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(reflection_row)
            .transpose()
    }

    pub async fn reflections(&self) -> Result<Vec<ReflectionRow>> {
        sqlx::query("SELECT * FROM reflections ORDER BY problem_id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(reflection_row)
            .collect()
    }

    pub async fn set_reflection(
        &self,
        problem: &str,
        why: &str,
        time_class: Option<&str>,
        space_class: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO reflections (problem_id, why, time_class, space_class, updated_at)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT (problem_id) DO UPDATE SET why = excluded.why,
               time_class = excluded.time_class, space_class = excluded.space_class,
               updated_at = excluded.updated_at",
        )
        .bind(problem)
        .bind(why)
        .bind(time_class)
        .bind(space_class)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;
    use crate::user::tests::report;
    use kb_core::{Language, Verdict};

    #[tokio::test]
    async fn mistakes_and_reflections() {
        let dir = temp_dir("journal");
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        db.record_submission("p", Language::Python, "a", &report(Verdict::WrongAnswer))
            .await
            .unwrap();
        db.record_submission("p", Language::Python, "b", &report(Verdict::Accepted))
            .await
            .unwrap();
        let open = db.untagged_failures().await.unwrap();
        assert_eq!(open.len(), 1, "only the failure");
        let f = &open[0];
        db.add_mistake(&f.problem_id, &f.language, f.submitted_at, &["wa".into()])
            .await
            .unwrap();
        // Adding again is a no-op; nothing is left untagged.
        db.add_mistake(&f.problem_id, &f.language, f.submitted_at, &["tle".into()])
            .await
            .unwrap();
        assert!(db.untagged_failures().await.unwrap().is_empty());
        db.set_mistake(
            "p",
            "python",
            f.submitted_at,
            &["wa".into(), "edge:empty".into()],
            "empty list",
        )
        .await
        .unwrap();
        db.add_mistake_tag("p", "python", f.submitted_at, "shrunk")
            .await
            .unwrap();
        let m = db
            .mistake("p", "python", f.submitted_at)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(m.auto_tags, ["wa", "shrunk"]);
        assert_eq!(m.tags, ["wa", "edge:empty", "shrunk"]);
        assert_eq!(m.note, "empty list");
        assert_eq!(db.mistakes().await.unwrap().len(), 1);

        assert_eq!(db.reflection("p").await.unwrap(), None);
        db.set_reflection("p", "two pointers meet", Some("O(n)"), None)
            .await
            .unwrap();
        db.set_reflection(
            "p",
            "two pointers meet in the middle",
            Some("O(n)"),
            Some("O(1)"),
        )
        .await
        .unwrap();
        let r = db.reflection("p").await.unwrap().unwrap();
        assert_eq!(
            (r.why.as_str(), r.space_class.as_deref()),
            ("two pointers meet in the middle", Some("O(1)"))
        );
        assert_eq!(db.reflections().await.unwrap().len(), 1);
        db.close().await;
    }
}
