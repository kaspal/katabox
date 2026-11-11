//! Installing and uninstalling languages (ADR 0015).
//!
//! A job, under an in-process mutex and a cross-process lock file:
//! 1. purge leftovers (staging, receipt-less component folders, stale downloads);
//! 2. plan the missing components and check free space;
//! 3. per component: download and verify its artifacts, extract them into a fresh
//!    staging folder, rename that folder into place as a whole, write the receipt last.
//!
//! "Installed" means a receipt whose digest matches the compiled manifest *and* whose
//! check file exists; receipts are never trusted for paths, URLs or hashes.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use kb_core::Language;
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::download::Downloader;
use crate::extract::{Budget, assert_no_links, extract_source};
use crate::layout::{LOCK_FILE, Layout, check_root, ensure_real_dir};
use crate::manifest::{Artifact, Component, Manifest, Platform, UNITS};
use crate::{Error, Result};

const RECEIPT_SCHEMA: u32 = 1;

/// Written last, after a component is in place.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub schema: u32,
    pub component: String,
    pub digest: String,
    pub artifacts: Vec<String>,
    pub installed_bytes: u64,
    pub app_version: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InstallOptions {
    /// Install only what the judge runs (no language servers): CI's Linux jobs.
    pub runtimes_only: bool,
}

/// One component of an install plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanItem {
    pub component: String,
    /// Bytes still to download (shared artifacts counted once, on first use).
    pub download_bytes: u64,
    pub installed_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum ComponentPhase {
    Queued,
    Downloading,
    Verifying,
    Extracting,
    Installing,
    Installed,
}

/// Progress of an install, in order: one `Plan`, then `Progress` per component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InstallEvent {
    Plan {
        items: Vec<PlanItem>,
    },
    Progress {
        component: String,
        phase: ComponentPhase,
        /// Bytes downloaded so far (downloading), else 0.
        done: u64,
        total: u64,
    },
}

/// Per language, for the first-run screen and Settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LanguageStatus {
    pub language: Language,
    /// Installable on this platform.
    pub available: bool,
    /// Runtime and language server installed.
    pub installed: bool,
    pub runtime_installed: bool,
    pub servers_installed: bool,
    pub components: Vec<String>,
    /// What an install would still download.
    pub download_bytes: u64,
    /// Disk use of the whole language once installed.
    pub installed_bytes: u64,
}

/// Components whose receipts were removed by [`Manager::uninstall_prepare`]; the folders
/// go in [`Manager::uninstall_finish`]. Holds the locks in between.
#[derive(Debug)]
pub struct Uninstalled {
    pub components: Vec<String>,
    dirs: Vec<PathBuf>,
    clear_rust_cache: bool,
    _job: tokio::sync::OwnedMutexGuard<()>,
    _lock: RootLock,
}

pub struct Manager {
    layout: Layout,
    manifest: &'static Manifest,
    platform: Option<Platform>,
    job: Arc<tokio::sync::Mutex<()>>,
    cancel: std::sync::Mutex<Option<CancellationToken>>,
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager")
            .field("root", &self.layout.root)
            .finish_non_exhaustive()
    }
}

impl Manager {
    /// Manage a marked sandbox root with the built-in manifest, for this platform.
    pub fn open(root: &Path) -> Result<Manager> {
        Manager::with_manifest(root, Manifest::builtin(), Platform::current())
    }

    /// Manage a root with another manifest (tests) or platform.
    pub fn with_manifest(
        root: &Path,
        manifest: &'static Manifest,
        platform: Option<Platform>,
    ) -> Result<Manager> {
        let root = check_root(root)?;
        let layout =
            Layout::with_manifest(root, platform.unwrap_or(Platform::WindowsX86_64), manifest);
        Ok(Manager {
            layout,
            manifest,
            platform,
            job: Arc::new(tokio::sync::Mutex::new(())),
            cancel: std::sync::Mutex::new(None),
        })
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn root(&self) -> &Path {
        &self.layout.root
    }

    pub fn platform(&self) -> Option<Platform> {
        self.platform
    }

    /// Whether a component is installed: a valid receipt and its check file.
    pub fn component_installed(&self, id: &str) -> bool {
        let Some(p) = self.platform else { return false };
        let Some(c) = self.manifest.component(id, p) else {
            return false;
        };
        self.receipt_valid(c, p)
    }

    /// The judge can run `language` (its runtime is installed).
    pub fn runtime_installed(&self, language: Language) -> bool {
        self.unit_installed(language, false)
    }

    /// The language server for `language` can run (its components are installed).
    pub fn servers_installed(&self, language: Language) -> bool {
        self.unit_installed(language, true)
    }

    fn unit_installed(&self, language: Language, servers: bool) -> bool {
        let Some(unit) = self.manifest.language(language) else {
            return false;
        };
        let ids = if servers {
            &unit.servers
        } else {
            &unit.runtime
        };
        let Some(p) = self.platform else { return false };
        ids.iter()
            .all(|id| self.manifest.component(id, p).is_some() && self.component_installed(id))
    }

    /// Install state of every language the user can pick.
    pub fn status(&self) -> Vec<LanguageStatus> {
        UNITS
            .iter()
            .map(|&language| {
                let comps = self
                    .platform
                    .and_then(|p| self.manifest.language_components(language, p, false));
                match comps {
                    None => LanguageStatus {
                        language,
                        available: false,
                        installed: false,
                        runtime_installed: false,
                        servers_installed: false,
                        components: Vec::new(),
                        download_bytes: 0,
                        installed_bytes: 0,
                    },
                    Some(comps) => {
                        let p = self.platform.expect("components imply a platform");
                        let mut seen = BTreeSet::new();
                        let mut download = 0;
                        for c in comps.iter().filter(|c| !self.receipt_valid(c, p)) {
                            for s in &c.sources {
                                if let Some(a) = self.manifest.artifact(&s.artifact, p) {
                                    if seen.insert(a.id.clone()) {
                                        download += a.size;
                                    }
                                }
                            }
                        }
                        let runtime_installed = self.runtime_installed(language);
                        let servers_installed = self.servers_installed(language);
                        LanguageStatus {
                            language,
                            available: true,
                            installed: runtime_installed && servers_installed,
                            runtime_installed,
                            servers_installed,
                            components: comps.iter().map(|c| c.id.clone()).collect(),
                            download_bytes: download,
                            installed_bytes: comps.iter().map(|c| c.installed_size).sum(),
                        }
                    }
                }
            })
            .collect()
    }

    /// Free bytes on the root's volume.
    pub fn free_space(&self) -> Option<u64> {
        fs4::available_space(&self.layout.root).ok()
    }

    /// Cancel the running install, if any (its `.part` files are kept for a retry).
    pub fn cancel(&self) {
        if let Some(t) = self
            .cancel
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            t.cancel();
        }
    }

    /// Remove leftovers: staging (trash included), receipt-less or outdated component
    /// folders at known destinations, downloads of no current artifact, unknown receipts.
    /// Returns `Busy` while another process installs into this root.
    pub fn purge(&self) -> Result<()> {
        let _lock = RootLock::acquire(&self.layout.root)?;
        self.purge_locked()
    }

    fn purge_locked(&self) -> Result<()> {
        let l = &self.layout;
        // Staging: everything goes; what cannot be deleted now (locked files) waits.
        if let Ok(rd) = fs::read_dir(l.staging()) {
            for e in rd.flatten() {
                let _ = remove_all(&e.path());
            }
        }
        let Some(p) = self.platform else {
            return Ok(());
        };
        let mine: Vec<&Component> = self
            .manifest
            .components
            .iter()
            .filter(|c| {
                self.manifest
                    .component(&c.id, p)
                    .is_some_and(|x| std::ptr::eq(x, *c))
            })
            .collect();
        for c in &mine {
            if self.receipt_valid(c, p) {
                continue;
            }
            let _ = remove_file(&self.receipt_path(&c.id));
            let dest = l.dest(c);
            if fs::symlink_metadata(&dest).is_ok() {
                ensure_real_dir(&l.root, &l.staging())?;
                let trash = l.staging().join(format!("trash-{}", short_id()));
                retry_io(&dest, || fs::rename(&dest, &trash))?;
                let _ = remove_all(&trash);
            }
        }
        // Receipts of components this manifest does not know.
        if let Ok(rd) = fs::read_dir(l.receipts()) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                let id = name.strip_suffix(".json").unwrap_or("");
                if !mine.iter().any(|c| c.id == id) {
                    let _ = remove_file(&e.path());
                }
            }
        }
        // Downloads: keep only current artifacts (finished or partial).
        if let Ok(rd) = fs::read_dir(l.downloads()) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                let sha = name.strip_suffix(".part").unwrap_or(&name);
                if !self.manifest.artifacts.iter().any(|a| a.sha256 == sha) {
                    let _ = remove_file(&e.path());
                }
            }
        }
        Ok(())
    }

    /// Install `languages` (plus Node when they need it). Events go to `on_event`.
    pub async fn install(
        &self,
        languages: &[Language],
        opts: InstallOptions,
        downloader: &Downloader,
        on_event: &(dyn Fn(InstallEvent) + Send + Sync),
    ) -> Result<()> {
        let _job = self.job.clone().try_lock_owned().map_err(|_| Error::Busy)?;
        let _lock = RootLock::acquire(&self.layout.root)?;
        let cancel = CancellationToken::new();
        *self.cancel.lock().unwrap_or_else(|e| e.into_inner()) = Some(cancel.clone());
        let result = self
            .install_locked(languages, opts, downloader, on_event, &cancel)
            .await;
        *self.cancel.lock().unwrap_or_else(|e| e.into_inner()) = None;
        result
    }

    async fn install_locked(
        &self,
        languages: &[Language],
        opts: InstallOptions,
        downloader: &Downloader,
        on_event: &(dyn Fn(InstallEvent) + Send + Sync),
        cancel: &CancellationToken,
    ) -> Result<()> {
        let p = self.platform.ok_or_else(|| {
            Error::UnsupportedPlatform(format!(
                "{} {}",
                std::env::consts::OS,
                std::env::consts::ARCH
            ))
        })?;
        let l = &self.layout;
        for dir in [l.staging(), l.downloads(), l.receipts()] {
            ensure_real_dir(&l.root, &dir)?;
        }
        self.purge_locked()?;

        // Plan: the missing components, in a stable order (runtimes first).
        let mut comps: Vec<&Component> = Vec::new();
        for &lang in languages {
            let list = self
                .manifest
                .language_components(lang, p, opts.runtimes_only)
                .ok_or_else(|| Error::UnsupportedPlatform(lang.to_string()))?;
            for c in list {
                if !comps.iter().any(|x| x.id == c.id) {
                    comps.push(c);
                }
            }
        }
        comps.retain(|c| !self.receipt_valid(c, p));
        if comps.is_empty() {
            on_event(InstallEvent::Plan { items: Vec::new() });
            return Ok(());
        }
        let mut counted = BTreeSet::new();
        let mut items = Vec::new();
        let mut need_download = 0u64;
        for c in &comps {
            let mut d = 0;
            for a in self.artifacts_of(c, p)? {
                if counted.insert(a.id.clone()) {
                    d += remaining_download(&l.downloads(), a);
                }
            }
            need_download += d;
            items.push(PlanItem {
                component: c.id.clone(),
                download_bytes: d,
                installed_bytes: c.installed_size,
            });
        }
        let installed: u64 = comps.iter().map(|c| c.installed_size).sum();
        on_event(InstallEvent::Plan { items });
        let needed = need_download + installed + installed / 10 + (200 << 20);
        if let Some(available) = self.free_space() {
            if available < needed {
                return Err(Error::NotEnoughSpace { needed, available });
            }
        }
        for c in &comps {
            on_event(progress(&c.id, crate::ComponentPhase::Queued, 0, 0));
        }

        let job = short_id();
        let staging = l.staging().join(&job);
        ensure_real_dir(&l.root, &staging)?;
        let mut fetched: HashMap<String, PathBuf> = HashMap::new();
        let result = async {
            for (i, c) in comps.iter().enumerate() {
                // 1. Download and verify.
                let artifacts = self.artifacts_of(c, p)?;
                let total: u64 = artifacts.iter().map(|a| a.size).sum();
                let mut base = 0u64;
                for a in &artifacts {
                    if let Some(path) = fetched.get(&a.id) {
                        let _ = path;
                        base += a.size;
                        continue;
                    }
                    let id = c.id.clone();
                    let mut last_emit: Option<u64> = None;
                    let mut cb = |done: u64, _t: u64| {
                        let now = base + done;
                        // The first bytes, then every 256 KB, and the end.
                        let due = match last_emit {
                            None => true,
                            Some(prev) => {
                                now == base + a.size || now >= prev + (256 << 10) || now < prev
                            }
                        };
                        if due {
                            last_emit = Some(now);
                            on_event(progress(
                                &id,
                                crate::ComponentPhase::Downloading,
                                now,
                                total,
                            ));
                        }
                    };
                    let path = downloader.fetch(a, &l.downloads(), cancel, &mut cb).await?;
                    fetched.insert(a.id.clone(), path);
                    base += a.size;
                }
                if cancel.is_cancelled() {
                    return Err(Error::Cancelled);
                }
                // 2. Verify again from the open file and extract into staging.
                on_event(progress(&c.id, crate::ComponentPhase::Verifying, 0, 0));
                let dir = staging.join(i.to_string());
                fs::create_dir(&dir)
                    .map_err(|e| Error::io(format!("create {}", dir.display()), e))?;
                let sources: Vec<(PathBuf, Artifact, crate::manifest::Source)> = c
                    .sources
                    .iter()
                    .map(|s| {
                        let a = self
                            .manifest
                            .artifact(&s.artifact, p)
                            .expect("validated")
                            .clone();
                        (fetched[&a.id].clone(), a, s.clone())
                    })
                    .collect();
                on_event(progress(&c.id, crate::ComponentPhase::Extracting, 0, 0));
                let (dir2, cancel2, size, check) = (
                    dir.clone(),
                    cancel.clone(),
                    c.installed_size,
                    c.check.clone(),
                );
                tokio::task::spawn_blocking(move || -> Result<()> {
                    let mut budget = Budget::for_size(size);
                    for (file, a, s) in &sources {
                        extract_source(file, a, s, &dir2, &mut budget, &cancel2)?;
                    }
                    assert_no_links(&dir2)?;
                    if !check_path(&dir2, &check).is_file() {
                        return Err(Error::UnsafeArchive {
                            artifact: String::new(),
                            entry: check,
                            why: "the expected file is missing after extraction".into(),
                        });
                    }
                    sync_tree(&dir2);
                    Ok(())
                })
                .await
                .map_err(|e| Error::Manifest(format!("extract task: {e}")))??;
                if cancel.is_cancelled() {
                    return Err(Error::Cancelled);
                }
                // 3. Promote (not cancellable): rename into place, then the receipt.
                on_event(progress(&c.id, crate::ComponentPhase::Installing, 0, 0));
                self.promote(c, p, &dir, &artifacts)?;
                on_event(progress(&c.id, crate::ComponentPhase::Installed, 0, 0));
                // Downloads no later component needs are deleted.
                for a in &artifacts {
                    let later = comps[i + 1..]
                        .iter()
                        .any(|n| n.sources.iter().any(|s| s.artifact == a.id));
                    if !later {
                        if let Some(path) = fetched.get(&a.id) {
                            let _ = remove_file(path);
                        }
                    }
                }
            }
            Ok(())
        }
        .await;
        let _ = remove_all(&staging);
        result
    }

    fn promote(
        &self,
        c: &Component,
        p: Platform,
        staged: &Path,
        artifacts: &[&Artifact],
    ) -> Result<()> {
        let l = &self.layout;
        let dest = l.dest(c);
        let parent = dest.parent().expect("dest has a parent");
        ensure_real_dir(&l.root, parent)?;
        // A folder still here has no valid receipt (purge just ran): replace it whole.
        if fs::symlink_metadata(&dest).is_ok() {
            let _ = remove_file(&self.receipt_path(&c.id));
            let trash = l.staging().join(format!("trash-{}", short_id()));
            retry_io(&dest, || fs::rename(&dest, &trash))?;
            let _ = remove_all(&trash);
        }
        retry_io(&dest, || fs::rename(staged, &dest))?;
        // Antivirus may quarantine a file right after it lands: check before the receipt.
        if !check_path(&dest, &c.check).is_file() {
            return Err(Error::FileLocked {
                path: check_path(&dest, &c.check),
            });
        }
        let receipt = Receipt {
            schema: RECEIPT_SCHEMA,
            component: c.id.clone(),
            digest: self.manifest.digest(c, p),
            artifacts: artifacts.iter().map(|a| a.sha256.clone()).collect(),
            installed_bytes: c.installed_size,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        write_receipt(&self.receipt_path(&c.id), &receipt)
    }

    /// Step 1 of an uninstall: delete the receipts of the language's components that no
    /// other installed language needs (so they count as not installed right away). The
    /// caller then stops whatever uses them and calls [`Manager::uninstall_finish`].
    pub fn uninstall_prepare(&self, language: Language) -> Result<Uninstalled> {
        let job = self.job.clone().try_lock_owned().map_err(|_| Error::Busy)?;
        let lock = RootLock::acquire(&self.layout.root)?;
        let p = self
            .platform
            .ok_or_else(|| Error::UnsupportedPlatform(language.to_string()))?;
        let unit = self
            .manifest
            .language(language)
            .ok_or_else(|| Error::UnsupportedPlatform(language.to_string()))?;
        let mine: Vec<String> = unit.runtime.iter().chain(&unit.servers).cloned().collect();
        // Components another language still needs, judged by whether that language's
        // runtime is installed.
        let mut keep = BTreeSet::new();
        for other in &self.manifest.languages {
            if other.id == unit.id {
                continue;
            }
            let other_lang = UNITS
                .iter()
                .find(|l| crate::manifest::unit_id(**l) == other.id);
            if other_lang.is_some_and(|l| self.runtime_installed(*l)) {
                keep.extend(other.runtime.iter().chain(&other.servers).cloned());
            }
        }
        let mut components = Vec::new();
        let mut dirs = Vec::new();
        for id in mine {
            if keep.contains(&id) {
                continue;
            }
            let Some(c) = self.manifest.component(&id, p) else {
                continue;
            };
            let dest = self.layout.dest(c);
            if fs::symlink_metadata(&dest).is_err() && !self.receipt_path(&id).exists() {
                continue;
            }
            remove_file(&self.receipt_path(&id))?;
            components.push(id);
            dirs.push(dest);
        }
        Ok(Uninstalled {
            clear_rust_cache: components.iter().any(|c| c == "rust"),
            components,
            dirs,
            _job: job,
            _lock: lock,
        })
    }

    /// Step 2: move the folders to the trash and delete them. A folder that stays locked
    /// is left in staging for the next purge (its receipt is already gone).
    pub fn uninstall_finish(&self, u: Uninstalled) -> Result<()> {
        let l = &self.layout;
        ensure_real_dir(&l.root, &l.staging())?;
        let mut first_err = None;
        for dest in &u.dirs {
            if fs::symlink_metadata(dest).is_err() {
                continue;
            }
            let trash = l.staging().join(format!("trash-{}", short_id()));
            match retry_io(dest, || fs::rename(dest, &trash)) {
                Ok(()) => {
                    let _ = remove_all(&trash);
                }
                Err(e) => {
                    first_err.get_or_insert(e);
                }
            }
        }
        if u.clear_rust_cache {
            let _ = remove_all(&l.rust_cache());
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Both uninstall steps (the CLI; the app stops language servers in between).
    pub fn uninstall(&self, language: Language) -> Result<Vec<String>> {
        let u = self.uninstall_prepare(language)?;
        let ids = u.components.clone();
        self.uninstall_finish(u)?;
        Ok(ids)
    }

    fn artifacts_of<'m>(&'m self, c: &Component, p: Platform) -> Result<Vec<&'m Artifact>> {
        let mut out: Vec<&Artifact> = Vec::new();
        for s in &c.sources {
            let a = self
                .manifest
                .artifact(&s.artifact, p)
                .ok_or_else(|| Error::Manifest(format!("{}: no artifact {}", c.id, s.artifact)))?;
            if !out.iter().any(|x| x.id == a.id) {
                out.push(a);
            }
        }
        Ok(out)
    }

    fn receipt_path(&self, id: &str) -> PathBuf {
        self.layout.receipts().join(format!("{id}.json"))
    }

    fn receipt_valid(&self, c: &Component, p: Platform) -> bool {
        let Ok(text) = fs::read_to_string(self.receipt_path(&c.id)) else {
            return false;
        };
        let Ok(r) = serde_json::from_str::<Receipt>(&text) else {
            return false;
        };
        r.schema == RECEIPT_SCHEMA
            && r.component == c.id
            && r.digest == self.manifest.digest(c, p)
            && check_path(&self.layout.dest(c), &c.check).is_file()
    }
}

fn progress(component: &str, phase: ComponentPhase, done: u64, total: u64) -> InstallEvent {
    InstallEvent::Progress {
        component: component.to_string(),
        phase,
        done,
        total,
    }
}

/// Bytes still to download for an artifact (a finished or partial file counts).
fn remaining_download(dir: &Path, a: &Artifact) -> u64 {
    if dir.join(&a.sha256).is_file() {
        return 0;
    }
    let have = fs::metadata(dir.join(format!("{}.part", a.sha256)))
        .map(|m| m.len())
        .unwrap_or(0);
    a.size.saturating_sub(have.min(a.size))
}

fn check_path(dir: &Path, check: &str) -> PathBuf {
    let mut p = dir.to_path_buf();
    for part in check.split('/') {
        p.push(part);
    }
    p
}

/// A short, unique-enough id for staging folders (paths stay short for long rust-src names).
fn short_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let x = t
        ^ (u64::from(std::process::id()) << 32)
        ^ N.fetch_add(1, Ordering::Relaxed)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    format!("{:08x}", (x ^ (x >> 32)) as u32)
}

fn write_receipt(path: &Path, r: &Receipt) -> Result<()> {
    let tmp = path.with_extension("json.tmp");
    let text = serde_json::to_string_pretty(r).map_err(|e| Error::Manifest(e.to_string()))?;
    {
        use std::io::Write;
        let mut f =
            fs::File::create(&tmp).map_err(|e| Error::io(format!("write {}", tmp.display()), e))?;
        f.write_all(text.as_bytes())
            .map_err(|e| Error::io("write receipt", e))?;
        f.sync_all().map_err(|e| Error::io("flush receipt", e))?;
    }
    retry_io(path, || fs::rename(&tmp, path))?;
    #[cfg(unix)]
    if let Some(dir) = path.parent() {
        if let Ok(d) = fs::File::open(dir) {
            let _ = d.sync_all();
        }
    }
    Ok(())
}

/// Before a receipt: make the staged files durable (Linux `syncfs`; NTFS journals the
/// rename itself on Windows).
fn sync_tree(dir: &Path) {
    #[cfg(unix)]
    if let Ok(f) = fs::File::open(dir) {
        let _ = rustix::fs::syncfs(&f);
    }
    #[cfg(not(unix))]
    let _ = dir;
}

/// Renames and deletes on Windows fail while antivirus, the indexer or a running program
/// has a file open: retry for about 20 s, then report `file_locked`.
fn retry_io(path: &Path, mut op: impl FnMut() -> std::io::Result<()>) -> Result<()> {
    let mut delay = Duration::from_millis(100);
    let mut waited = Duration::ZERO;
    loop {
        match op() {
            Ok(()) => return Ok(()),
            Err(e) if is_locked(&e) && waited < Duration::from_secs(20) => {
                std::thread::sleep(delay);
                waited += delay;
                delay = (delay * 2).min(Duration::from_secs(2));
            }
            Err(e) if is_locked(&e) => {
                return Err(Error::FileLocked {
                    path: path.to_path_buf(),
                });
            }
            Err(e) => return Err(Error::io(format!("move {}", path.display()), e)),
        }
    }
}

fn is_locked(e: &std::io::Error) -> bool {
    cfg!(windows) && matches!(e.raw_os_error(), Some(5 | 32 | 33))
}

fn remove_file(p: &Path) -> Result<()> {
    match fs::remove_file(p) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Error::io(format!("remove {}", p.display()), e)),
    }
}

fn remove_all(p: &Path) -> Result<()> {
    match fs::symlink_metadata(p) {
        Err(_) => Ok(()),
        Ok(m) if m.is_dir() && !crate::layout::is_link(&m) => retry_io(p, || fs::remove_dir_all(p)),
        Ok(_) => retry_io(p, || fs::remove_file(p)),
    }
}

/// The cross-process install lock (`<root>/.install.lock`), released on drop.
#[derive(Debug)]
struct RootLock(fs::File);

impl RootLock {
    fn acquire(root: &Path) -> Result<RootLock> {
        let f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(root.join(LOCK_FILE))
            .map_err(|e| Error::io("open the install lock", e))?;
        match fs4::FileExt::try_lock(&f) {
            Ok(()) => Ok(RootLock(f)),
            Err(_) => Err(Error::Busy),
        }
    }
}

impl Drop for RootLock {
    fn drop(&mut self) {
        let _ = fs4::FileExt::unlock(&self.0);
    }
}
