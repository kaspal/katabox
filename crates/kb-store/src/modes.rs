//! user.db queries for the practice modes (ADR 0020): problem opens, mock interview
//! sessions, and accepted solutions per language (cross-language practice). The rules
//! live in kb-app.

use std::collections::HashSet;

use kb_core::{Language, Verdict};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, StoreError, UserDb, now_ms};

/// A mock interview session row. `problem_ids` and `languages` are stored as JSON arrays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRow {
    pub id: i64,
    pub started_at: i64,
    pub duration_s: i64,
    pub problem_ids: Vec<String>,
    pub languages: Vec<Language>,
    pub mix: String,
    pub ended_at: Option<i64>,
    /// The scorecard (kb-app's JSON), once ended.
    pub results: Option<serde_json::Value>,
}

/// A submission's facts for a scorecard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitFact {
    pub problem_id: String,
    pub verdict: Verdict,
    pub created_at: i64,
}

/// The code of an Accepted submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AcceptedSolution {
    pub language: Language,
    pub code: String,
    pub submitted_at: i64,
}

fn session_row(r: &SqliteRow) -> Result<SessionRow> {
    let json = |col: &str| -> Result<String> { Ok(r.try_get::<String, _>(col)?) };
    Ok(SessionRow {
        id: r.try_get("id")?,
        started_at: r.try_get("started_at")?,
        duration_s: r.try_get("duration_s")?,
        problem_ids: serde_json::from_str(&json("problem_ids")?)?,
        languages: serde_json::from_str(&json("languages")?)?,
        mix: r.try_get("mix")?,
        ended_at: r.try_get("ended_at")?,
        results: r
            .try_get::<Option<String>, _>("results")?
            .map(|s| serde_json::from_str(&s))
            .transpose()?,
    })
}

fn parse_lang(s: &str) -> Result<Language> {
    s.parse()
        .map_err(|_| StoreError::Corrupt(format!("unknown language {s:?}")))
}

impl UserDb {
    // --- Problem opens --------------------------------------------------------------

    /// Record that the workspace opened `problem`.
    pub async fn mark_opened(&self, problem: &str) -> Result<()> {
        let now = now_ms();
        sqlx::query(
            "INSERT INTO problem_opens (problem_id, first_opened_at, last_opened_at)
             VALUES (?1, ?2, ?2)
             ON CONFLICT (problem_id) DO UPDATE SET last_opened_at = excluded.last_opened_at",
        )
        .bind(problem)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Every problem the user has touched: opened, drafted, run, submitted, has a status
    /// or unlocked a hint. Older databases (before opens were recorded) count through the
    /// other traces.
    pub async fn touched_problems(&self) -> Result<HashSet<String>> {
        let ids: Vec<String> = sqlx::query_scalar(
            "SELECT problem_id FROM problem_opens
             UNION SELECT problem_id FROM drafts
             UNION SELECT problem_id FROM snapshots
             UNION SELECT problem_id FROM submissions
             UNION SELECT problem_id FROM problem_status
             UNION SELECT problem_id FROM hint_unlocks",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ids.into_iter().collect())
    }

    // --- Interview sessions ---------------------------------------------------------

    pub async fn create_session(
        &self,
        started_at: i64,
        duration_s: i64,
        problem_ids: &[String],
        languages: &[Language],
        mix: &str,
    ) -> Result<SessionRow> {
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO interview_sessions (started_at, duration_s, problem_ids, languages, mix)
             VALUES (?, ?, ?, ?, ?) RETURNING id",
        )
        .bind(started_at)
        .bind(duration_s)
        .bind(serde_json::to_string(problem_ids)?)
        .bind(serde_json::to_string(languages)?)
        .bind(mix)
        .fetch_one(&self.pool)
        .await?;
        self.session(id).await
    }

    pub async fn session(&self, id: i64) -> Result<SessionRow> {
        sqlx::query("SELECT * FROM interview_sessions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(session_row)
            .transpose()?
            .ok_or_else(|| StoreError::NotFound(format!("interview {id}")))
    }

    /// The session not yet ended, if any (the newest one).
    pub async fn open_session(&self) -> Result<Option<SessionRow>> {
        sqlx::query(
            "SELECT * FROM interview_sessions WHERE ended_at IS NULL ORDER BY started_at DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?
        .as_ref()
        .map(session_row)
        .transpose()
    }

    /// Every session, newest first.
    pub async fn sessions(&self) -> Result<Vec<SessionRow>> {
        sqlx::query("SELECT * FROM interview_sessions ORDER BY started_at DESC")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(session_row)
            .collect()
    }

    /// End a session with its scorecard (no-op if it already ended).
    pub async fn end_session(
        &self,
        id: i64,
        ended_at: i64,
        results: &serde_json::Value,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE interview_sessions SET ended_at = ?, results = ? WHERE id = ? AND ended_at IS NULL",
        )
        .bind(ended_at)
        .bind(serde_json::to_string(results)?)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Submissions on `problems` created in `[from, to]`, oldest first.
    pub async fn submit_facts(
        &self,
        problems: &[String],
        from: i64,
        to: i64,
    ) -> Result<Vec<SubmitFact>> {
        let mut out = Vec::new();
        for p in problems {
            let rows = sqlx::query(
                "SELECT problem_id, verdict, created_at FROM submissions
                 WHERE problem_id = ? AND created_at >= ? AND created_at <= ? ORDER BY created_at, id",
            )
            .bind(p)
            .bind(from)
            .bind(to)
            .fetch_all(&self.pool)
            .await?;
            for r in rows {
                let v: String = r.try_get("verdict")?;
                out.push(SubmitFact {
                    problem_id: r.try_get("problem_id")?,
                    verdict: serde_json::from_value(serde_json::Value::String(v.clone()))
                        .map_err(|_| StoreError::Corrupt(format!("unknown verdict {v:?}")))?,
                    created_at: r.try_get("created_at")?,
                });
            }
        }
        out.sort_by_key(|f| f.created_at);
        Ok(out)
    }

    /// The problem a snapshot belongs to.
    pub async fn snapshot_problem(&self, id: i64) -> Result<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT problem_id FROM snapshots WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    // --- Cross-language practice ----------------------------------------------------

    /// The latest Accepted submission of `problem` in `language`.
    pub async fn accepted_solution(
        &self,
        problem: &str,
        language: Language,
    ) -> Result<Option<AcceptedSolution>> {
        sqlx::query(
            "SELECT code, created_at FROM submissions
             WHERE problem_id = ? AND language = ? AND verdict = 'accepted'
             ORDER BY created_at DESC, id DESC LIMIT 1",
        )
        .bind(problem)
        .bind(language.id())
        .fetch_optional(&self.pool)
        .await?
        .map(|r| {
            Ok(AcceptedSolution {
                language,
                code: r.try_get("code")?,
                submitted_at: r.try_get("created_at")?,
            })
        })
        .transpose()
    }

    /// Languages `problem` was solved in (an Accepted submission), in a stable order.
    pub async fn solved_languages(&self, problem: &str) -> Result<Vec<Language>> {
        let ids: Vec<String> = sqlx::query_scalar(
            "SELECT DISTINCT language FROM submissions
             WHERE problem_id = ? AND verdict = 'accepted' ORDER BY language",
        )
        .bind(problem)
        .fetch_all(&self.pool)
        .await?;
        ids.iter().map(|s| parse_lang(s)).collect()
    }

    /// For each language with an Accepted submission: the distinct problems solved in it.
    pub async fn solved_by_language(&self) -> Result<Vec<(Language, Vec<String>)>> {
        let rows = sqlx::query(
            "SELECT DISTINCT language, problem_id FROM submissions
             WHERE verdict = 'accepted' ORDER BY language, problem_id",
        )
        .fetch_all(&self.pool)
        .await?;
        let mut out: Vec<(Language, Vec<String>)> = Vec::new();
        for r in rows {
            let lang = parse_lang(&r.try_get::<String, _>("language")?)?;
            let p: String = r.try_get("problem_id")?;
            match out.last_mut() {
                Some((l, ps)) if *l == lang => ps.push(p),
                _ => out.push((lang, vec![p])),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;
    use crate::user::tests::report;

    async fn fresh(name: &str) -> (std::path::PathBuf, UserDb) {
        let dir = temp_dir(name);
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        (dir, db)
    }

    #[tokio::test]
    async fn touched_problems_count_every_trace() {
        let (_d, db) = fresh("touched").await;
        assert!(db.touched_problems().await.unwrap().is_empty());
        db.mark_opened("opened").await.unwrap();
        db.mark_opened("opened").await.unwrap();
        db.save_draft("drafted", Language::Rust, "fn x() {}")
            .await
            .unwrap();
        db.record_submission(
            "submitted",
            Language::Php,
            "<?php",
            &report(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
        db.unlock_hint("hinted", 1).await.unwrap();
        let t = db.touched_problems().await.unwrap();
        for p in ["opened", "drafted", "submitted", "hinted"] {
            assert!(t.contains(p), "{p}");
        }
        assert_eq!(t.len(), 4);
    }

    #[tokio::test]
    async fn sessions_round_trip_and_end_once() {
        let (_d, db) = fresh("sessions").await;
        assert_eq!(db.open_session().await.unwrap(), None);
        let s = db
            .create_session(
                1_000,
                1800,
                &["a".into(), "b".into()],
                &[Language::Python],
                "easy+medium",
            )
            .await
            .unwrap();
        assert_eq!(db.open_session().await.unwrap().unwrap().id, s.id);
        db.end_session(s.id, 2_000, &serde_json::json!({"solved": 1}))
            .await
            .unwrap();
        db.end_session(s.id, 9_000, &serde_json::json!({"solved": 2}))
            .await
            .unwrap();
        let ended = db.session(s.id).await.unwrap();
        assert_eq!(ended.ended_at, Some(2_000));
        assert_eq!(ended.results, Some(serde_json::json!({"solved": 1})));
        assert_eq!(db.open_session().await.unwrap(), None);
        assert_eq!(db.sessions().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn accepted_solutions_per_language() {
        let (_d, db) = fresh("accepted").await;
        db.record_submission(
            "p",
            Language::Python,
            "wrong",
            &report(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
        assert_eq!(
            db.accepted_solution("p", Language::Python).await.unwrap(),
            None
        );
        db.record_submission("p", Language::Python, "right", &report(Verdict::Accepted))
            .await
            .unwrap();
        db.record_submission("p", Language::Rust, "fn r() {}", &report(Verdict::Accepted))
            .await
            .unwrap();
        db.record_submission("q", Language::Rust, "fn q() {}", &report(Verdict::Accepted))
            .await
            .unwrap();
        let a = db
            .accepted_solution("p", Language::Python)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(a.code, "right");
        assert_eq!(
            db.solved_languages("p").await.unwrap(),
            [Language::Python, Language::Rust]
        );
        let by = db.solved_by_language().await.unwrap();
        assert_eq!(
            by,
            [
                (Language::Python, vec!["p".to_string()]),
                (Language::Rust, vec!["p".to_string(), "q".to_string()])
            ]
        );
        let facts = db.submit_facts(&["p".into()], 0, i64::MAX).await.unwrap();
        assert_eq!(facts.len(), 3);
        assert_eq!(facts[0].verdict, Verdict::WrongAnswer);
    }
}
