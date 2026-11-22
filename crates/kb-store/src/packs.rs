//! Installed problem packs (expansion M11, user.db migration 0012). The pack files live in
//! the app's local data folder; this table records which are installed. It is not part of
//! an export: packs are files on this machine.

use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::Result;
use crate::user::UserDb;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PackRow {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub licence: String,
    pub sha256: String,
    pub problems: u32,
    pub installed_at: i64,
    pub removed_at: Option<i64>,
}

fn pack_row(r: &SqliteRow) -> Result<PackRow> {
    Ok(PackRow {
        id: r.try_get("id")?,
        name: r.try_get("name")?,
        author: r.try_get("author")?,
        version: r.try_get("version")?,
        licence: r.try_get("licence")?,
        sha256: r.try_get("sha256")?,
        problems: r.try_get::<i64, _>("problems")? as u32,
        installed_at: r.try_get("installed_at")?,
        removed_at: r.try_get("removed_at")?,
    })
}

impl UserDb {
    /// Every pack ever installed (removed ones too), by id.
    pub async fn packs(&self) -> Result<Vec<PackRow>> {
        sqlx::query("SELECT * FROM packs ORDER BY id")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(pack_row)
            .collect()
    }

    /// Install or update: the row is replaced and `removed_at` cleared.
    pub async fn put_pack(&self, p: &PackRow) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO packs
               (id, name, author, version, licence, sha256, problems, installed_at, removed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL)",
        )
        .bind(&p.id)
        .bind(&p.name)
        .bind(&p.author)
        .bind(&p.version)
        .bind(&p.licence)
        .bind(&p.sha256)
        .bind(p.problems)
        .bind(p.installed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_pack(&self, id: &str, now_ms: i64) -> Result<bool> {
        Ok(
            sqlx::query("UPDATE packs SET removed_at = ? WHERE id = ? AND removed_at IS NULL")
                .bind(now_ms)
                .bind(id)
                .execute(&self.pool)
                .await?
                .rows_affected()
                == 1,
        )
    }
}
