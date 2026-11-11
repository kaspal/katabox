//! `user.db`: the user's progress. Read-write, WAL mode, numbered migrations applied at
//! open after a backup copy (ADR 0010).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kb_core::{Language, Verdict};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteRow,
    SqliteSynchronous,
};
use sqlx::{Row, SqliteConnection};

use crate::{Result, StoreError, now_ms};

/// Snapshots kept per problem (all languages together); the oldest go first.
pub const SNAPSHOT_CAP: i64 = 200;

/// `user.db.bak-*` files kept after a migration backup, the new one included.
pub const BACKUPS_KEPT: usize = 5;

/// One numbered migration. Versions start at 1 and are contiguous; a released
/// migration is never edited (ADR 0010).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Migration {
    pub version: i64,
    pub sql: &'static str,
}

pub(crate) const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    sql: include_str!("../migrations/0001_init.sql"),
}];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProblemStatus {
    /// Submitted at least once, never Accepted.
    Attempted,
    /// Accepted at least once (never goes back to attempted).
    Solved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusInfo {
    pub status: ProblemStatus,
    pub first_solved_at: Option<i64>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Draft {
    pub code: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotSource {
    Run,
    Submit,
    /// The draft that was replaced by restoring another snapshot.
    Restore,
}

impl SnapshotSource {
    fn code(self) -> &'static str {
        match self {
            SnapshotSource::Run => "run",
            SnapshotSource::Submit => "submit",
            SnapshotSource::Restore => "restore",
        }
    }

    fn parse(s: &str) -> Result<Self> {
        match s {
            "run" => Ok(SnapshotSource::Run),
            "submit" => Ok(SnapshotSource::Submit),
            "restore" => Ok(SnapshotSource::Restore),
            _ => Err(StoreError::Corrupt(format!("snapshot source {s:?}"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Snapshot {
    pub id: i64,
    pub problem_id: String,
    pub language: Language,
    pub code: String,
    pub source: SnapshotSource,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubmissionSummary {
    pub id: i64,
    pub problem_id: String,
    pub language: Language,
    pub verdict: Verdict,
    pub time_us: u64,
    pub memory_kb: u32,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Submission {
    #[serde(flatten)]
    pub summary: SubmissionSummary,
    pub code: String,
    /// The kb-judge Report as recorded.
    pub report: Value,
}

/// Read-write handle to user.db. One connection: the app is the only writer, and a
/// single connection rules out SQLITE_BUSY between our own transactions.
#[derive(Debug, Clone)]
pub struct UserDb {
    pool: SqlitePool,
    backup: Option<PathBuf>,
}

impl UserDb {
    /// Open (creating if needed) and bring the schema up to date.
    pub async fn open(path: &Path) -> Result<UserDb> {
        Self::open_with(path, MIGRATIONS).await
    }

    pub(crate) async fn open_with(path: &Path, migrations: &[Migration]) -> Result<UserDb> {
        let existed = std::fs::metadata(path).is_ok_and(|m| m.len() > 0);
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir)?;
        }
        let opts = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await?;
        match migrate(&pool, path, existed, migrations).await {
            Ok(backup) => Ok(UserDb { pool, backup }),
            Err(e) => {
                pool.close().await;
                Err(e)
            }
        }
    }

    /// The backup written before migrating at this open, if any.
    pub fn backup_path(&self) -> Option<&Path> {
        self.backup.as_deref()
    }

    pub async fn schema_version(&self) -> Result<i64> {
        Ok(sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    // --- Settings -------------------------------------------------------------------

    pub async fn get_setting(&self, key: &str) -> Result<Option<Value>> {
        let v: Option<String> = sqlx::query_scalar("SELECT value FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(v.map(|s| serde_json::from_str(&s)).transpose()?)
    }

    pub async fn set_setting(&self, key: &str, value: &Value) -> Result<()> {
        sqlx::query(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, ?)
             ON CONFLICT (key) DO UPDATE SET value = excluded.value,
                                             updated_at = excluded.updated_at",
        )
        .bind(key)
        .bind(serde_json::to_string(value)?)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Drafts ---------------------------------------------------------------------

    pub async fn save_draft(&self, problem: &str, language: Language, code: &str) -> Result<()> {
        let mut conn = self.pool.acquire().await?;
        put_draft(&mut conn, problem, language, code).await
    }

    pub async fn load_draft(&self, problem: &str, language: Language) -> Result<Option<Draft>> {
        let row = sqlx::query(
            "SELECT code, updated_at FROM drafts WHERE problem_id = ? AND language = ?",
        )
        .bind(problem)
        .bind(language.id())
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| {
            Ok(Draft {
                code: r.try_get("code")?,
                updated_at: r.try_get("updated_at")?,
            })
        })
        .transpose()
    }

    // --- Runs and submissions -------------------------------------------------------

    /// A Run (public tests): snapshot the code. Does not change the problem's status.
    pub async fn record_run(&self, problem: &str, language: Language, code: &str) -> Result<i64> {
        self.snapshot(problem, language, code, SnapshotSource::Run)
            .await
    }

    /// A Submit: store the submission with its full Report, snapshot the code and
    /// update the problem's status. Returns the submission id.
    pub async fn record_submission(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        report: &kb_core::Report,
    ) -> Result<i64> {
        let now = now_ms();
        let mut tx = self.pool.begin().await?;
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO submissions (problem_id, language, code, verdict, time_us,
                                      memory_kb, report, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
        )
        .bind(problem)
        .bind(language.id())
        .bind(code)
        .bind(verdict_code(report.verdict)?)
        .bind(i64::try_from(report.time_us).unwrap_or(i64::MAX))
        .bind(report.memory_kb)
        .bind(serde_json::to_string(report)?)
        .bind(now)
        .fetch_one(&mut *tx)
        .await?;
        insert_snapshot(
            &mut tx,
            problem,
            language,
            code,
            SnapshotSource::Submit,
            now,
        )
        .await?;

        let solved = report.verdict.is_accepted();
        sqlx::query(
            "INSERT INTO problem_status (problem_id, status, first_solved_at, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (problem_id) DO UPDATE SET
                 status = CASE WHEN problem_status.status = 'solved'
                               THEN 'solved' ELSE excluded.status END,
                 first_solved_at = COALESCE(problem_status.first_solved_at,
                                            excluded.first_solved_at),
                 updated_at = excluded.updated_at",
        )
        .bind(problem)
        .bind(if solved { "solved" } else { "attempted" })
        .bind(solved.then_some(now))
        .bind(now)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Submissions for `problem`, newest first.
    pub async fn list_submissions(&self, problem: &str) -> Result<Vec<SubmissionSummary>> {
        sqlx::query(
            "SELECT id, problem_id, language, verdict, time_us, memory_kb, created_at
             FROM submissions WHERE problem_id = ? ORDER BY id DESC",
        )
        .bind(problem)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(submission_summary)
        .collect()
    }

    pub async fn get_submission(&self, id: i64) -> Result<Submission> {
        let r = sqlx::query("SELECT * FROM submissions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| StoreError::NotFound(format!("submission {id}")))?;
        Ok(Submission {
            summary: submission_summary(&r)?,
            code: r.try_get("code")?,
            report: serde_json::from_str(&r.try_get::<String, _>("report")?)?,
        })
    }

    pub async fn problem_status(&self, problem: &str) -> Result<Option<StatusInfo>> {
        let row = sqlx::query(
            "SELECT status, first_solved_at, updated_at FROM problem_status WHERE problem_id = ?",
        )
        .bind(problem)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| {
            Ok(StatusInfo {
                status: parse_status(&r.try_get::<String, _>("status")?)?,
                first_solved_at: r.try_get("first_solved_at")?,
                updated_at: r.try_get("updated_at")?,
            })
        })
        .transpose()
    }

    /// Status of every problem that has one.
    pub async fn statuses(&self) -> Result<HashMap<String, ProblemStatus>> {
        sqlx::query("SELECT problem_id, status FROM problem_status")
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|r| {
                Ok((
                    r.try_get("problem_id")?,
                    parse_status(&r.try_get::<String, _>("status")?)?,
                ))
            })
            .collect()
    }

    // --- Solution history -----------------------------------------------------------

    /// Save `code` in the history. The same code (same problem and language) is not
    /// duplicated: the old entry is removed and the new one goes on top. Only the newest
    /// [`SNAPSHOT_CAP`] per problem are kept. Returns the snapshot id.
    pub async fn snapshot(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        source: SnapshotSource,
    ) -> Result<i64> {
        let mut tx = self.pool.begin().await?;
        let id = insert_snapshot(&mut tx, problem, language, code, source, now_ms()).await?;
        tx.commit().await?;
        Ok(id)
    }

    /// The history of `problem`, newest first.
    pub async fn list_snapshots(&self, problem: &str) -> Result<Vec<Snapshot>> {
        sqlx::query(
            "SELECT id, problem_id, language, code, source, created_at FROM snapshots
             WHERE problem_id = ? ORDER BY id DESC",
        )
        .bind(problem)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(snapshot_row)
        .collect()
    }

    /// Make snapshot `id` the draft for its problem and language. The current draft is
    /// saved as a snapshot first (source `restore`), so nothing is lost. Returns the
    /// restored snapshot.
    pub async fn restore_snapshot(&self, id: i64) -> Result<Snapshot> {
        let mut tx = self.pool.begin().await?;
        let snap = sqlx::query(
            "SELECT id, problem_id, language, code, source, created_at FROM snapshots
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| StoreError::NotFound(format!("snapshot {id}")))?;
        let snap = snapshot_row(&snap)?;
        let current: Option<String> =
            sqlx::query_scalar("SELECT code FROM drafts WHERE problem_id = ? AND language = ?")
                .bind(&snap.problem_id)
                .bind(snap.language.id())
                .fetch_optional(&mut *tx)
                .await?;
        if let Some(cur) = current.filter(|c| *c != snap.code) {
            insert_snapshot(
                &mut tx,
                &snap.problem_id,
                snap.language,
                &cur,
                SnapshotSource::Restore,
                now_ms(),
            )
            .await?;
        }
        put_draft(&mut tx, &snap.problem_id, snap.language, &snap.code).await?;
        tx.commit().await?;
        Ok(snap)
    }

    // --- Notes ----------------------------------------------------------------------

    pub async fn get_note(&self, problem: &str) -> Result<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT body FROM notes WHERE problem_id = ?")
                .bind(problem)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    /// Replace the Markdown note of `problem`; a blank body deletes it.
    pub async fn set_note(&self, problem: &str, body: &str) -> Result<()> {
        if body.trim().is_empty() {
            sqlx::query("DELETE FROM notes WHERE problem_id = ?")
                .bind(problem)
                .execute(&self.pool)
                .await?;
            return Ok(());
        }
        sqlx::query(
            "INSERT INTO notes (problem_id, body, updated_at) VALUES (?, ?, ?)
             ON CONFLICT (problem_id) DO UPDATE SET body = excluded.body,
                                                    updated_at = excluded.updated_at",
        )
        .bind(problem)
        .bind(body)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Hints ----------------------------------------------------------------------

    /// Record that hint `tier` (1-based) was revealed. Idempotent.
    pub async fn unlock_hint(&self, problem: &str, tier: u32) -> Result<()> {
        sqlx::query(
            "INSERT INTO hint_unlocks (problem_id, tier, unlocked_at) VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(problem)
        .bind(tier)
        .bind(now_ms())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn unlocked_hints(&self, problem: &str) -> Result<Vec<u32>> {
        Ok(
            sqlx::query_scalar("SELECT tier FROM hint_unlocks WHERE problem_id = ? ORDER BY tier")
                .bind(problem)
                .fetch_all(&self.pool)
                .await?,
        )
    }
}

/// Apply pending migrations; returns the backup written first, if any.
async fn migrate(
    pool: &SqlitePool,
    path: &Path,
    existed: bool,
    migrations: &[Migration],
) -> Result<Option<PathBuf>> {
    let current: i64 = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(pool)
        .await?;
    let latest = migrations.last().map_or(0, |m| m.version);
    if current > latest {
        return Err(StoreError::NewerSchema {
            found: current,
            supported: latest,
        });
    }
    if current == latest {
        return Ok(None);
    }
    let backup = if existed && current > 0 {
        Some(backup(pool, path, current).await?)
    } else {
        None
    };
    for m in migrations.iter().filter(|m| m.version > current) {
        let mut tx = pool.begin().await?;
        sqlx::raw_sql(m.sql).execute(&mut *tx).await?;
        sqlx::raw_sql(&format!("PRAGMA user_version = {}", m.version))
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
    }
    Ok(backup)
}

/// `VACUUM INTO <path>.bak-v<version>-<UTC time>`: a consistent copy even in WAL mode.
async fn backup(pool: &SqlitePool, path: &Path, version: i64) -> Result<PathBuf> {
    let stamp: String = sqlx::query_scalar("SELECT strftime('%Y%m%dT%H%M%SZ', 'now')")
        .fetch_one(pool)
        .await?;
    let base = format!("{}.bak-v{version}-{stamp}", path.display());
    let mut target = PathBuf::from(&base);
    let mut n = 1;
    while target.exists() {
        target = PathBuf::from(format!("{base}-{n}"));
        n += 1;
    }
    let target_str = target
        .to_str()
        .ok_or_else(|| StoreError::Corrupt(format!("non-UTF-8 path {}", target.display())))?;
    sqlx::query("VACUUM INTO ?")
        .bind(target_str)
        .execute(pool)
        .await?;
    prune_backups(path, &target);
    Ok(target)
}

/// Keep the newest [`BACKUPS_KEPT`] backups of `db` (by the UTC time in their names),
/// always including `just_written`. Best effort: a file that cannot be deleted stays,
/// and files that do not follow the backup naming pattern are never touched.
fn prune_backups(db: &Path, just_written: &Path) {
    let (Some(dir), Some(name)) = (db.parent(), db.file_name().and_then(|n| n.to_str())) else {
        return;
    };
    let dir = if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let prefix = format!("{name}.bak-v");
    let mut older: Vec<((String, u32), PathBuf)> = entries
        .flatten()
        .filter_map(|e| {
            let file = e.file_name().into_string().ok()?;
            let key = backup_key(file.strip_prefix(&prefix)?)?;
            let p = e.path();
            (e.file_type().ok()?.is_file() && p.file_name() != just_written.file_name())
                .then_some((key, p))
        })
        .collect();
    older.sort_by(|a, b| b.0.cmp(&a.0)); // newest first
    for (_, p) in older.into_iter().skip(BACKUPS_KEPT.saturating_sub(1)) {
        let _ = std::fs::remove_file(p);
    }
}

/// Sort key of a backup name after `<db>.bak-v`: `<version>-<YYYYMMDDTHHMMSSZ>[-<n>]`
/// gives `(stamp, n)`. `None` for anything else.
fn backup_key(rest: &str) -> Option<(String, u32)> {
    let (version, rest) = rest.split_once('-')?;
    version.parse::<i64>().ok()?;
    let (stamp, n) = match rest.split_once('-') {
        Some((stamp, n)) => (stamp, n.parse().ok()?),
        None => (rest, 0),
    };
    let valid = stamp.len() == 16
        && stamp.as_bytes()[8] == b'T'
        && stamp.ends_with('Z')
        && stamp[..8]
            .bytes()
            .chain(stamp[9..15].bytes())
            .all(|b| b.is_ascii_digit());
    valid.then(|| (stamp.to_string(), n))
}

async fn put_draft(
    conn: &mut SqliteConnection,
    problem: &str,
    language: Language,
    code: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO drafts (problem_id, language, code, updated_at) VALUES (?, ?, ?, ?)
         ON CONFLICT (problem_id, language) DO UPDATE SET code = excluded.code,
                                                          updated_at = excluded.updated_at",
    )
    .bind(problem)
    .bind(language.id())
    .bind(code)
    .bind(now_ms())
    .execute(&mut *conn)
    .await?;
    Ok(())
}

async fn insert_snapshot(
    conn: &mut SqliteConnection,
    problem: &str,
    language: Language,
    code: &str,
    source: SnapshotSource,
    now: i64,
) -> Result<i64> {
    let hash = code_hash(code);
    sqlx::query("DELETE FROM snapshots WHERE problem_id = ? AND language = ? AND code_hash = ?")
        .bind(problem)
        .bind(language.id())
        .bind(&hash)
        .execute(&mut *conn)
        .await?;
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO snapshots (problem_id, language, code, code_hash, source, created_at)
         VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
    )
    .bind(problem)
    .bind(language.id())
    .bind(code)
    .bind(&hash)
    .bind(source.code())
    .bind(now)
    .fetch_one(&mut *conn)
    .await?;
    sqlx::query(
        "DELETE FROM snapshots WHERE problem_id = ?1 AND id NOT IN
             (SELECT id FROM snapshots WHERE problem_id = ?1 ORDER BY id DESC LIMIT ?2)",
    )
    .bind(problem)
    .bind(SNAPSHOT_CAP)
    .execute(&mut *conn)
    .await?;
    Ok(id)
}

fn code_hash(code: &str) -> String {
    Sha256::digest(code.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn snapshot_row(r: &SqliteRow) -> Result<Snapshot> {
    Ok(Snapshot {
        id: r.try_get("id")?,
        problem_id: r.try_get("problem_id")?,
        language: parse_language(&r.try_get::<String, _>("language")?)?,
        code: r.try_get("code")?,
        source: SnapshotSource::parse(&r.try_get::<String, _>("source")?)?,
        created_at: r.try_get("created_at")?,
    })
}

fn submission_summary(r: &SqliteRow) -> Result<SubmissionSummary> {
    let time_us: i64 = r.try_get("time_us")?;
    Ok(SubmissionSummary {
        id: r.try_get("id")?,
        problem_id: r.try_get("problem_id")?,
        language: parse_language(&r.try_get::<String, _>("language")?)?,
        verdict: parse_verdict(&r.try_get::<String, _>("verdict")?)?,
        time_us: u64::try_from(time_us).unwrap_or(0),
        memory_kb: r.try_get("memory_kb")?,
        created_at: r.try_get("created_at")?,
    })
}

fn parse_language(s: &str) -> Result<Language> {
    s.parse()
        .map_err(|_| StoreError::Corrupt(format!("language {s:?}")))
}

/// The verdict's serde name (`accepted`, `wrong_answer`, ...), as in the Report JSON.
fn verdict_code(v: Verdict) -> Result<String> {
    match serde_json::to_value(v)? {
        Value::String(s) => Ok(s),
        other => Err(StoreError::Corrupt(format!("verdict {other}"))),
    }
}

fn parse_verdict(s: &str) -> Result<Verdict> {
    serde_json::from_value(Value::String(s.into()))
        .map_err(|_| StoreError::Corrupt(format!("verdict {s:?}")))
}

fn parse_status(s: &str) -> Result<ProblemStatus> {
    match s {
        "attempted" => Ok(ProblemStatus::Attempted),
        "solved" => Ok(ProblemStatus::Solved),
        _ => Err(StoreError::Corrupt(format!("status {s:?}"))),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::testutil::temp_dir;
    use kb_core::{Failure, Report, TestOutcome};
    use kb_core::{Message, MessageCode, TestKind, TestResult};
    use serde_json::json;

    const TABLES: [&str; 7] = [
        "settings",
        "drafts",
        "submissions",
        "problem_status",
        "snapshots",
        "notes",
        "hint_unlocks",
    ];

    async fn fresh(name: &str) -> (PathBuf, UserDb) {
        let dir = temp_dir(name);
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        (dir, db)
    }

    fn backups(dir: &Path) -> Vec<PathBuf> {
        std::fs::read_dir(dir)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.to_string_lossy().contains(".bak-"))
            .collect()
    }

    #[test]
    fn migration_versions_are_contiguous_from_one() {
        for (i, m) in MIGRATIONS.iter().enumerate() {
            assert_eq!(m.version, i64::try_from(i + 1).unwrap());
        }
    }

    #[tokio::test]
    async fn migrates_from_empty_without_backup() {
        let (dir, db) = fresh("empty").await;
        assert_eq!(db.schema_version().await.unwrap(), MIGRATIONS.len() as i64);
        assert!(db.backup_path().is_none());
        for t in TABLES {
            let n: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM {t}"))
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(n, 0, "{t}");
        }
        let mode: String = sqlx::query_scalar("PRAGMA journal_mode")
            .fetch_one(&db.pool)
            .await
            .unwrap();
        assert_eq!(mode, "wal");
        db.close().await;
        // Reopening at the latest version migrates nothing and backs up nothing.
        let db = UserDb::open(&dir.join("user.db")).await.unwrap();
        assert!(db.backup_path().is_none());
        db.close().await;
        assert!(backups(&dir).is_empty());
    }

    #[tokio::test]
    async fn backs_up_before_migrating() {
        let dir = temp_dir("backup");
        let path = dir.join("user.db");
        let v1 = &MIGRATIONS[..1];
        let db = UserDb::open_with(&path, v1).await.unwrap();
        db.set_note("two-sum", "hash map").await.unwrap();
        db.close().await;

        let v2 = [
            v1[0],
            Migration {
                version: 2,
                sql: "CREATE TABLE extra (x INTEGER); INSERT INTO extra VALUES (7);",
            },
        ];
        let db = UserDb::open_with(&path, &v2).await.unwrap();
        assert_eq!(db.schema_version().await.unwrap(), 2);
        assert_eq!(
            db.get_note("two-sum").await.unwrap().as_deref(),
            Some("hash map")
        );
        let bak = db.backup_path().unwrap().to_path_buf();
        db.close().await;
        assert_eq!(backups(&dir), std::slice::from_ref(&bak));
        assert!(
            bak.to_string_lossy().contains("user.db.bak-v1-"),
            "{}",
            bak.display()
        );

        // The backup is the version-1 database with its data.
        let old = UserDb::open_with(&bak, v1).await.unwrap();
        assert_eq!(old.schema_version().await.unwrap(), 1);
        assert!(old.backup_path().is_none());
        assert_eq!(
            old.get_note("two-sum").await.unwrap().as_deref(),
            Some("hash map")
        );
        old.close().await;

        // A database newer than the binary is refused, never downgraded.
        assert!(matches!(
            UserDb::open_with(&path, v1).await,
            Err(StoreError::NewerSchema {
                found: 2,
                supported: 1
            })
        ));
    }

    #[test]
    fn backup_names_parse() {
        assert_eq!(
            backup_key("1-20261111T093000Z"),
            Some(("20261111T093000Z".into(), 0))
        );
        assert_eq!(
            backup_key("12-20261111T093000Z-2"),
            Some(("20261111T093000Z".into(), 2))
        );
        for bad in [
            "x-20261111T093000Z",
            "1-2026",
            "1-20261111T093000Z-x",
            "1-20261111X093000Z",
        ] {
            assert_eq!(backup_key(bad), None, "{bad}");
        }
    }

    #[tokio::test]
    async fn keeps_the_newest_backups() {
        let dir = temp_dir("prune");
        let path = dir.join("user.db");
        let v1 = &MIGRATIONS[..1];
        UserDb::open_with(&path, v1).await.unwrap().close().await;

        // Six older backups, one dated in the future (clock skew), and unrelated files.
        let fake = |n: &str| {
            let p = dir.join(n);
            std::fs::write(&p, b"old").unwrap();
            p
        };
        let old: Vec<PathBuf> = (1..=6)
            .map(|d| fake(&format!("user.db.bak-v1-2020010{d}T000000Z")))
            .collect();
        let future = fake("user.db.bak-v1-20991231T000000Z");
        let notes = fake("user.db.bak-notes.txt");
        let other = fake("other.db.bak-v1-20200101T000000Z");

        let v2 = [
            v1[0],
            Migration {
                version: 2,
                sql: "CREATE TABLE extra (x INTEGER);",
            },
        ];
        let db = UserDb::open_with(&path, &v2).await.unwrap();
        let new = db.backup_path().unwrap().to_path_buf();
        db.close().await;

        let mut kept = backups(&dir);
        kept.retain(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("user.db.bak-v")
        });
        kept.sort();
        let mut want = vec![new, future, old[5].clone(), old[4].clone(), old[3].clone()];
        want.sort();
        assert_eq!(kept, want);
        assert_eq!(kept.len(), BACKUPS_KEPT);
        assert!(notes.exists() && other.exists());
    }

    #[tokio::test]
    async fn settings_drafts_notes_hints() {
        let (_dir, db) = fresh("misc").await;
        assert_eq!(db.get_setting("theme").await.unwrap(), None);
        db.set_setting("theme", &json!("dark")).await.unwrap();
        db.set_setting("theme", &json!({"mode": "light"}))
            .await
            .unwrap();
        assert_eq!(
            db.get_setting("theme").await.unwrap(),
            Some(json!({"mode": "light"}))
        );

        assert_eq!(db.load_draft("p", Language::Rust).await.unwrap(), None);
        db.save_draft("p", Language::Rust, "fn a() {}")
            .await
            .unwrap();
        db.save_draft("p", Language::Rust, "fn b() {}")
            .await
            .unwrap();
        db.save_draft("p", Language::Python, "pass").await.unwrap();
        let d = db.load_draft("p", Language::Rust).await.unwrap().unwrap();
        assert_eq!(d.code, "fn b() {}");
        assert!(d.updated_at > 1_700_000_000_000);

        db.set_note("p", "# idea\nuse a stack").await.unwrap();
        db.set_note("p", "# idea\nuse a deque").await.unwrap();
        assert_eq!(
            db.get_note("p").await.unwrap().as_deref(),
            Some("# idea\nuse a deque")
        );
        db.set_note("p", "  \n").await.unwrap();
        assert_eq!(db.get_note("p").await.unwrap(), None);

        db.unlock_hint("p", 2).await.unwrap();
        db.unlock_hint("p", 1).await.unwrap();
        db.unlock_hint("p", 2).await.unwrap();
        assert_eq!(db.unlocked_hints("p").await.unwrap(), [1, 2]);
    }

    #[tokio::test]
    async fn snapshots_dedupe_cap_and_restore() {
        let (_dir, db) = fresh("snap").await;
        let a = db.record_run("p", Language::Python, "a").await.unwrap();
        db.record_run("p", Language::Python, "b").await.unwrap();
        let a2 = db.record_run("p", Language::Python, "a").await.unwrap();
        assert!(a2 > a);
        let codes = |s: Vec<Snapshot>| s.into_iter().map(|s| s.code).collect::<Vec<_>>();
        assert_eq!(codes(db.list_snapshots("p").await.unwrap()), ["a", "b"]);
        // Same code in another language is a different snapshot.
        db.record_run("p", Language::Rust, "a").await.unwrap();
        assert_eq!(db.list_snapshots("p").await.unwrap().len(), 3);
        // Other problems are separate.
        db.record_run("q", Language::Python, "a").await.unwrap();
        assert_eq!(db.list_snapshots("q").await.unwrap().len(), 1);

        // Cap: 205 distinct codes leave the newest 200.
        for i in 0..205 {
            db.record_run("p", Language::Python, &format!("v{i}"))
                .await
                .unwrap();
        }
        let all = db.list_snapshots("p").await.unwrap();
        assert_eq!(all.len(), 200);
        assert_eq!(all[0].code, "v204");
        assert_eq!(all[199].code, "v5");
        assert_eq!(db.list_snapshots("q").await.unwrap().len(), 1);

        // Restore: the current draft becomes a snapshot, then the draft is replaced.
        db.save_draft("p", Language::Python, "work in progress")
            .await
            .unwrap();
        let target = all.iter().find(|s| s.code == "v10").unwrap().id;
        let restored = db.restore_snapshot(target).await.unwrap();
        assert_eq!(restored.code, "v10");
        let d = db.load_draft("p", Language::Python).await.unwrap().unwrap();
        assert_eq!(d.code, "v10");
        let after = db.list_snapshots("p").await.unwrap();
        assert_eq!(after.len(), 200);
        assert_eq!(after[0].code, "work in progress");
        assert_eq!(after[0].source, SnapshotSource::Restore);
        assert!(after.iter().any(|s| s.id == target)); // the restored one stays

        // Restoring when the draft already equals the snapshot adds nothing.
        db.restore_snapshot(target).await.unwrap();
        assert_eq!(
            db.list_snapshots("p").await.unwrap()[0].code,
            "work in progress"
        );
        assert!(matches!(
            db.restore_snapshot(-1).await,
            Err(StoreError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn history_of_200_snapshots_opens_under_100ms() {
        let dir = temp_dir("perf");
        let path = dir.join("user.db");
        let db = UserDb::open(&path).await.unwrap();
        let body = "x = 1  # padding\n".repeat(120); // ~2 KB
        for i in 0..200 {
            db.record_run("big", Language::Python, &format!("# {i}\n{body}"))
                .await
                .unwrap();
        }
        db.close().await;

        let db = UserDb::open(&path).await.unwrap();
        let start = std::time::Instant::now();
        let history = db.list_snapshots("big").await.unwrap();
        let took = start.elapsed();
        eprintln!("list_snapshots(200): {took:?}");
        assert_eq!(history.len(), 200);
        assert!(took < Duration::from_millis(100), "took {took:?}");
    }

    pub(crate) fn report(verdict: Verdict) -> Report {
        let ok = verdict.is_accepted();
        let outcome = |index, v: Verdict| TestOutcome {
            result: TestResult {
                index,
                kind: TestKind::Public,
                verdict: v,
                time_us: 120,
                memory_kb: 9000,
                message: (!v.is_accepted()).then(|| {
                    Message::new(MessageCode::OutputMismatch, "expected [0,1], got [1,2]")
                }),
            },
            actual: Some(json!([0, 1])),
            stdout: String::new(),
        };
        Report {
            verdict,
            tests: vec![outcome(1, Verdict::Accepted), outcome(2, verdict)],
            total_tests: 2,
            first_failure: (!ok).then(|| Failure {
                kind: TestKind::Public,
                index: 2,
                verdict,
                input: json!({"nums": [1, 2]}).as_object().unwrap().clone(),
                expected: json!([0, 1]),
                actual: Some(json!([1, 2])),
                message: Some(Message::new(MessageCode::OutputMismatch, "x")),
                stdout: String::new(),
            }),
            compile_output: None,
            time_us: 240,
            memory_kb: 9000,
            compile: None,
        }
    }

    #[tokio::test]
    async fn submissions_and_status() {
        let (_dir, db) = fresh("subs").await;
        assert_eq!(db.problem_status("p").await.unwrap(), None);

        let wa = report(Verdict::WrongAnswer);
        let id1 = db
            .record_submission("p", Language::Php, "<?php wrong", &wa)
            .await
            .unwrap();
        let st = db.problem_status("p").await.unwrap().unwrap();
        assert_eq!(st.status, ProblemStatus::Attempted);
        assert_eq!(st.first_solved_at, None);

        let id2 = db
            .record_submission(
                "p",
                Language::Php,
                "<?php right",
                &report(Verdict::Accepted),
            )
            .await
            .unwrap();
        let solved = db.problem_status("p").await.unwrap().unwrap();
        assert_eq!(solved.status, ProblemStatus::Solved);
        let first = solved.first_solved_at.unwrap();

        // A later failure never downgrades, and first_solved_at stays.
        db.record_submission("p", Language::Php, "<?php wrong", &wa)
            .await
            .unwrap();
        let still = db.problem_status("p").await.unwrap().unwrap();
        assert_eq!(still.status, ProblemStatus::Solved);
        assert_eq!(still.first_solved_at, Some(first));
        // Runs never touch the status.
        db.record_run("r", Language::Php, "<?php").await.unwrap();
        assert_eq!(db.problem_status("r").await.unwrap(), None);
        assert_eq!(
            db.statuses().await.unwrap().get("p"),
            Some(&ProblemStatus::Solved)
        );

        let list = db.list_submissions("p").await.unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[1].id, id2);
        assert_eq!(list[2].id, id1);
        assert_eq!(list[2].verdict, Verdict::WrongAnswer);
        assert_eq!(list[2].time_us, 240);
        assert_eq!(list[2].memory_kb, 9000);

        let full = db.get_submission(id1).await.unwrap();
        assert_eq!(full.code, "<?php wrong");
        assert_eq!(full.report, serde_json::to_value(&wa).unwrap());
        assert_eq!(full.report["verdict"], "wrong_answer");

        // Submissions are snapshotted (deduped: "wrong" twice is one entry).
        let snaps = db.list_snapshots("p").await.unwrap();
        assert_eq!(snaps.len(), 2);
        assert_eq!(snaps[0].source, SnapshotSource::Submit);
        assert_eq!(snaps[0].code, "<?php wrong");
    }
}
