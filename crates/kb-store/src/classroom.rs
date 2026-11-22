//! Classroom mode (expansion M12, user.db migration 0013): a student's assignments, which
//! come with packs, and the results files a teacher imported. The rules (locks, the
//! results file and its hash chain, the CSV) live in kb-app.

use kb_core::{Language, Verdict};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, StoreError, UserDb};

/// `assignment.toml`, checked by kb-pack (problem ids already resolved to full ids).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct AssignmentMeta {
    pub title: String,
    /// `YYYY-MM-DD`.
    pub due: String,
    /// Full problem ids: `<pack>--<id>` for the pack's own problems, or built-in ids.
    pub problems: Vec<String>,
    /// The languages allowed on those problems while the assignment runs (SQL problems
    /// always take SQL).
    pub languages: Vec<Language>,
    /// A wall-clock limit from the start, in minutes.
    pub time_limit_min: Option<u32>,
    /// Whether hints and the editorial stay available.
    pub hints: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AssignmentRow {
    /// The pack id.
    pub id: String,
    pub pack_sha256: String,
    pub meta: AssignmentMeta,
    pub imported_at: i64,
    pub started_at: Option<i64>,
    pub submitted_at: Option<i64>,
}

fn assignment_row(r: &SqliteRow) -> Result<AssignmentRow> {
    Ok(AssignmentRow {
        id: r.try_get("id")?,
        pack_sha256: r.try_get("pack_sha256")?,
        meta: serde_json::from_str(&r.try_get::<String, _>("meta")?)?,
        imported_at: r.try_get("imported_at")?,
        started_at: r.try_get("started_at")?,
        submitted_at: r.try_get("submitted_at")?,
    })
}

/// A submission inside an assignment's window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSubmission {
    pub problem_id: String,
    pub language: Language,
    pub code: String,
    pub verdict: Verdict,
    pub created_at: i64,
}

/// One imported results file.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClassResultRow {
    pub id: i64,
    pub assignment_id: String,
    pub student: String,
    pub file_sha256: String,
    /// The `katabox-results/v1` file, as read.
    pub data: serde_json::Value,
    pub chain_ok: bool,
    pub imported_at: i64,
}

fn class_row(r: &SqliteRow) -> Result<ClassResultRow> {
    Ok(ClassResultRow {
        id: r.try_get("id")?,
        assignment_id: r.try_get("assignment_id")?,
        student: r.try_get("student")?,
        file_sha256: r.try_get("file_sha256")?,
        data: serde_json::from_str(&r.try_get::<String, _>("data")?)?,
        chain_ok: r.try_get::<i64, _>("chain_ok")? != 0,
        imported_at: r.try_get("imported_at")?,
    })
}

impl UserDb {
    // --- Student side ---------------------------------------------------------------

    /// Record an assignment that came with a pack. A newer pack replaces the text; the
    /// start and finish times are kept.
    pub async fn put_assignment(
        &self,
        id: &str,
        pack_sha256: &str,
        meta: &AssignmentMeta,
        now_ms: i64,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO assignments (id, pack_sha256, meta, imported_at, started_at, submitted_at)
             VALUES (?, ?, ?, ?, NULL, NULL)
             ON CONFLICT(id) DO UPDATE SET pack_sha256 = excluded.pack_sha256,
                 meta = excluded.meta, imported_at = excluded.imported_at",
        )
        .bind(id)
        .bind(pack_sha256)
        .bind(serde_json::to_string(meta)?)
        .bind(now_ms)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Every assignment ever imported, by id.
    pub async fn assignments(&self) -> Result<Vec<AssignmentRow>> {
        sqlx::query("SELECT * FROM assignments ORDER BY id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(assignment_row)
            .collect()
    }

    pub async fn assignment(&self, id: &str) -> Result<Option<AssignmentRow>> {
        sqlx::query("SELECT * FROM assignments WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(assignment_row)
            .transpose()
    }

    /// Start once: `false` when it had already started.
    pub async fn start_assignment(&self, id: &str, now_ms: i64) -> Result<bool> {
        Ok(
            sqlx::query(
                "UPDATE assignments SET started_at = ? WHERE id = ? AND started_at IS NULL",
            )
            .bind(now_ms)
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected()
                == 1,
        )
    }

    /// Finish once: `false` when it was not running.
    pub async fn finish_assignment(&self, id: &str, at_ms: i64) -> Result<bool> {
        Ok(sqlx::query(
            "UPDATE assignments SET submitted_at = ?
             WHERE id = ? AND started_at IS NOT NULL AND submitted_at IS NULL",
        )
        .bind(at_ms)
        .bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected()
            == 1)
    }

    /// Submissions of `problems` made between `from` and `to` (inclusive), oldest first.
    pub async fn window_submissions(
        &self,
        problems: &[String],
        from: i64,
        to: i64,
    ) -> Result<Vec<WindowSubmission>> {
        let mut out = Vec::new();
        for p in problems {
            let rows = sqlx::query(
                "SELECT problem_id, language, code, verdict, created_at FROM submissions
                 WHERE problem_id = ? AND created_at >= ? AND created_at <= ?
                 ORDER BY created_at, id",
            )
            .bind(p)
            .bind(from)
            .bind(to)
            .fetch_all(&self.pool)
            .await?;
            for r in rows {
                let lang: String = r.try_get("language")?;
                let v: String = r.try_get("verdict")?;
                out.push(WindowSubmission {
                    problem_id: r.try_get("problem_id")?,
                    language: lang
                        .parse()
                        .map_err(|_| StoreError::Corrupt(format!("unknown language {lang:?}")))?,
                    code: r.try_get("code")?,
                    verdict: serde_json::from_value(serde_json::Value::String(v.clone()))
                        .map_err(|_| StoreError::Corrupt(format!("unknown verdict {v:?}")))?,
                    created_at: r.try_get("created_at")?,
                });
            }
        }
        out.sort_by_key(|s| s.created_at);
        Ok(out)
    }

    // --- Teacher side ---------------------------------------------------------------

    /// Keep an imported results file: `false` when the same file was imported before.
    pub async fn add_class_result(
        &self,
        assignment_id: &str,
        student: &str,
        file_sha256: &str,
        data: &serde_json::Value,
        chain_ok: bool,
        now_ms: i64,
    ) -> Result<bool> {
        Ok(sqlx::query(
            "INSERT OR IGNORE INTO class_results
               (assignment_id, student, file_sha256, data, chain_ok, imported_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(assignment_id)
        .bind(student)
        .bind(file_sha256)
        .bind(serde_json::to_string(data)?)
        .bind(i64::from(chain_ok))
        .bind(now_ms)
        .execute(&self.pool)
        .await?
        .rows_affected()
            == 1)
    }

    /// Imported results, by assignment then student then import order.
    pub async fn class_results(&self) -> Result<Vec<ClassResultRow>> {
        sqlx::query("SELECT * FROM class_results ORDER BY assignment_id, student, id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(class_row)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn db(name: &str) -> UserDb {
        let dir =
            std::env::temp_dir().join(format!("kb-store-classroom-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        UserDb::open(&dir.join("user.db")).await.unwrap()
    }

    fn meta(title: &str) -> AssignmentMeta {
        AssignmentMeta {
            title: title.into(),
            due: "2026-12-01".into(),
            problems: vec!["drills--a".into(), "two-sum".into()],
            languages: vec![Language::Python],
            time_limit_min: Some(60),
            hints: false,
        }
    }

    #[tokio::test]
    async fn assignments_start_finish_and_keep_times_on_update() {
        let db = db("assign").await;
        db.put_assignment("drills", "aa", &meta("Week 1"), 10)
            .await
            .unwrap();
        assert!(db.start_assignment("drills", 20).await.unwrap());
        assert!(!db.start_assignment("drills", 30).await.unwrap());
        db.put_assignment("drills", "bb", &meta("Week 1 (fixed)"), 40)
            .await
            .unwrap();
        let a = db.assignment("drills").await.unwrap().unwrap();
        assert_eq!(a.meta.title, "Week 1 (fixed)");
        assert_eq!((a.pack_sha256.as_str(), a.started_at), ("bb", Some(20)));
        assert!(db.finish_assignment("drills", 50).await.unwrap());
        assert!(!db.finish_assignment("drills", 60).await.unwrap());
        assert_eq!(
            db.assignment("drills").await.unwrap().unwrap().submitted_at,
            Some(50)
        );
        assert_eq!(db.assignments().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn class_results_are_imported_once_per_file() {
        let db = db("class").await;
        let v = serde_json::json!({"student": "Ana"});
        assert!(
            db.add_class_result("drills", "Ana", "f1", &v, true, 1)
                .await
                .unwrap()
        );
        assert!(
            !db.add_class_result("drills", "Ana", "f1", &v, true, 2)
                .await
                .unwrap()
        );
        assert!(
            db.add_class_result("drills", "Bo", "f2", &v, false, 3)
                .await
                .unwrap()
        );
        let rows = db.class_results().await.unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0].chain_ok && !rows[1].chain_ok);
        assert_eq!(rows[1].data, v);
    }
}
