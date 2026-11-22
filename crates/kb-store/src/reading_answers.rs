//! Answers to reading exercises (expansion M7): attempts per item and when it was first
//! answered correctly (user.db migration 0009).

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::Result;
use crate::user::UserDb;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingAnswerRow {
    /// `<set>/<n>`.
    pub item_id: String,
    pub attempts: u32,
    pub correct_at: Option<i64>,
    pub updated_at: i64,
}

fn answer_row(r: &SqliteRow) -> Result<ReadingAnswerRow> {
    Ok(ReadingAnswerRow {
        item_id: r.try_get("item_id")?,
        attempts: r.try_get::<i64, _>("attempts")? as u32,
        correct_at: r.try_get("correct_at")?,
        updated_at: r.try_get("updated_at")?,
    })
}

impl UserDb {
    /// Every answered item, by id.
    pub async fn reading_answers(&self) -> Result<Vec<ReadingAnswerRow>> {
        sqlx::query("SELECT * FROM reading_answers ORDER BY item_id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(answer_row)
            .collect()
    }

    /// Count one attempt at `item`; the first correct one keeps its time.
    pub async fn record_reading_answer(
        &self,
        item: &str,
        correct: bool,
        now_ms: i64,
    ) -> Result<ReadingAnswerRow> {
        let correct_at = correct.then_some(now_ms);
        let r = sqlx::query(
            "INSERT INTO reading_answers (item_id, attempts, correct_at, updated_at)
             VALUES (?, 1, ?, ?)
             ON CONFLICT (item_id) DO UPDATE SET attempts = attempts + 1,
               correct_at = COALESCE(reading_answers.correct_at, excluded.correct_at),
               updated_at = excluded.updated_at
             RETURNING *",
        )
        .bind(item)
        .bind(correct_at)
        .bind(now_ms)
        .fetch_one(&self.pool)
        .await?;
        answer_row(&r)
    }
}

#[cfg(test)]
mod tests {
    use crate::content::tests::temp_dir;
    use crate::user::UserDb;

    #[tokio::test]
    async fn attempts_count_and_the_first_correct_time_stays() {
        let dir = temp_dir("reading-answers");
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        let a = db
            .record_reading_answer("print-basics/1", false, 10)
            .await
            .unwrap();
        assert_eq!((a.attempts, a.correct_at), (1, None));
        let b = db
            .record_reading_answer("print-basics/1", true, 20)
            .await
            .unwrap();
        assert_eq!((b.attempts, b.correct_at), (2, Some(20)));
        let c = db
            .record_reading_answer("print-basics/1", true, 30)
            .await
            .unwrap();
        assert_eq!((c.attempts, c.correct_at, c.updated_at), (3, Some(20), 30));
        assert_eq!(db.reading_answers().await.unwrap(), vec![c]);
    }
}
