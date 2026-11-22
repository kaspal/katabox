//! The stress lab's brute-force solutions (expansion M8, user.db migration 0010).

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::Result;
use crate::user::UserDb;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StressBruteRow {
    pub problem_id: String,
    /// The language id (`python`, `typescript`, …).
    pub language: String,
    pub code: String,
    pub updated_at: i64,
}

fn brute_row(r: &SqliteRow) -> Result<StressBruteRow> {
    Ok(StressBruteRow {
        problem_id: r.try_get("problem_id")?,
        language: r.try_get("language")?,
        code: r.try_get("code")?,
        updated_at: r.try_get("updated_at")?,
    })
}

impl UserDb {
    pub async fn stress_brutes(&self) -> Result<Vec<StressBruteRow>> {
        sqlx::query("SELECT * FROM stress_brutes ORDER BY problem_id, language")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(brute_row)
            .collect()
    }

    pub async fn stress_brute(
        &self,
        problem: &str,
        language: &str,
    ) -> Result<Option<StressBruteRow>> {
        sqlx::query("SELECT * FROM stress_brutes WHERE problem_id = ? AND language = ?")
            .bind(problem)
            .bind(language)
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(brute_row)
            .transpose()
    }

    pub async fn put_stress_brute(&self, b: &StressBruteRow) -> Result<()> {
        sqlx::query(
            "INSERT INTO stress_brutes (problem_id, language, code, updated_at) VALUES (?, ?, ?, ?)
             ON CONFLICT (problem_id, language) DO UPDATE SET code = excluded.code,
               updated_at = excluded.updated_at",
        )
        .bind(&b.problem_id)
        .bind(&b.language)
        .bind(&b.code)
        .bind(b.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::tests::temp_dir;

    #[tokio::test]
    async fn a_brute_is_kept_per_problem_and_language() {
        let db = UserDb::open(&temp_dir("stress-brutes").join("user.db"))
            .await
            .unwrap();
        let b = StressBruteRow {
            problem_id: "two-sum".into(),
            language: "python".into(),
            code: "def two_sum(nums, target):\n    return []\n".into(),
            updated_at: 1,
        };
        db.put_stress_brute(&b).await.unwrap();
        let newer = StressBruteRow {
            code: "# v2".into(),
            updated_at: 2,
            ..b.clone()
        };
        db.put_stress_brute(&newer).await.unwrap();
        assert_eq!(
            db.stress_brute("two-sum", "python").await.unwrap(),
            Some(newer.clone())
        );
        assert_eq!(db.stress_brute("two-sum", "rust").await.unwrap(), None);
        assert_eq!(db.stress_brutes().await.unwrap(), vec![newer]);
    }
}
