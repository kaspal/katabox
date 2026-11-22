//! Automatic backups (expansion M10, ADR 0037): the ordinary export, written to a folder
//! the user picked, every few days and optionally on exit. Pruning deletes only files
//! whose names match exactly `katabox-backup-YYYYMMDD-HHMMSS.kbx.json`, oldest first,
//! beyond the number to keep; anything else in the folder is never touched. The first
//! backup to a folder writes a README that says so (review A4). Every `backup.*` setting
//! stays on this machine (exports carry only allow-listed settings).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{App, AppError, AppErrorCode, Result};

pub const FOLDER: &str = "backup.folder";
pub const EVERY_DAYS: &str = "backup.every_days";
pub const ON_EXIT: &str = "backup.on_exit";
pub const KEEP: &str = "backup.keep";
pub const LAST: &str = "backup.last";
pub const ERROR: &str = "backup.error";
pub const README: &str = "README-katabox-backups.txt";
pub const DEFAULT_EVERY_DAYS: u32 = 7;
pub const DEFAULT_KEEP: u32 = 5;
const DAY_MS: i64 = 86_400_000;
/// The longest the exit backup may take.
pub const EXIT_CAP: std::time::Duration = std::time::Duration::from_secs(5);

const README_TEXT: &str = "This folder holds KataBox backups.\r\n\
\r\n\
KataBox writes a file named katabox-backup-YYYYMMDD-HHMMSS.kbx.json here (the date and\r\n\
time in UTC), the same file as Settings > Data > Export. To restore one, use\r\n\
Settings > Data > Import.\r\n\
\r\n\
KataBox deletes only files whose names match katabox-backup-YYYYMMDD-HHMMSS.kbx.json\r\n\
exactly, oldest first, beyond the number of backups to keep (Settings > Data). It never\r\n\
deletes or changes anything else in this folder, including this README.\r\n";

/// The file name of a backup taken at `now_ms` (UTC).
pub fn file_name(now_ms: i64) -> String {
    let days = now_ms.div_euclid(DAY_MS);
    let secs = now_ms.rem_euclid(DAY_MS) / 1000;
    let day = kb_core::Day::from_days(days).to_string().replace('-', "");
    format!(
        "katabox-backup-{day}-{:02}{:02}{:02}.kbx.json",
        secs / 3600,
        secs / 60 % 60,
        secs % 60
    )
}

/// Exactly `katabox-backup-` + 8 digits + `-` + 6 digits + `.kbx.json`.
pub fn is_backup_name(name: &str) -> bool {
    let Some(rest) = name
        .strip_prefix("katabox-backup-")
        .and_then(|r| r.strip_suffix(".kbx.json"))
    else {
        return false;
    };
    let b = rest.as_bytes();
    b.len() == 15
        && b[8] == b'-'
        && b[..8].iter().all(u8::is_ascii_digit)
        && b[9..].iter().all(u8::is_ascii_digit)
}

/// The backups to delete so that `keep` remain: the oldest ones (names sort by time).
pub fn to_prune(names: &[String], keep: u32) -> Vec<String> {
    let mut ours: Vec<&String> = names.iter().filter(|n| is_backup_name(n)).collect();
    ours.sort();
    let extra = ours.len().saturating_sub(keep.max(1) as usize);
    ours.into_iter().take(extra).cloned().collect()
}

/// A backup is due when there was none, or the last is `every_days` old.
pub fn due(now_ms: i64, last: Option<i64>, every_days: u32) -> bool {
    last.is_none_or(|l| now_ms - l >= i64::from(every_days.max(1)) * DAY_MS)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BackupStatus {
    pub folder: Option<String>,
    pub every_days: u32,
    pub keep: u32,
    pub on_exit: bool,
    /// UTC ms of the last backup.
    pub last: Option<i64>,
    /// UTC ms when the next one is due (with a folder).
    pub next_due: Option<i64>,
    /// Why the last attempt failed, if it did.
    pub error: Option<String>,
}

fn as_u32(v: Option<Value>, default: u32) -> u32 {
    v.and_then(|v| v.as_u64())
        .and_then(|n| u32::try_from(n).ok())
        .unwrap_or(default)
}

impl App {
    pub async fn backup_status(&self) -> Result<BackupStatus> {
        let u = &self.store.user;
        let folder = u
            .get_setting(FOLDER)
            .await?
            .and_then(|v| v.as_str().map(str::to_string));
        let every_days = as_u32(u.get_setting(EVERY_DAYS).await?, DEFAULT_EVERY_DAYS).clamp(1, 90);
        let keep = as_u32(u.get_setting(KEEP).await?, DEFAULT_KEEP).clamp(1, 100);
        let last = u.get_setting(LAST).await?.and_then(|v| v.as_i64());
        Ok(BackupStatus {
            next_due: folder
                .as_ref()
                .map(|_| last.map_or(self.now_ms(), |l| l + i64::from(every_days) * DAY_MS)),
            folder,
            every_days,
            keep,
            on_exit: u
                .get_setting(ON_EXIT)
                .await?
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            last,
            error: u
                .get_setting(ERROR)
                .await?
                .and_then(|v| v.as_str().map(str::to_string)),
        })
    }

    /// The folder (picked by a Rust dialog in the shell; `None` turns backups off).
    pub async fn set_backup_folder(&self, folder: Option<&Path>) -> Result<BackupStatus> {
        let v = match folder {
            Some(f) if f.is_dir() => json!(f.to_string_lossy()),
            Some(_) => return Err(AppError::new(AppErrorCode::InvalidInput, "not a folder")),
            None => Value::Null,
        };
        self.store.user.set_setting(FOLDER, &v).await?;
        self.backup_status().await
    }

    pub async fn set_backup_options(
        &self,
        every_days: u32,
        keep: u32,
        on_exit: bool,
    ) -> Result<BackupStatus> {
        if !(1..=90).contains(&every_days) || !(1..=100).contains(&keep) {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "every 1-90 days, keep 1-100",
            ));
        }
        let u = &self.store.user;
        u.set_setting(EVERY_DAYS, &json!(every_days)).await?;
        u.set_setting(KEEP, &json!(keep)).await?;
        u.set_setting(ON_EXIT, &json!(on_exit)).await?;
        self.backup_status().await
    }

    /// Back up now to the chosen folder; on failure the reason is kept for Settings.
    pub async fn backup_now(&self) -> Result<BackupStatus> {
        let status = self.backup_status().await?;
        let Some(folder) = status.folder.clone() else {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "choose a backup folder first",
            ));
        };
        match self.write_backup(Path::new(&folder), status.keep).await {
            Ok(()) => {
                self.store
                    .user
                    .set_setting(LAST, &json!(self.now_ms()))
                    .await?;
                self.store.user.set_setting(ERROR, &Value::Null).await?;
            }
            Err(e) => {
                self.store
                    .user
                    .set_setting(ERROR, &json!(e.message))
                    .await?;
                return Err(e);
            }
        }
        self.backup_status().await
    }

    /// On start (`on_exit` false) when due, or on exit when that option is on. Never fails:
    /// a problem is kept for Settings → Data. `true` when a backup was written.
    pub async fn backup_if_due(&self, on_exit: bool) -> bool {
        let Ok(s) = self.backup_status().await else {
            return false;
        };
        if s.folder.is_none() {
            return false;
        }
        let wanted = if on_exit {
            s.on_exit
        } else {
            due(self.now_ms(), s.last, s.every_days)
        };
        wanted && self.backup_now().await.is_ok()
    }

    /// The exit backup, capped at [`EXIT_CAP`] so closing the app never hangs.
    pub async fn backup_on_exit(&self) -> bool {
        tokio::time::timeout(EXIT_CAP, self.backup_if_due(true))
            .await
            .unwrap_or(false)
    }

    async fn write_backup(&self, folder: &Path, keep: u32) -> Result<()> {
        let io = |e: std::io::Error| {
            AppError::new(AppErrorCode::Internal, format!("backup folder: {e}"))
        };
        if !folder.is_dir() {
            return Err(AppError::new(
                AppErrorCode::NotFound,
                "the backup folder is missing",
            ));
        }
        let readme = folder.join(README);
        if !readme.exists() {
            std::fs::write(&readme, README_TEXT).map_err(io)?;
        }
        let path: PathBuf = folder.join(file_name(self.now_ms()));
        self.export_to(&path).await?;
        let names: Vec<String> = std::fs::read_dir(folder)
            .map_err(io)?
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        for old in to_prune(&names, keep) {
            // Only names that matched exactly; a file that vanished meanwhile is fine.
            let _ = std::fs::remove_file(folder.join(old));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_utc_and_matched_exactly() {
        // 2026-11-22 08:05:09 UTC.
        let ms = 1_795_334_709_000;
        assert_eq!(file_name(ms), "katabox-backup-20261122-080509.kbx.json");
        assert!(is_backup_name(&file_name(ms)));
        for bad in [
            "katabox-backup-20261122-080509.kbx.json.tmp",
            "katabox-backup-2026112-080509.kbx.json",
            "katabox-backup-20261122-08050a.kbx.json",
            "my-katabox-backup-20261122-080509.kbx.json",
            "katabox-backup-20261122_080509.kbx.json",
            "README-katabox-backups.txt",
            "notes.txt",
        ] {
            assert!(!is_backup_name(bad), "{bad}");
        }
    }

    #[test]
    fn pruning_keeps_the_newest_and_never_foreign_files() {
        let names: Vec<String> = [
            "katabox-backup-20261101-000000.kbx.json",
            "katabox-backup-20261103-000000.kbx.json",
            "katabox-backup-20261102-000000.kbx.json",
            "katabox-backup-20261001-000000.kbx.json.tmp",
            "holiday.jpg",
            README,
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            to_prune(&names, 2),
            ["katabox-backup-20261101-000000.kbx.json"]
        );
        assert!(to_prune(&names, 5).is_empty());
        // Keep is at least one.
        assert_eq!(to_prune(&names, 0).len(), 2);
    }

    #[test]
    fn due_after_every_days() {
        assert!(due(10, None, 7));
        assert!(!due(6 * DAY_MS, Some(0), 7));
        assert!(due(7 * DAY_MS, Some(0), 7));
    }
}
