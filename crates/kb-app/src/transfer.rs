//! Progress export and import (ADR 0020): one `.kbx.json` file, format
//! `katabox-export/v2` (v1 files still import; expansion M0). Import is strict — size cap, exact format, no unknown fields,
//! every value checked — and merges by key, keeping the newer record. Nothing in a file
//! is ever executed: code is stored as text.

use std::path::Path;

use kb_core::Day;
use kb_store::ExportData;
use serde::{Deserialize, Serialize};

use crate::{App, AppError, AppErrorCode, CALIBRATION_SETTING, Result, SANDBOX_ROOT_SETTING};

/// The format every export writes. v2 adds the expansion's record kinds, each an
/// optional (`serde(default)`) section of [`ExportData`].
pub const EXPORT_FORMAT: &str = "katabox-export/v2";
/// The previous format: still imported, as long as it holds none of v2's sections.
pub const EXPORT_FORMAT_V1: &str = "katabox-export/v1";
/// Largest file Import reads.
pub const IMPORT_MAX_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ROWS: usize = 200_000;
const MAX_CODE: usize = 1024 * 1024;
const MAX_NOTE: usize = 256 * 1024;
const MAX_REPORT: usize = 4 * 1024 * 1024;

/// Settings that stay on this machine.
fn machine_setting(key: &str) -> bool {
    key == kb_store::INSTALL_ID_SETTING || key == CALIBRATION_SETTING || key == SANDBOX_ROOT_SETTING
}

/// Learn progress travels with an export (ADR 0029); the Learn page layout and speed
/// stay on this machine.
const LEARN_SETTINGS: [&str; 2] = ["learn.viewed", "learn.last"];

/// Appearance preferences that travel with an export (features-content M11).
const UI_SETTINGS: [&str; 2] = ["ui.contrast", "ui.motion"];

/// Settings an import may write.
fn importable_setting(key: &str) -> bool {
    ["editor.", "workspace.", "practice."]
        .iter()
        .any(|p| key.starts_with(p) && key.len() <= 64)
        || key == "ui_lang"
        || key == "theme"
        || LEARN_SETTINGS.contains(&key)
        || UI_SETTINGS.contains(&key)
}

/// Settings an export leaves out: machine-specific ones, and any an import would refuse
/// (so an export always imports).
fn unexported_setting(key: &str) -> bool {
    machine_setting(key) || !importable_setting(key)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExportFile {
    format: String,
    /// UTC milliseconds.
    exported_at: i64,
    app_version: String,
    data: ExportData,
}

/// How many records of each kind a file holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TransferCounts {
    pub statuses: u32,
    pub submissions: u32,
    pub snapshots: u32,
    pub notes: u32,
    pub drafts: u32,
    pub hint_unlocks: u32,
    pub opens: u32,
    pub reviews: u32,
    pub daily_picks: u32,
    pub lists: u32,
    pub interviews: u32,
    pub settings: u32,
}

fn counts(d: &ExportData) -> TransferCounts {
    let n = |v: usize| u32::try_from(v).unwrap_or(u32::MAX);
    TransferCounts {
        statuses: n(d.statuses.len()),
        submissions: n(d.submissions.len()),
        snapshots: n(d.snapshots.len()),
        notes: n(d.notes.len()),
        drafts: n(d.drafts.len()),
        hint_unlocks: n(d.hint_unlocks.len()),
        opens: n(d.opens.len()),
        reviews: n(d.reviews.len()),
        daily_picks: n(d.daily_picks.len()),
        lists: n(d.lists.len()),
        interviews: n(d.interviews.len()),
        settings: n(d.settings.len()),
    }
}

/// What Import found in a file, before merging.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ImportPreview {
    /// Pass back to `import_apply` / `import_cancel`.
    pub id: u32,
    pub counts: TransferCounts,
    /// Records that already exist here (the newer one will be kept).
    pub conflicts: u32,
    pub exported_at: i64,
    pub app_version: String,
}

/// What Import changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ImportResult {
    pub added: u32,
    pub updated: u32,
    pub unchanged: u32,
}

fn invalid(why: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::ImportInvalid, why.into())
}

fn check_id(path: &str, id: &str) -> Result<()> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    if ok {
        Ok(())
    } else {
        Err(invalid(format!("{path}: bad problem id {id:?}")))
    }
}

fn check_time(path: &str, t: i64) -> Result<()> {
    // 2000-01-01 .. 2200-01-01 in milliseconds.
    if (946_684_800_000..=7_258_118_400_000).contains(&t) {
        Ok(())
    } else {
        Err(invalid(format!("{path}: time out of range")))
    }
}

fn check_len(path: &str, s: &str, max: usize) -> Result<()> {
    if s.len() <= max {
        Ok(())
    } else {
        Err(invalid(format!("{path}: longer than {max} bytes")))
    }
}

fn check_day(path: &str, s: &str) -> Result<()> {
    s.parse::<Day>()
        .map(|_| ())
        .map_err(|e| invalid(format!("{path}: {e}")))
}

/// Every value of a parsed file, checked (types were already checked by serde).
fn validate(f: &ExportFile) -> Result<()> {
    if f.format != EXPORT_FORMAT && f.format != EXPORT_FORMAT_V1 {
        return Err(invalid(format!(
            "format must be {EXPORT_FORMAT:?} or {EXPORT_FORMAT_V1:?}, got {:?}",
            f.format
        )));
    }
    if f.format == EXPORT_FORMAT_V1 && !f.data.v2_sections_empty() {
        return Err(invalid(format!("a {EXPORT_FORMAT_V1} file has v2 records")));
    }
    check_len("app_version", &f.app_version, 64)?;
    let d = &f.data;
    let sizes = [
        d.statuses.len(),
        d.submissions.len(),
        d.snapshots.len(),
        d.notes.len(),
        d.drafts.len(),
        d.hint_unlocks.len(),
        d.opens.len(),
        d.reviews.len(),
        d.daily_picks.len(),
        d.lists.len(),
        d.interviews.len(),
        d.settings.len(),
    ];
    if sizes.iter().any(|n| *n > MAX_ROWS) {
        return Err(invalid(format!("more than {MAX_ROWS} records of one kind")));
    }
    for (i, r) in d.statuses.iter().enumerate() {
        let p = format!("statuses[{i}]");
        check_id(&p, &r.problem_id)?;
        check_time(&p, r.updated_at)?;
        if let Some(t) = r.first_solved_at {
            check_time(&p, t)?;
        }
    }
    for (i, r) in d.submissions.iter().enumerate() {
        let p = format!("submissions[{i}]");
        check_id(&p, &r.problem_id)?;
        check_len(&p, &r.code, MAX_CODE)?;
        check_time(&p, r.created_at)?;
        if r.time_us < 0 || r.memory_kb < 0 {
            return Err(invalid(format!("{p}: negative time or memory")));
        }
        if !r.report.is_object() || r.report.to_string().len() > MAX_REPORT {
            return Err(invalid(format!(
                "{p}: report must be an object under {MAX_REPORT} bytes"
            )));
        }
    }
    for (i, r) in d.snapshots.iter().enumerate() {
        let p = format!("snapshots[{i}]");
        check_id(&p, &r.problem_id)?;
        check_len(&p, &r.code, MAX_CODE)?;
        check_time(&p, r.created_at)?;
    }
    for (i, r) in d.notes.iter().enumerate() {
        let p = format!("notes[{i}]");
        check_id(&p, &r.problem_id)?;
        check_len(&p, &r.body, MAX_NOTE)?;
        check_time(&p, r.updated_at)?;
    }
    for (i, r) in d.drafts.iter().enumerate() {
        let p = format!("drafts[{i}]");
        check_id(&p, &r.problem_id)?;
        check_len(&p, &r.code, MAX_CODE)?;
        check_time(&p, r.updated_at)?;
    }
    for (i, r) in d.hint_unlocks.iter().enumerate() {
        let p = format!("hint_unlocks[{i}]");
        check_id(&p, &r.problem_id)?;
        check_time(&p, r.unlocked_at)?;
        if !(1..=10).contains(&r.tier) {
            return Err(invalid(format!("{p}: tier out of range")));
        }
    }
    for (i, r) in d.opens.iter().enumerate() {
        let p = format!("opens[{i}]");
        check_id(&p, &r.problem_id)?;
        check_time(&p, r.first_opened_at)?;
        check_time(&p, r.last_opened_at)?;
    }
    for (i, r) in d.reviews.iter().enumerate() {
        let p = format!("reviews[{i}]");
        check_id(&p, &r.problem_id)?;
        check_day(&p, &r.due_on)?;
        check_time(&p, r.created_at)?;
        if !crate::practice::REVIEW_INTERVALS.contains(&r.interval_days) || r.passes > 1000 {
            return Err(invalid(format!("{p}: interval or passes out of range")));
        }
    }
    for (i, r) in d.daily_picks.iter().enumerate() {
        let p = format!("daily_picks[{i}]");
        check_id(&p, &r.problem_id)?;
        check_day(&p, &r.day)?;
        if let Some(t) = r.solved_at {
            check_time(&p, t)?;
        }
    }
    for (i, l) in d.lists.iter().enumerate() {
        let p = format!("lists[{i}]");
        let name = l.name.trim();
        if name.is_empty() || name.chars().count() > crate::practice::LIST_NAME_MAX {
            return Err(invalid(format!("{p}: bad list name")));
        }
        check_time(&p, l.created_at)?;
        if l.items.len() > 10_000 {
            return Err(invalid(format!("{p}: too many items")));
        }
        for (j, item) in l.items.iter().enumerate() {
            check_id(&format!("{p}.items[{j}]"), item)?;
        }
    }
    for (i, s) in d.interviews.iter().enumerate() {
        let p = format!("interviews[{i}]");
        check_time(&p, s.started_at)?;
        if let Some(t) = s.ended_at {
            check_time(&p, t)?;
        }
        if !crate::interview::DURATIONS_MIN
            .iter()
            .any(|m| i64::from(*m) * 60 == s.duration_s)
            || !crate::interview::MIXES.contains(&s.mix.as_str())
            || s.problem_ids.len() != 2
            || s.languages.is_empty()
            || s.languages.len() > 4
        {
            return Err(invalid(format!("{p}: bad session")));
        }
        for id in &s.problem_ids {
            check_id(&p, id)?;
        }
        if let Some(r) = &s.results
            && serde_json::from_value::<crate::interview::InterviewResult>(r.clone()).is_err()
        {
            return Err(invalid(format!("{p}: bad scorecard")));
        }
    }
    for (i, s) in d.settings.iter().enumerate() {
        let p = format!("settings[{i}]");
        if !importable_setting(&s.key) {
            return Err(invalid(format!(
                "{p}: setting {:?} cannot be imported",
                s.key
            )));
        }
        check_time(&p, s.updated_at)?;
        if s.value.to_string().len() > 64 * 1024 {
            return Err(invalid(format!("{p}: value too large")));
        }
    }
    Ok(())
}

/// Read and check a file (no database access).
fn read_export(path: &Path) -> Result<(ExportFile, TransferCounts)> {
    let meta =
        std::fs::metadata(path).map_err(|e| invalid(format!("cannot read the file: {e}")))?;
    if meta.len() > IMPORT_MAX_BYTES {
        return Err(invalid(format!(
            "the file is larger than {} MB",
            IMPORT_MAX_BYTES / 1024 / 1024
        )));
    }
    let bytes = std::fs::read(path).map_err(|e| invalid(format!("cannot read the file: {e}")))?;
    let file: ExportFile = serde_json::from_slice(&bytes)
        .map_err(|e| invalid(format!("not a KataBox export: {e}")))?;
    validate(&file)?;
    let c = counts(&file.data);
    Ok((file, c))
}

/// A file read by `import_preview`, waiting for `import_apply`.
#[derive(Debug)]
pub(crate) struct PendingImport {
    id: u32,
    data: ExportData,
}

impl App {
    /// Write every exportable record to `path` (atomically: a temp file, then rename).
    pub async fn export_to(&self, path: &Path) -> Result<TransferCounts> {
        let data = self.store.user.export_all(unexported_setting).await?;
        let c = counts(&data);
        let file = ExportFile {
            format: EXPORT_FORMAT.into(),
            exported_at: self.now_ms(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            data,
        };
        let json = serde_json::to_vec_pretty(&file)
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        let tmp = path.with_extension("kbx.json.tmp");
        let io = |e: std::io::Error| {
            AppError::new(
                AppErrorCode::Internal,
                format!("cannot write the export: {e}"),
            )
        };
        std::fs::write(&tmp, json).map_err(io)?;
        std::fs::rename(&tmp, path).map_err(io)?;
        Ok(c)
    }

    /// Read and validate `path`, keep it pending, and say what it would import.
    pub async fn import_preview_from(&self, path: &Path) -> Result<ImportPreview> {
        let (file, c) = read_export(path)?;
        let conflicts = self.store.user.import_conflicts(&file.data).await?;
        let mut pending = self
            .pending_import
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let id = pending.as_ref().map_or(1, |p| p.id.wrapping_add(1).max(1));
        *pending = Some(PendingImport {
            id,
            data: file.data,
        });
        Ok(ImportPreview {
            id,
            counts: c,
            conflicts: u32::try_from(conflicts).unwrap_or(u32::MAX),
            exported_at: file.exported_at,
            app_version: file.app_version,
        })
    }

    /// Merge the pending import `id` (from `import_preview`).
    pub async fn import_apply(&self, id: u32) -> Result<ImportResult> {
        let data = {
            let mut pending = self
                .pending_import
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            match pending.take() {
                Some(p) if p.id == id => p.data,
                other => {
                    *pending = other;
                    return Err(AppError::new(
                        AppErrorCode::NotFound,
                        "no such pending import",
                    ));
                }
            }
        };
        let stats = self.store.user.import_merge(&data).await?;
        let mut r = ImportResult::default();
        let n = |v: u64| u32::try_from(v).unwrap_or(u32::MAX);
        for s in stats.values() {
            r.added += n(s.added);
            r.updated += n(s.updated);
            r.unchanged += n(s.unchanged);
        }
        Ok(r)
    }

    pub fn import_cancel(&self, id: u32) {
        let mut pending = self
            .pending_import
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if pending.as_ref().is_some_and(|p| p.id == id) {
            *pending = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_rules() {
        for k in ["app.install_id", "judge.calibration", "sandbox_root"] {
            assert!(machine_setting(k), "{k}");
            assert!(!importable_setting(k), "{k}");
        }
        for k in [
            "editor.keymap",
            "workspace.layout",
            "practice.daily",
            "ui_lang",
            "theme",
            "learn.viewed",
            "learn.last",
            "ui.contrast",
            "ui.motion",
        ] {
            assert!(importable_setting(k), "{k}");
            assert!(!unexported_setting(k), "{k}");
        }
        for k in [
            "theme2",
            "anything",
            "learn.layout",
            "learn.speed",
            "learn.other",
            "ui.other",
            crate::window::WINDOW_BOUNDS_KEY,
            "progress.tab",
            "settings.section",
        ] {
            assert!(!importable_setting(k), "{k}");
            assert!(
                unexported_setting(k),
                "{k}: an export never holds what import refuses"
            );
        }
    }
}
