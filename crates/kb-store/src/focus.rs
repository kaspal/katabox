//! Focus sessions (expansion M10, user.db migration 0011): finished sessions and what
//! happened during them, counted from the other tables over the session's time window.

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::Result;
use crate::user::UserDb;

/// What happened during a session.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct FocusSummary {
    /// Problems opened or submitted.
    pub touched: u32,
    /// Problems with an Accepted submission.
    pub solved: u32,
    /// Kata attempts solved.
    pub katas: u32,
    /// Study plan items marked done.
    pub plan_items: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct FocusSessionRow {
    pub started_at: i64,
    pub ended_at: i64,
    pub work_min: u32,
    pub break_min: u32,
    pub cycles: u32,
    pub summary: FocusSummary,
}

fn session_row(r: &SqliteRow) -> Result<FocusSessionRow> {
    Ok(FocusSessionRow {
        started_at: r.try_get("started_at")?,
        ended_at: r.try_get("ended_at")?,
        work_min: r.try_get::<i64, _>("work_min")? as u32,
        break_min: r.try_get::<i64, _>("break_min")? as u32,
        cycles: r.try_get::<i64, _>("cycles")? as u32,
        summary: serde_json::from_str(&r.try_get::<String, _>("summary")?)?,
    })
}

impl UserDb {
    /// Finished sessions, newest first.
    pub async fn focus_sessions(&self) -> Result<Vec<FocusSessionRow>> {
        sqlx::query("SELECT * FROM focus_sessions ORDER BY started_at DESC")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(session_row)
            .collect()
    }

    pub async fn put_focus_session(&self, s: &FocusSessionRow) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO focus_sessions
               (started_at, ended_at, work_min, break_min, cycles, summary)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(s.started_at)
        .bind(s.ended_at)
        .bind(s.work_min)
        .bind(s.break_min)
        .bind(s.cycles)
        .bind(serde_json::to_string(&s.summary)?)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// What happened in `[from, to]` (UTC ms).
    pub async fn focus_summary(&self, from: i64, to: i64) -> Result<FocusSummary> {
        let count = |sql: &'static str| {
            sqlx::query_scalar::<_, i64>(sql)
                .bind(from)
                .bind(to)
                .bind(from)
                .bind(to)
                .fetch_one(&self.pool)
        };
        let touched = count(
            "SELECT COUNT(*) FROM (
               SELECT problem_id FROM problem_opens WHERE last_opened_at BETWEEN ? AND ?
               UNION SELECT problem_id FROM submissions WHERE created_at BETWEEN ? AND ?)",
        )
        .await?;
        let solved = count(
            "SELECT COUNT(DISTINCT problem_id) FROM submissions
             WHERE verdict = 'accepted' AND created_at BETWEEN ? AND ? AND ? <= ?",
        )
        .await?;
        let katas =
            count("SELECT COUNT(*) FROM kata_attempts WHERE solved_at BETWEEN ? AND ? AND ? <= ?")
                .await?;
        let plan_items = count(
            "SELECT COUNT(*) FROM plan_items
             WHERE status = 'done' AND updated_at BETWEEN ? AND ? AND ? <= ?",
        )
        .await?;
        let n = |v: i64| u32::try_from(v).unwrap_or(0);
        Ok(FocusSummary {
            touched: n(touched),
            solved: n(solved),
            katas: n(katas),
            plan_items: n(plan_items),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::tests::temp_dir;

    #[tokio::test]
    async fn the_summary_counts_the_window_only() {
        let db = UserDb::open(&temp_dir("focus").join("user.db"))
            .await
            .unwrap();
        for sql in [
            "INSERT INTO problem_opens VALUES ('a', 100, 150), ('b', 10, 20)",
            "INSERT INTO submissions (problem_id, language, code, verdict, time_us, memory_kb, report, created_at)
             VALUES ('c', 'python', '', 'accepted', 1, 1, '{}', 160),
                    ('c', 'python', '', 'accepted', 1, 1, '{}', 170),
                    ('d', 'python', '', 'wrong_answer', 1, 1, '{}', 180),
                    ('e', 'python', '', 'accepted', 1, 1, '{}', 999)",
        ] {
            sqlx::raw_sql(sql).execute(&db.pool).await.unwrap();
        }
        let s = db.focus_summary(100, 200).await.unwrap();
        assert_eq!(
            s,
            FocusSummary {
                touched: 3,
                solved: 1,
                katas: 0,
                plan_items: 0
            }
        );
        let row = FocusSessionRow {
            started_at: 100,
            ended_at: 200,
            work_min: 25,
            break_min: 5,
            cycles: 1,
            summary: s,
        };
        db.put_focus_session(&row).await.unwrap();
        assert_eq!(db.focus_sessions().await.unwrap(), vec![row]);
    }
}
