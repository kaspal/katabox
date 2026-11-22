//! Problem packs in the app (expansion M11, ADR 0038): import (preview, then apply),
//! uninstall, and opening the content with the installed packs merged in.
//!
//! Files live in `<local data>/packs/`:
//! - `.staging-<n>/` while a preview is pending (`src/` extracted and checked, plus the
//!   pack's own `content.db`); deleted on cancel or failure;
//! - `<id>/` once applied (whole-folder promotion: a rename); uninstall deletes only this
//!   folder (T12);
//! - `merged.db` + `merged.fp`: the core content with every installed pack's problems,
//!   rebuilt when the fingerprint (core file, packs and their hashes) changes.
//!
//! Installing, updating or removing a pack takes effect when the app restarts.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use kb_pack::kbpack::{self, Version};
use kb_store::packs::PackRow;
use kb_store::{ContentDb, Store};
use serde::Serialize;

use crate::{App, AppError, AppErrorCode, Result};

static STAGING: AtomicU64 = AtomicU64::new(1);

/// What a pack would do, shown before importing (T13: unsigned, author as stated).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PackPreview {
    /// Pass back to `import_pack_apply` / `import_pack_cancel`.
    pub token: u64,
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub licence: String,
    /// SHA-256 of the `.kbpack` file.
    pub sha256: String,
    pub problems: u32,
    /// The installed version of the same pack, if any.
    pub installed: Option<String>,
    pub action: PackAction,
    /// The title of the assignment it carries (classroom mode, expansion M12).
    pub assignment: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum PackAction {
    Install,
    /// A newer version of an installed pack.
    Update,
    /// The same file is already installed: nothing to do.
    Same,
    /// Not newer than the installed version (with different content): refused (A3).
    NotNewer,
}

#[derive(Debug)]
pub(crate) struct PendingPack {
    token: u64,
    staging: PathBuf,
    row: PackRow,
    action: PackAction,
    assignment: Option<kb_store::classroom::AssignmentMeta>,
}

fn refused(reasons: impl IntoIterator<Item = String>) -> AppError {
    let list: Vec<String> = reasons.into_iter().take(12).collect();
    AppError::new(AppErrorCode::PackRefused, list.join("\n"))
}

fn io(e: std::io::Error) -> AppError {
    AppError::new(AppErrorCode::Internal, format!("packs folder: {e}"))
}

pub(crate) fn packs_dir(local: &Path) -> PathBuf {
    local.join("packs")
}

/// Open the content the app uses: the core, or the core with installed packs merged in.
/// A pack that cannot be opened is skipped (and reported by `packs()`), never fatal.
pub(crate) async fn open_content(store: &mut Store, core: &Path, local: &Path) -> Result<()> {
    let rows: Vec<PackRow> = store
        .user
        .packs()
        .await?
        .into_iter()
        .filter(|p| p.removed_at.is_none())
        .collect();
    if rows.is_empty() {
        return Ok(());
    }
    let dir = packs_dir(local);
    let mut dbs = Vec::new();
    let mut fp = String::new();
    if let Ok(m) = std::fs::metadata(core) {
        let modified = m
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_secs());
        fp.push_str(&format!("core {} {modified}\n", m.len()));
    }
    for r in &rows {
        let db = dir.join(&r.id).join("content.db");
        // A pack built by an older app is rebuilt from its kept (already checked) sources.
        let usable = match ContentDb::open(&db).await {
            Ok(d) => {
                d.close().await;
                true
            }
            Err(_) => false,
        };
        if !usable {
            let src = dir.join(&r.id).join("src");
            let rebuilt =
                tokio::task::spawn_blocking(move || -> std::result::Result<(), String> {
                    let pack = kbpack::reopen_kept(&src)?;
                    let tmp = src.with_file_name("content.db.new");
                    let _ = std::fs::remove_file(&tmp);
                    kbpack::build_pack_db(&pack, &tmp)?;
                    std::fs::rename(&tmp, src.with_file_name("content.db"))
                        .map_err(|e| e.to_string())
                })
                .await;
            if !matches!(rebuilt, Ok(Ok(()))) {
                continue;
            }
        }
        fp.push_str(&format!("{} {}\n", r.id, r.sha256));
        dbs.push(db);
    }
    if dbs.is_empty() {
        return Ok(());
    }
    fp.push_str(&format!("schema {}\n", kb_store::CONTENT_SCHEMA_VERSION));
    let merged = dir.join("merged.db");
    let fp_file = dir.join("merged.fp");
    let fresh =
        merged.is_file() && std::fs::read_to_string(&fp_file).ok().as_deref() == Some(fp.as_str());
    if !fresh {
        let _ = std::fs::remove_file(&fp_file);
        kb_store::content::merge_packs(core, &dbs, &merged).await?;
        std::fs::write(&fp_file, &fp).map_err(io)?;
    }
    let content = ContentDb::open(&merged).await?;
    store.content.close().await;
    store.content = content;
    Ok(())
}

impl App {
    /// Installed packs (not removed), by id.
    pub async fn packs(&self) -> Result<Vec<PackRow>> {
        Ok(self
            .store
            .user
            .packs()
            .await?
            .into_iter()
            .filter(|p| p.removed_at.is_none())
            .collect())
    }

    /// Open a `.kbpack` and run every check (T1-T7, T9, T10): nothing is installed yet.
    pub async fn import_pack_preview(&self, file: &Path) -> Result<PackPreview> {
        let n = STAGING.fetch_add(1, Ordering::Relaxed);
        let staging = packs_dir(&self.local_dir).join(format!(".staging-{}-{n}", self.now_ms()));
        let builtin: BTreeSet<String> = self
            .store
            .content
            .builtin_ids()
            .await?
            .into_iter()
            .collect();
        let file = file.to_path_buf();
        let stage = staging.clone();
        let opened = tokio::task::spawn_blocking(
            move || -> std::result::Result<(kbpack::OpenedPack, String, u32), Vec<String>> {
                let sha = kbpack::archive::sha256_file(&file)
                    .map_err(|e| vec![format!("cannot read the pack: {e}")])?;
                let src = stage.join("src");
                let files =
                    kbpack::archive::extract(&file, &src).map_err(|r| vec![r.to_string()])?;
                let pack =
                    kbpack::check_extracted(&src, &files, &builtin, env!("CARGO_PKG_VERSION"))?;
                let count =
                    kbpack::build_pack_db(&pack, &stage.join("content.db")).map_err(|e| vec![e])?;
                Ok((pack, sha, u32::try_from(count).unwrap_or(u32::MAX)))
            },
        )
        .await
        .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        let (pack, sha, problems) = match opened {
            Ok(x) => x,
            Err(reasons) => {
                let _ = std::fs::remove_dir_all(&staging);
                return Err(refused(reasons));
            }
        };
        let assignment = pack.assignment;
        let m = pack.manifest;
        let installed = self.packs().await?.into_iter().find(|p| p.id == m.id);
        let action = match &installed {
            None => PackAction::Install,
            Some(i) if i.sha256 == sha => PackAction::Same,
            Some(i) => match (Version::parse(&m.version), Version::parse(&i.version)) {
                (Some(new), Some(old)) if new > old => PackAction::Update,
                _ => PackAction::NotNewer,
            },
        };
        let row = PackRow {
            id: m.id.clone(),
            name: m.name.clone(),
            author: m.author.clone(),
            version: m.version.clone(),
            licence: m.licence.clone(),
            sha256: sha.clone(),
            problems,
            installed_at: self.now_ms(),
            removed_at: None,
        };
        let token = n;
        let old = self
            .pending_pack
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .replace(PendingPack {
                token,
                staging,
                row,
                action,
                assignment: assignment.clone(),
            });
        if let Some(old) = old {
            let _ = std::fs::remove_dir_all(old.staging);
        }
        Ok(PackPreview {
            token,
            id: m.id,
            name: m.name,
            author: m.author,
            version: m.version,
            licence: m.licence,
            sha256: sha,
            problems,
            installed: installed.map(|i| i.version),
            action,
            assignment: assignment.map(|a| a.title),
        })
    }

    /// Install the previewed pack (or its newer version). Takes effect after a restart.
    pub async fn import_pack_apply(&self, token: u64) -> Result<PackRow> {
        let pending = {
            let mut slot = self.pending_pack.lock().unwrap_or_else(|e| e.into_inner());
            match slot.take() {
                Some(p) if p.token == token => p,
                other => {
                    *slot = other;
                    return Err(AppError::new(
                        AppErrorCode::NotFound,
                        "no pending pack import",
                    ));
                }
            }
        };
        match pending.action {
            PackAction::Install | PackAction::Update => {}
            PackAction::Same => {
                let _ = std::fs::remove_dir_all(&pending.staging);
                return Err(refused(["this pack is already installed".to_string()]));
            }
            PackAction::NotNewer => {
                let _ = std::fs::remove_dir_all(&pending.staging);
                return Err(refused([
                    "a pack with this id is installed: only a newer version can replace it"
                        .to_string(),
                ]));
            }
        }
        let dest = packs_dir(&self.local_dir).join(&pending.row.id);
        // Whole-folder promotion: the old folder moves aside, the new one takes its place.
        let aside = dest.with_file_name(format!(".old-{}-{}", pending.row.id, self.now_ms()));
        if dest.exists() {
            std::fs::rename(&dest, &aside).map_err(io)?;
        }
        if let Err(e) = std::fs::rename(&pending.staging, &dest) {
            if aside.exists() {
                let _ = std::fs::rename(&aside, &dest);
            }
            return Err(io(e));
        }
        let _ = std::fs::remove_dir_all(&aside);
        self.store.user.put_pack(&pending.row).await?;
        if let Some(a) = &pending.assignment {
            self.store
                .user
                .put_assignment(&pending.row.id, &pending.row.sha256, a, self.now_ms())
                .await?;
        }
        Ok(pending.row)
    }

    pub fn import_pack_cancel(&self, token: u64) {
        let mut slot = self.pending_pack.lock().unwrap_or_else(|e| e.into_inner());
        if slot.as_ref().is_some_and(|p| p.token == token)
            && let Some(p) = slot.take()
        {
            let _ = std::fs::remove_dir_all(p.staging);
        }
    }

    /// Remove a pack: its folder only (T12). Its problems' history stays in user.db and
    /// comes back with a reinstall. Takes effect after a restart.
    pub async fn uninstall_pack(&self, id: &str) -> Result<()> {
        if !kbpack::valid_pack_id(id) {
            return Err(AppError::new(AppErrorCode::InvalidInput, "not a pack id"));
        }
        if !self.store.user.remove_pack(id, self.now_ms()).await? {
            return Err(AppError::new(
                AppErrorCode::NotFound,
                format!("pack {id} is not installed"),
            ));
        }
        let dir = packs_dir(&self.local_dir).join(id);
        if dir.is_dir() {
            std::fs::remove_dir_all(&dir).map_err(io)?;
        }
        Ok(())
    }
}
