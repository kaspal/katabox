//! user.db queries for kata drills (expansion M1, ADR 0033). The rules (picks, rusty
//! scores, bests, streaks) live in kb-app; this module only reads and writes rows.

use std::collections::HashMap;

use kb_core::{Language, Verdict};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::user::{parse_language, parse_verdict, verdict_code};
use crate::{Result, StoreError, UserDb};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum KataKind {
    Daily,
    Pattern,
    List,
    Custom,
}

impl KataKind {
    pub fn id(&self) -> &'static str {
        match self {
            KataKind::Daily => "daily",
            KataKind::Pattern => "pattern",
            KataKind::List => "list",
            KataKind::Custom => "custom",
        }
    }

    pub fn parse(s: &str) -> Option<KataKind> {
        Some(match s {
            "daily" => KataKind::Daily,
            "pattern" => KataKind::Pattern,
            "list" => KataKind::List,
            "custom" => KataKind::Custom,
            _ => return None,
        })
    }
}

/// A session nobody ended stops counting as running this long after it started.
pub const KATA_MAX_MS: i64 = 2 * 60 * 60 * 1000;

/// One kata session (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataSessionRow {
    /// UTC milliseconds; the session's key.
    pub started_at: i64,
    /// The local day it started, `YYYY-MM-DD`.
    pub day: String,
    pub kind: KataKind,
    pub source: Option<String>,
    pub problem_ids: Vec<String>,
    pub duration_s: i64,
    pub ended_at: Option<i64>,
}

/// One problem tried in a kata session (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataAttemptRow {
    pub session_started_at: i64,
    pub problem_id: String,
    pub started_at: i64,
    pub submits: u32,
    pub verdict: Option<Verdict>,
    pub solved_at: Option<i64>,
    pub time_ms: Option<i64>,
    pub language: Option<Language>,
    pub code: Option<String>,
}

fn session_row(r: &SqliteRow) -> Result<KataSessionRow> {
    let kind: String = r.try_get("kind")?;
    let ids: String = r.try_get("problem_ids")?;
    Ok(KataSessionRow {
        started_at: r.try_get("started_at")?,
        day: r.try_get("day")?,
        kind: KataKind::parse(&kind)
            .ok_or_else(|| StoreError::Corrupt(format!("kata kind {kind:?}")))?,
        source: r.try_get("source")?,
        problem_ids: serde_json::from_str(&ids)?,
        duration_s: r.try_get("duration_s")?,
        ended_at: r.try_get("ended_at")?,
    })
}

fn attempt_row(r: &SqliteRow) -> Result<KataAttemptRow> {
    let verdict: Option<String> = r.try_get("verdict")?;
    let language: Option<String> = r.try_get("language")?;
    Ok(KataAttemptRow {
        session_started_at: r.try_get("session_started_at")?,
        problem_id: r.try_get("problem_id")?,
        started_at: r.try_get("started_at")?,
        submits: u32::try_from(r.try_get::<i64, _>("submits")?).unwrap_or(0),
        verdict: verdict.as_deref().map(parse_verdict).transpose()?,
        solved_at: r.try_get("solved_at")?,
        time_ms: r.try_get("time_ms")?,
        language: language.as_deref().map(parse_language).transpose()?,
        code: r.try_get("code")?,
    })
}

const SESSION_COLS: &str = "started_at, day, kind, source, problem_ids, duration_s, ended_at";
const ATTEMPT_COLS: &str = "session_started_at, problem_id, started_at, submits, verdict, solved_at, time_ms, language, code";

impl UserDb {
    /// Start a session. `started_at` must be unique (the caller passes the clock).
    pub async fn create_kata(&self, s: &KataSessionRow) -> Result<()> {
        sqlx::query(
            "INSERT INTO kata_sessions (started_at, day, kind, source, problem_ids, duration_s, ended_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(s.started_at)
        .bind(&s.day)
        .bind(s.kind.id())
        .bind(&s.source)
        .bind(serde_json::to_string(&s.problem_ids)?)
        .bind(s.duration_s)
        .bind(s.ended_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// The session not ended yet, if any (the newest one).
    pub async fn open_kata(&self) -> Result<Option<KataSessionRow>> {
        sqlx::query(&format!(
            "SELECT {SESSION_COLS} FROM kata_sessions WHERE ended_at IS NULL
             ORDER BY started_at DESC LIMIT 1"
        ))
        .fetch_optional(&self.pool)
        .await?
        .as_ref()
        .map(session_row)
        .transpose()
    }

    pub async fn kata(&self, started_at: i64) -> Result<KataSessionRow> {
        sqlx::query(&format!(
            "SELECT {SESSION_COLS} FROM kata_sessions WHERE started_at = ?"
        ))
        .bind(started_at)
        .fetch_optional(&self.pool)
        .await?
        .as_ref()
        .map(session_row)
        .transpose()?
        .ok_or_else(|| StoreError::NotFound(format!("kata {started_at}")))
    }

    /// Every session, newest first.
    pub async fn katas(&self) -> Result<Vec<KataSessionRow>> {
        sqlx::query(&format!(
            "SELECT {SESSION_COLS} FROM kata_sessions ORDER BY started_at DESC"
        ))
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(session_row)
        .collect()
    }

    pub async fn end_kata(&self, started_at: i64, ended_at: i64) -> Result<()> {
        sqlx::query(
            "UPDATE kata_sessions SET ended_at = ? WHERE started_at = ? AND ended_at IS NULL",
        )
        .bind(ended_at)
        .bind(started_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// The attempt row for `problem` in a session, created (started now) if missing.
    pub async fn open_kata_attempt(
        &self,
        session: i64,
        problem: &str,
        now: i64,
    ) -> Result<KataAttemptRow> {
        sqlx::query(
            "INSERT OR IGNORE INTO kata_attempts (session_started_at, problem_id, started_at)
             VALUES (?, ?, ?)",
        )
        .bind(session)
        .bind(problem)
        .bind(now)
        .execute(&self.pool)
        .await?;
        let r = sqlx::query(&format!(
            "SELECT {ATTEMPT_COLS} FROM kata_attempts WHERE session_started_at = ? AND problem_id = ?"
        ))
        .bind(session)
        .bind(problem)
        .fetch_one(&self.pool)
        .await?;
        attempt_row(&r)
    }

    /// Record one kata Submit. The first Accepted fixes `solved_at`, `time_ms`, the
    /// language and the code; later submits only count.
    pub async fn record_kata_submit(
        &self,
        session: i64,
        problem: &str,
        language: Language,
        code: &str,
        verdict: Verdict,
        now: i64,
    ) -> Result<KataAttemptRow> {
        let a = self.open_kata_attempt(session, problem, now).await?;
        let first_accept = verdict.is_accepted() && a.solved_at.is_none();
        sqlx::query(
            "UPDATE kata_attempts SET submits = submits + 1, verdict = ?,
               solved_at = CASE WHEN ? THEN ? ELSE solved_at END,
               time_ms = CASE WHEN ? THEN ? - started_at ELSE time_ms END,
               language = CASE WHEN ? THEN ? ELSE language END,
               code = CASE WHEN ? THEN ? ELSE code END
             WHERE session_started_at = ? AND problem_id = ?",
        )
        .bind(verdict_code(verdict)?)
        .bind(first_accept)
        .bind(now)
        .bind(first_accept)
        .bind(now)
        .bind(first_accept)
        .bind(language.id())
        .bind(first_accept)
        .bind(code)
        .bind(session)
        .bind(problem)
        .execute(&self.pool)
        .await?;
        self.open_kata_attempt(session, problem, now).await
    }

    /// Every attempt (all sessions), oldest first.
    pub async fn kata_attempts(&self) -> Result<Vec<KataAttemptRow>> {
        sqlx::query(&format!(
            "SELECT {ATTEMPT_COLS} FROM kata_attempts ORDER BY started_at, problem_id"
        ))
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(attempt_row)
        .collect()
    }

    /// Every submission as `(problem, accepted, created_at)`, oldest first (the skill
    /// rating's replay, expansion M2).
    pub async fn submission_outcomes(&self) -> Result<Vec<(String, bool, i64)>> {
        let rows = sqlx::query(
            "SELECT problem_id, verdict = 'accepted' AS ok, created_at FROM submissions
             ORDER BY created_at, id",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|r| {
                Ok((
                    r.try_get("problem_id")?,
                    r.try_get::<i64, _>("ok")? != 0,
                    r.try_get("created_at")?,
                ))
            })
            .collect()
    }

    /// When each problem was first opened (UTC ms).
    pub async fn first_opens(&self) -> Result<HashMap<String, i64>> {
        let rows = sqlx::query("SELECT problem_id, first_opened_at FROM problem_opens")
            .fetch_all(&self.pool)
            .await?;
        rows.iter()
            .map(|r| Ok((r.try_get("problem_id")?, r.try_get("first_opened_at")?)))
            .collect()
    }

    /// The latest Accepted submission time of every problem solved (UTC ms).
    pub async fn last_accepts(&self) -> Result<HashMap<String, i64>> {
        let rows = sqlx::query(
            "SELECT problem_id, max(created_at) AS at FROM submissions
             WHERE verdict = 'accepted' GROUP BY problem_id",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|r| Ok((r.try_get("problem_id")?, r.try_get("at")?)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;

    fn session(at: i64) -> KataSessionRow {
        KataSessionRow {
            started_at: at,
            day: "2026-11-21".into(),
            kind: KataKind::Daily,
            source: None,
            problem_ids: vec!["two-sum".into(), "budget-pair".into()],
            duration_s: 900,
            ended_at: None,
        }
    }

    #[tokio::test]
    async fn sessions_and_attempts() {
        let dir = temp_dir("kata");
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        assert_eq!(db.open_kata().await.unwrap(), None);
        db.create_kata(&session(1_000)).await.unwrap();
        assert_eq!(db.open_kata().await.unwrap().unwrap().started_at, 1_000);

        let a = db.open_kata_attempt(1_000, "two-sum", 1_100).await.unwrap();
        assert_eq!((a.started_at, a.submits, a.solved_at), (1_100, 0, None));
        // Opening again keeps the first start.
        let a = db.open_kata_attempt(1_000, "two-sum", 1_200).await.unwrap();
        assert_eq!(a.started_at, 1_100);

        let a = db
            .record_kata_submit(
                1_000,
                "two-sum",
                Language::Python,
                "bad",
                Verdict::WrongAnswer,
                1_300,
            )
            .await
            .unwrap();
        assert_eq!(
            (a.submits, a.verdict, a.solved_at),
            (1, Some(Verdict::WrongAnswer), None)
        );
        let a = db
            .record_kata_submit(
                1_000,
                "two-sum",
                Language::Rust,
                "good",
                Verdict::Accepted,
                1_600,
            )
            .await
            .unwrap();
        assert_eq!(
            (a.submits, a.solved_at, a.time_ms),
            (2, Some(1_600), Some(500))
        );
        assert_eq!(
            (a.language, a.code.as_deref()),
            (Some(Language::Rust), Some("good"))
        );
        // A later Accepted does not move the first one.
        let a = db
            .record_kata_submit(
                1_000,
                "two-sum",
                Language::Python,
                "later",
                Verdict::Accepted,
                2_000,
            )
            .await
            .unwrap();
        assert_eq!(
            (a.submits, a.solved_at, a.code.as_deref()),
            (3, Some(1_600), Some("good"))
        );

        db.end_kata(1_000, 3_000).await.unwrap();
        assert_eq!(db.open_kata().await.unwrap(), None);
        assert_eq!(db.kata(1_000).await.unwrap().ended_at, Some(3_000));
        assert_eq!(db.katas().await.unwrap().len(), 1);
        assert_eq!(db.kata_attempts().await.unwrap().len(), 1);
        // Kata submits never touch ordinary submissions.
        assert!(db.last_accepts().await.unwrap().is_empty());
        db.close().await;
    }
}
