//! user.db queries for the practice features (ADR 0019): reviews, daily picks, lists,
//! and the install id. The rules (schedules, picks, streaks) live in kb-app; this module
//! only reads and writes rows.

use std::hash::{BuildHasher, Hasher};

use kb_core::Day;
use serde::Serialize;
use serde_json::Value;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, StoreError, UserDb, now_ms};

/// The settings key holding this installation's random id (the daily pick's seed).
pub const INSTALL_ID_SETTING: &str = "app.install_id";

/// A problem's place in the review queue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReviewRow {
    pub problem_id: String,
    /// `YYYY-MM-DD`.
    pub due_on: String,
    pub interval_days: u32,
    pub passes: u32,
    /// Failed this round: the next pass schedules 3 days, not the next interval.
    pub lapsed: bool,
}

impl ReviewRow {
    pub fn due(&self) -> Result<Day> {
        self.due_on
            .parse()
            .map_err(|e: String| StoreError::Corrupt(format!("review due date: {e}")))
    }
}

/// The daily problem picked for a day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyPick {
    pub day: Day,
    pub problem_id: String,
    pub solved: bool,
}

/// One Submit, for the Progress heatmap: the UI buckets `at` (UTC ms) into local days.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ActivityRow {
    /// UTC milliseconds.
    pub at: i64,
    pub problem_id: String,
    pub accepted: bool,
}

/// A personal list without its items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListRow {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
}

fn review_row(r: &SqliteRow) -> Result<ReviewRow> {
    Ok(ReviewRow {
        problem_id: r.try_get("problem_id")?,
        due_on: r.try_get("due_on")?,
        interval_days: u32::try_from(r.try_get::<i64, _>("interval_days")?).unwrap_or(3),
        passes: u32::try_from(r.try_get::<i64, _>("passes")?).unwrap_or(0),
        lapsed: r.try_get::<i64, _>("lapsed")? != 0,
    })
}

impl UserDb {
    // --- Install id -----------------------------------------------------------------

    /// This installation's random id, created on first use (32 hex characters).
    pub async fn install_id(&self) -> Result<String> {
        if let Some(Value::String(id)) = self.get_setting(INSTALL_ID_SETTING).await? {
            if !id.is_empty() {
                return Ok(id);
            }
        }
        // Two independently keyed std hashers: random per process, seeded by the OS.
        let part = || {
            let mut h = std::collections::hash_map::RandomState::new().build_hasher();
            h.write_i64(now_ms());
            h.write_u32(std::process::id());
            h.finish()
        };
        let id = format!("{:016x}{:016x}", part(), part());
        self.set_setting(INSTALL_ID_SETTING, &Value::String(id.clone()))
            .await?;
        Ok(id)
    }

    // --- Reviews --------------------------------------------------------------------

    pub async fn review(&self, problem: &str) -> Result<Option<ReviewRow>> {
        sqlx::query("SELECT * FROM reviews WHERE problem_id = ?")
            .bind(problem)
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(review_row)
            .transpose()
    }

    /// Reviews due on or before `day`, earliest first.
    pub async fn reviews_due(&self, day: Day) -> Result<Vec<ReviewRow>> {
        sqlx::query("SELECT * FROM reviews WHERE due_on <= ? ORDER BY due_on, problem_id")
            .bind(day.to_string())
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(review_row)
            .collect()
    }

    /// Insert or replace a problem's review row.
    pub async fn put_review(&self, r: &ReviewRow) -> Result<()> {
        sqlx::query(
            "INSERT INTO reviews (problem_id, due_on, interval_days, passes, lapsed, created_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT (problem_id) DO UPDATE SET due_on = excluded.due_on,
                 interval_days = excluded.interval_days, passes = excluded.passes,
                 lapsed = excluded.lapsed",
        )
        .bind(&r.problem_id)
        .bind(&r.due_on)
        .bind(i64::from(r.interval_days))
        .bind(i64::from(r.passes))
        .bind(i64::from(r.lapsed))
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Every scheduled review (graduated problems have no row), earliest first.
    pub async fn review_rows(&self) -> Result<Vec<ReviewRow>> {
        sqlx::query("SELECT * FROM reviews ORDER BY due_on, problem_id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(review_row)
            .collect()
    }

    // --- Activity -------------------------------------------------------------------

    /// Submits made at or after `from_ms` (UTC ms), oldest first.
    pub async fn activity_since(&self, from_ms: i64) -> Result<Vec<ActivityRow>> {
        let accepted = crate::user::verdict_code(kb_core::Verdict::Accepted)?;
        sqlx::query(
            "SELECT created_at, problem_id, verdict = ? AS accepted FROM submissions
             WHERE created_at >= ? ORDER BY created_at, id",
        )
        .bind(accepted)
        .bind(from_ms)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| {
            Ok(ActivityRow {
                at: r.try_get("created_at")?,
                problem_id: r.try_get("problem_id")?,
                accepted: r.try_get::<i64, _>("accepted")? != 0,
            })
        })
        .collect()
    }

    /// Remove a problem from the review queue. Returns whether it was there.
    pub async fn delete_review(&self, problem: &str) -> Result<bool> {
        let n = sqlx::query("DELETE FROM reviews WHERE problem_id = ?")
            .bind(problem)
            .execute(&self.pool)
            .await?
            .rows_affected();
        Ok(n > 0)
    }

    // --- Daily problem --------------------------------------------------------------

    pub async fn daily_pick(&self, day: Day) -> Result<Option<DailyPick>> {
        sqlx::query("SELECT problem_id, solved_at FROM daily_picks WHERE day = ?")
            .bind(day.to_string())
            .fetch_optional(&self.pool)
            .await?
            .map(|r| {
                Ok(DailyPick {
                    day,
                    problem_id: r.try_get("problem_id")?,
                    solved: r.try_get::<Option<i64>, _>("solved_at")?.is_some(),
                })
            })
            .transpose()
    }

    /// Store `problem` as `day`'s pick unless that day already has one; returns the
    /// day's pick either way (the first pick of a day wins).
    pub async fn put_daily_pick(&self, day: Day, problem: &str) -> Result<DailyPick> {
        sqlx::query("INSERT OR IGNORE INTO daily_picks (day, problem_id) VALUES (?, ?)")
            .bind(day.to_string())
            .bind(problem)
            .execute(&self.pool)
            .await?;
        self.daily_pick(day)
            .await?
            .ok_or_else(|| StoreError::Corrupt("daily pick vanished".into()))
    }

    /// Replace `day`'s pick (its problem left the problem pack).
    pub async fn replace_daily_pick(&self, day: Day, problem: &str) -> Result<DailyPick> {
        sqlx::query("INSERT OR REPLACE INTO daily_picks (day, problem_id) VALUES (?, ?)")
            .bind(day.to_string())
            .bind(problem)
            .execute(&self.pool)
            .await?;
        self.daily_pick(day)
            .await?
            .ok_or_else(|| StoreError::Corrupt("daily pick vanished".into()))
    }

    /// Mark `day`'s pick solved if it is `problem`. Returns whether it changed.
    pub async fn solve_daily_pick(&self, day: Day, problem: &str) -> Result<bool> {
        let n = sqlx::query(
            "UPDATE daily_picks SET solved_at = ? WHERE day = ? AND problem_id = ?
             AND solved_at IS NULL",
        )
        .bind(now_ms())
        .bind(day.to_string())
        .bind(problem)
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(n > 0)
    }

    /// Days whose pick was solved, newest first, on or before `day`.
    pub async fn solved_daily_days(&self, day: Day) -> Result<Vec<Day>> {
        let rows: Vec<String> = sqlx::query_scalar(
            "SELECT day FROM daily_picks WHERE solved_at IS NOT NULL AND day <= ?
             ORDER BY day DESC",
        )
        .bind(day.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|s| {
                s.parse()
                    .map_err(|e: String| StoreError::Corrupt(format!("daily pick day: {e}")))
            })
            .collect()
    }

    // --- Lists ----------------------------------------------------------------------

    pub async fn lists(&self) -> Result<Vec<ListRow>> {
        sqlx::query("SELECT id, name, created_at FROM lists ORDER BY created_at, id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|r| {
                Ok(ListRow {
                    id: r.try_get("id")?,
                    name: r.try_get("name")?,
                    created_at: r.try_get("created_at")?,
                })
            })
            .collect()
    }

    pub async fn create_list(&self, name: &str) -> Result<ListRow> {
        let now = now_ms();
        let id: i64 =
            sqlx::query_scalar("INSERT INTO lists (name, created_at) VALUES (?, ?) RETURNING id")
                .bind(name)
                .bind(now)
                .fetch_one(&self.pool)
                .await?;
        Ok(ListRow {
            id,
            name: name.to_string(),
            created_at: now,
        })
    }

    pub async fn rename_list(&self, id: i64, name: &str) -> Result<()> {
        let n = sqlx::query("UPDATE lists SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();
        if n == 0 {
            return Err(StoreError::NotFound(format!("list {id}")));
        }
        Ok(())
    }

    /// Delete a list and its items.
    pub async fn delete_list(&self, id: i64) -> Result<()> {
        let n = sqlx::query("DELETE FROM lists WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();
        if n == 0 {
            return Err(StoreError::NotFound(format!("list {id}")));
        }
        Ok(())
    }

    /// The problem ids of a list, in order.
    pub async fn list_items(&self, id: i64) -> Result<Vec<String>> {
        Ok(sqlx::query_scalar(
            "SELECT problem_id FROM list_items WHERE list_id = ? ORDER BY position, added_at",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?)
    }

    /// Ids of the lists holding `problem`.
    pub async fn lists_with(&self, problem: &str) -> Result<Vec<i64>> {
        Ok(sqlx::query_scalar(
            "SELECT list_id FROM list_items WHERE problem_id = ? ORDER BY list_id",
        )
        .bind(problem)
        .fetch_all(&self.pool)
        .await?)
    }

    /// Append `problem` to a list (no-op if already there).
    pub async fn add_to_list(&self, id: i64, problem: &str) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM lists WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
        if exists.is_none() {
            return Err(StoreError::NotFound(format!("list {id}")));
        }
        sqlx::query(
            "INSERT OR IGNORE INTO list_items (list_id, problem_id, position, added_at)
             VALUES (?1, ?2, (SELECT COALESCE(MAX(position), -1) + 1 FROM list_items
                              WHERE list_id = ?1), ?3)",
        )
        .bind(id)
        .bind(problem)
        .bind(now_ms())
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn remove_from_list(&self, id: i64, problem: &str) -> Result<()> {
        sqlx::query("DELETE FROM list_items WHERE list_id = ? AND problem_id = ?")
            .bind(id)
            .bind(problem)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Put a list's items in `order`, which must hold exactly the list's problems.
    pub async fn reorder_list(&self, id: i64, order: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let mut current: Vec<String> =
            sqlx::query_scalar("SELECT problem_id FROM list_items WHERE list_id = ?")
                .bind(id)
                .fetch_all(&mut *tx)
                .await?;
        let mut wanted = order.to_vec();
        current.sort();
        wanted.sort();
        if current != wanted {
            return Err(StoreError::Invalid(
                "reorder must list exactly the list's problems".into(),
            ));
        }
        for (pos, problem) in order.iter().enumerate() {
            sqlx::query("UPDATE list_items SET position = ? WHERE list_id = ? AND problem_id = ?")
                .bind(i64::try_from(pos).unwrap_or(i64::MAX))
                .bind(id)
                .bind(problem)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;

    async fn fresh(name: &str) -> (std::path::PathBuf, UserDb) {
        let dir = temp_dir(name);
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        (dir, db)
    }

    fn day(s: &str) -> Day {
        s.parse().unwrap()
    }

    #[tokio::test]
    async fn install_id_is_created_once_and_kept() {
        let (dir, db) = fresh("install-id").await;
        let id = db.install_id().await.unwrap();
        assert_eq!(id.len(), 32);
        assert_eq!(db.install_id().await.unwrap(), id);
        db.close().await;
        let again = UserDb::open(&dir.join("user.db")).await.unwrap();
        assert_eq!(again.install_id().await.unwrap(), id);
        let (_d2, other) = fresh("install-id-2").await;
        assert_ne!(other.install_id().await.unwrap(), id);
    }

    #[tokio::test]
    async fn reviews_round_trip_and_due_order() {
        let (_dir, db) = fresh("reviews").await;
        for (p, due) in [
            ("b", "2026-11-14"),
            ("a", "2026-11-14"),
            ("c", "2026-11-20"),
        ] {
            db.put_review(&ReviewRow {
                problem_id: p.into(),
                due_on: due.into(),
                interval_days: 3,
                passes: 0,
                lapsed: false,
            })
            .await
            .unwrap();
        }
        let due: Vec<String> = db
            .reviews_due(day("2026-11-15"))
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.problem_id)
            .collect();
        assert_eq!(due, ["a", "b"]);
        let mut a = db.review("a").await.unwrap().unwrap();
        a.interval_days = 7;
        a.passes = 1;
        a.lapsed = true;
        db.put_review(&a).await.unwrap();
        assert_eq!(db.review("a").await.unwrap().unwrap(), a);
        assert!(db.delete_review("a").await.unwrap());
        assert!(!db.delete_review("a").await.unwrap());
        assert_eq!(db.review("a").await.unwrap(), None);
    }

    #[tokio::test]
    async fn the_first_daily_pick_of_a_day_wins() {
        let (_dir, db) = fresh("daily").await;
        let d = day("2026-11-14");
        assert_eq!(db.daily_pick(d).await.unwrap(), None);
        assert_eq!(
            db.put_daily_pick(d, "two-sum").await.unwrap().problem_id,
            "two-sum"
        );
        assert_eq!(
            db.put_daily_pick(d, "other").await.unwrap().problem_id,
            "two-sum"
        );
        assert!(!db.solve_daily_pick(d, "other").await.unwrap());
        assert!(db.solve_daily_pick(d, "two-sum").await.unwrap());
        assert!(!db.solve_daily_pick(d, "two-sum").await.unwrap());
        assert!(db.daily_pick(d).await.unwrap().unwrap().solved);
        db.put_daily_pick(d.add_days(-2), "x").await.unwrap();
        db.solve_daily_pick(d.add_days(-2), "x").await.unwrap();
        db.put_daily_pick(d.add_days(1), "y").await.unwrap();
        db.solve_daily_pick(d.add_days(1), "y").await.unwrap();
        assert_eq!(db.solved_daily_days(d).await.unwrap(), [d, d.add_days(-2)]);
    }

    #[tokio::test]
    async fn lists_items_and_reorder() {
        let (_dir, db) = fresh("lists").await;
        let a = db.create_list("Warm-up").await.unwrap();
        let b = db.create_list("Hard ones").await.unwrap();
        for p in ["p1", "p2", "p3"] {
            db.add_to_list(a.id, p).await.unwrap();
        }
        db.add_to_list(a.id, "p2").await.unwrap(); // already there
        db.add_to_list(b.id, "p2").await.unwrap();
        assert_eq!(db.list_items(a.id).await.unwrap(), ["p1", "p2", "p3"]);
        assert_eq!(db.lists_with("p2").await.unwrap(), [a.id, b.id]);

        db.reorder_list(a.id, &["p3".into(), "p1".into(), "p2".into()])
            .await
            .unwrap();
        assert_eq!(db.list_items(a.id).await.unwrap(), ["p3", "p1", "p2"]);
        assert!(
            db.reorder_list(a.id, &["p3".into(), "p1".into()])
                .await
                .is_err()
        );
        db.remove_from_list(a.id, "p1").await.unwrap();
        db.add_to_list(a.id, "p4").await.unwrap();
        assert_eq!(db.list_items(a.id).await.unwrap(), ["p3", "p2", "p4"]);

        db.rename_list(b.id, "Tough ones").await.unwrap();
        let names: Vec<String> = db
            .lists()
            .await
            .unwrap()
            .into_iter()
            .map(|l| l.name)
            .collect();
        assert_eq!(names, ["Warm-up", "Tough ones"]);
        db.delete_list(a.id).await.unwrap();
        assert_eq!(db.list_items(a.id).await.unwrap(), Vec::<String>::new());
        assert_eq!(db.lists_with("p2").await.unwrap(), [b.id]);
        assert!(matches!(
            db.rename_list(a.id, "x").await,
            Err(StoreError::NotFound(_))
        ));
        assert!(matches!(
            db.add_to_list(a.id, "x").await,
            Err(StoreError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn activity_and_the_whole_review_queue() {
        use crate::user::tests::report;
        use kb_core::{Language, Verdict};
        let (_d, db) = fresh("activity").await;
        let before = now_ms();
        db.record_submission("a", Language::Python, "x", &report(Verdict::WrongAnswer))
            .await
            .unwrap();
        db.record_submission("a", Language::Rust, "y", &report(Verdict::Accepted))
            .await
            .unwrap();
        let rows = db.activity_since(before).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            (rows[0].problem_id.as_str(), rows[0].accepted),
            ("a", false)
        );
        assert!(rows[1].accepted && rows[1].at >= before);
        assert!(
            db.activity_since(now_ms() + 60_000)
                .await
                .unwrap()
                .is_empty()
        );

        for (id, due) in [("late", "2026-01-09"), ("early", "2026-01-02")] {
            db.put_review(&ReviewRow {
                problem_id: id.into(),
                due_on: due.into(),
                interval_days: 3,
                passes: 0,
                lapsed: false,
            })
            .await
            .unwrap();
        }
        let all = db.review_rows().await.unwrap();
        assert_eq!(
            all.iter()
                .map(|r| r.problem_id.as_str())
                .collect::<Vec<_>>(),
            ["early", "late"]
        );
        assert_eq!(db.reviews_due(day("2026-01-05")).await.unwrap().len(), 1);
    }
}
