//! user.db queries for study plans (expansion M2, ADR 0033). The generator and the
//! rules live in kb-app; this module only reads and writes rows.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::{Result, StoreError, UserDb, now_ms};

/// A study plan (also an export row).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanRow {
    /// UTC milliseconds; the plan's key.
    pub created_at: i64,
    /// The goal as the UI chose it (`{"patterns": [...]}`).
    pub goal: Value,
    pub weeks: u32,
    pub minutes_per_day: u32,
    /// `YYYY-MM-DD`.
    pub start_day: String,
    pub archived: bool,
}

/// One planned item (also an export row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanItemRow {
    pub plan_created_at: i64,
    pub position: u32,
    /// `YYYY-MM-DD`.
    pub day: String,
    /// `problem`, `topic`, `review` or `kata`.
    pub kind: String,
    pub reference: Option<String>,
    pub minutes: u32,
    /// `pending`, `done` or `skipped`.
    pub status: String,
    pub updated_at: i64,
}

fn plan_row(r: &SqliteRow) -> Result<PlanRow> {
    Ok(PlanRow {
        created_at: r.try_get("created_at")?,
        goal: serde_json::from_str(&r.try_get::<String, _>("goal")?)?,
        weeks: u32::try_from(r.try_get::<i64, _>("weeks")?).unwrap_or(0),
        minutes_per_day: u32::try_from(r.try_get::<i64, _>("minutes_per_day")?).unwrap_or(0),
        start_day: r.try_get("start_day")?,
        archived: r.try_get::<i64, _>("archived")? != 0,
    })
}

fn item_row(r: &SqliteRow) -> Result<PlanItemRow> {
    Ok(PlanItemRow {
        plan_created_at: r.try_get("plan_created_at")?,
        position: u32::try_from(r.try_get::<i64, _>("position")?).unwrap_or(0),
        day: r.try_get("day")?,
        kind: r.try_get("kind")?,
        reference: r.try_get("ref")?,
        minutes: u32::try_from(r.try_get::<i64, _>("minutes")?).unwrap_or(0),
        status: r.try_get("status")?,
        updated_at: r.try_get("updated_at")?,
    })
}

pub const PLAN_STATUSES: [&str; 3] = ["pending", "done", "skipped"];

impl UserDb {
    /// Store a new plan with its items, archiving every other plan (one plan is active).
    pub async fn create_plan(&self, plan: &PlanRow, items: &[PlanItemRow]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE study_plans SET archived = 1 WHERE archived = 0")
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO study_plans (created_at, goal, weeks, minutes_per_day, start_day, archived)
             VALUES (?, ?, ?, ?, ?, 0)",
        )
        .bind(plan.created_at)
        .bind(serde_json::to_string(&plan.goal)?)
        .bind(i64::from(plan.weeks))
        .bind(i64::from(plan.minutes_per_day))
        .bind(&plan.start_day)
        .execute(&mut *tx)
        .await?;
        for i in items {
            sqlx::query(
                "INSERT INTO plan_items (plan_created_at, position, day, kind, ref, minutes, status, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(plan.created_at)
            .bind(i64::from(i.position))
            .bind(&i.day)
            .bind(&i.kind)
            .bind(&i.reference)
            .bind(i64::from(i.minutes))
            .bind(&i.status)
            .bind(i.updated_at)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// The active plan, if any.
    pub async fn active_plan(&self) -> Result<Option<PlanRow>> {
        sqlx::query("SELECT * FROM study_plans WHERE archived = 0 ORDER BY created_at DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(plan_row)
            .transpose()
    }

    /// Every plan, newest first.
    pub async fn plans(&self) -> Result<Vec<PlanRow>> {
        sqlx::query("SELECT * FROM study_plans ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(plan_row)
            .collect()
    }

    pub async fn archive_plan(&self, created_at: i64) -> Result<()> {
        sqlx::query("UPDATE study_plans SET archived = 1 WHERE created_at = ?")
            .bind(created_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// A plan's items by day, then position.
    pub async fn plan_items(&self, plan: i64) -> Result<Vec<PlanItemRow>> {
        sqlx::query("SELECT * FROM plan_items WHERE plan_created_at = ? ORDER BY day, position")
            .bind(plan)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(item_row)
            .collect()
    }

    /// Every item of every plan (for export).
    pub async fn all_plan_items(&self) -> Result<Vec<PlanItemRow>> {
        sqlx::query("SELECT * FROM plan_items ORDER BY plan_created_at, position")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(item_row)
            .collect()
    }

    pub async fn set_plan_item_status(&self, plan: i64, position: u32, status: &str) -> Result<()> {
        if !PLAN_STATUSES.contains(&status) {
            return Err(StoreError::Invalid(format!("plan item status {status:?}")));
        }
        let n = sqlx::query(
            "UPDATE plan_items SET status = ?, updated_at = ? WHERE plan_created_at = ? AND position = ?",
        )
        .bind(status)
        .bind(now_ms())
        .bind(plan)
        .bind(i64::from(position))
        .execute(&self.pool)
        .await?
        .rows_affected();
        if n == 0 {
            return Err(StoreError::NotFound(format!("plan item {plan}/{position}")));
        }
        Ok(())
    }

    /// Move items to other days (each `(position, day)`), in one transaction.
    pub async fn move_plan_items(&self, plan: i64, moves: &[(u32, String)]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let now = now_ms();
        for (position, day) in moves {
            sqlx::query(
                "UPDATE plan_items SET day = ?, updated_at = ? WHERE plan_created_at = ? AND position = ?",
            )
            .bind(day)
            .bind(now)
            .bind(plan)
            .bind(i64::from(*position))
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
    use serde_json::json;

    fn plan(at: i64) -> PlanRow {
        PlanRow {
            created_at: at,
            goal: json!({"patterns": ["arrays-hashing"]}),
            weeks: 2,
            minutes_per_day: 30,
            start_day: "2026-11-23".into(),
            archived: false,
        }
    }

    fn item(at: i64, position: u32, day: &str) -> PlanItemRow {
        PlanItemRow {
            plan_created_at: at,
            position,
            day: day.into(),
            kind: "problem".into(),
            reference: Some(format!("p{position}")),
            minutes: 20,
            status: "pending".into(),
            updated_at: at,
        }
    }

    #[tokio::test]
    async fn plans_and_items() {
        let dir = temp_dir("plans");
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        assert_eq!(db.active_plan().await.unwrap(), None);
        db.create_plan(
            &plan(1),
            &[item(1, 0, "2026-11-23"), item(1, 1, "2026-11-24")],
        )
        .await
        .unwrap();
        assert_eq!(db.active_plan().await.unwrap().unwrap().created_at, 1);
        db.set_plan_item_status(1, 0, "done").await.unwrap();
        assert!(db.set_plan_item_status(1, 0, "maybe").await.is_err());
        assert!(db.set_plan_item_status(1, 9, "done").await.is_err());
        db.move_plan_items(1, &[(1, "2026-11-25".into())])
            .await
            .unwrap();
        let items = db.plan_items(1).await.unwrap();
        assert_eq!(items[0].status, "done");
        assert_eq!(items[1].day, "2026-11-25");
        // A new plan archives the old one.
        db.create_plan(&plan(2), &[item(2, 0, "2026-12-01")])
            .await
            .unwrap();
        assert_eq!(db.active_plan().await.unwrap().unwrap().created_at, 2);
        let all = db.plans().await.unwrap();
        assert_eq!(all.len(), 2);
        assert!(all[1].archived);
        assert_eq!(db.all_plan_items().await.unwrap().len(), 3);
        db.close().await;
    }
}
