//! The in-app problem editor (expansion M13, ADR 0040). The UI never touches the file
//! system: drafts live in `<local data>/authoring/<id>/` (`draft.json` plus the package it
//! generates in `package/<id>/`), written only here. Saving re-validates with a pack's
//! rules; "Check in sandbox" judges the reference on every test through the ordinary
//! judge path; "Export as pack" writes a `.kbpack` and opens it again with the import
//! checks (ADR 0038) before reporting success.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kb_core::Report;
use kb_pack::authoring;
pub use kb_pack::authoring::{DraftIssue, DraftParam, DraftTest, ProblemDraft};
use kb_pack::kbpack;
pub use kb_pack::kbpack::valid_pack_id;
use serde::{Deserialize, Serialize};

use crate::{App, AppError, AppErrorCode, Result};

const DRAFT_FILE: &str = "draft.json";
const PACKAGE_DIR: &str = "package";
/// The largest `draft.json` read back.
const MAX_DRAFT_BYTES: u64 = 8 * 1024 * 1024;

/// One draft in the editor's list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DraftSummary {
    pub id: String,
    pub title: String,
    /// UTC milliseconds of the last save.
    pub updated_at: i64,
}

/// What a pack made from drafts says about itself (its manifest).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PackHead {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub licence: String,
}

fn bad(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, m.into())
}

fn io(e: std::io::Error) -> AppError {
    AppError::new(AppErrorCode::Internal, format!("authoring folder: {e}"))
}

fn toml_string(s: &str) -> String {
    // TOML basic strings take JSON-style escapes for everything that needs one.
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into())
}

impl App {
    fn authoring_root(&self) -> PathBuf {
        self.local_dir.join("authoring")
    }

    fn draft_dir(&self, id: &str) -> Result<PathBuf> {
        if !authoring::valid_draft_id(id) {
            return Err(bad("not a problem id"));
        }
        Ok(self.authoring_root().join(id))
    }

    fn package_dir(&self, id: &str) -> Result<PathBuf> {
        Ok(self.draft_dir(id)?.join(PACKAGE_DIR).join(id))
    }

    /// Every draft, newest first.
    pub async fn authoring_drafts(&self) -> Result<Vec<DraftSummary>> {
        let mut out = Vec::new();
        let Ok(entries) = std::fs::read_dir(self.authoring_root()) else {
            return Ok(out);
        };
        for e in entries.flatten() {
            let id = e.file_name().to_string_lossy().into_owned();
            if !authoring::valid_draft_id(&id) {
                continue;
            }
            let Ok(d) = self.authoring_load(&id).await else {
                continue;
            };
            let updated_at = std::fs::metadata(e.path().join(DRAFT_FILE))
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX));
            out.push(DraftSummary {
                id,
                title: d.title,
                updated_at,
            });
        }
        out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at).then(a.id.cmp(&b.id)));
        Ok(out)
    }

    pub async fn authoring_load(&self, id: &str) -> Result<ProblemDraft> {
        let file = self.draft_dir(id)?.join(DRAFT_FILE);
        let meta = std::fs::metadata(&file)
            .map_err(|_| AppError::new(AppErrorCode::NotFound, format!("no draft {id}")))?;
        if meta.len() > MAX_DRAFT_BYTES {
            return Err(bad("the draft is larger than 8 MB"));
        }
        let bytes = std::fs::read(&file).map_err(io)?;
        let d: ProblemDraft = serde_json::from_slice(&bytes)
            .map_err(|e| AppError::new(AppErrorCode::Store, format!("unreadable draft: {e}")))?;
        if d.id != id {
            return Err(AppError::new(
                AppErrorCode::Store,
                "the draft's id does not match its folder",
            ));
        }
        Ok(d)
    }

    /// Start a new draft (refused when the id is taken).
    pub async fn authoring_new(&self, id: &str) -> Result<ProblemDraft> {
        let dir = self.draft_dir(id)?;
        if dir.exists() {
            return Err(bad(format!("a draft named {id} already exists")));
        }
        let d = ProblemDraft::new(id);
        self.authoring_save(&d).await?;
        Ok(d)
    }

    /// Save the draft, regenerate its package, and say what still needs fixing.
    pub async fn authoring_save(&self, d: &ProblemDraft) -> Result<Vec<DraftIssue>> {
        let dir = self.draft_dir(&d.id)?;
        let json = serde_json::to_vec_pretty(d)
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        if json.len() as u64 > MAX_DRAFT_BYTES {
            return Err(bad("the draft is larger than 8 MB"));
        }
        let d = d.clone();
        tokio::task::spawn_blocking(move || -> Result<Vec<DraftIssue>> {
            std::fs::create_dir_all(&dir).map_err(io)?;
            let tmp = dir.join("draft.json.tmp");
            std::fs::write(&tmp, json).map_err(io)?;
            std::fs::rename(&tmp, dir.join(DRAFT_FILE)).map_err(io)?;
            // The package is derived: rebuilt from the draft on every save.
            let packages = dir.join(PACKAGE_DIR);
            if packages.exists() {
                std::fs::remove_dir_all(&packages).map_err(io)?;
            }
            let package = packages.join(&d.id);
            authoring::write_package(&d, &package)
                .map_err(|e| AppError::new(AppErrorCode::Internal, e))?;
            Ok(authoring::check_draft(&d, &package))
        })
        .await
        .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?
    }

    /// The issues of a saved draft.
    pub async fn authoring_issues(&self, id: &str) -> Result<Vec<DraftIssue>> {
        let d = self.authoring_load(id).await?;
        let package = self.package_dir(id)?;
        tokio::task::spawn_blocking(move || authoring::check_draft(&d, &package))
            .await
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))
    }

    /// Delete a draft: its folder under `authoring/` only.
    pub async fn authoring_delete(&self, id: &str) -> Result<()> {
        let dir = self.draft_dir(id)?;
        if dir.is_dir() {
            std::fs::remove_dir_all(&dir).map_err(io)?;
        }
        Ok(())
    }

    /// Judge the reference solution on every test, in the sandbox (the ordinary judge
    /// path). The draft must have no issues first.
    pub async fn authoring_check(&self, id: &str) -> Result<Report> {
        let _one = self.judge_slot()?;
        let d = self.authoring_load(id).await?;
        if !self.authoring_issues(id).await?.is_empty() {
            return Err(bad("fix the listed issues first"));
        }
        let problem = kb_judge::Problem::load(&self.package_dir(id)?)?;
        let tests = problem.tests.clone();
        self.judge(problem, tests, d.reference_language, &d.reference, |_| {})
            .await
    }

    /// Write the drafts `ids` as one pack at `path`, then open it with the import checks.
    /// Returns the pack's SHA-256.
    pub async fn authoring_export(
        &self,
        ids: &[String],
        head: &PackHead,
        path: &Path,
    ) -> Result<String> {
        if ids.is_empty() {
            return Err(bad("pick at least one problem"));
        }
        let mut dirs = Vec::new();
        let mut issues = Vec::new();
        for id in ids {
            for i in self.authoring_issues(id).await? {
                issues.push(format!("{id}: {}", i.message));
            }
            dirs.push(self.package_dir(id)?);
        }
        if !issues.is_empty() {
            return Err(AppError::new(AppErrorCode::PackRefused, issues.join("\n")));
        }
        let builtin: BTreeSet<String> = self
            .store
            .content
            .builtin_ids()
            .await?
            .into_iter()
            .collect();
        let work = self
            .authoring_root()
            .join(format!(".export-{}", self.now_ms()));
        let head = head.clone();
        let out = path.to_path_buf();
        let result = tokio::task::spawn_blocking({
            let work = work.clone();
            move || -> std::result::Result<String, Vec<String>> {
                std::fs::create_dir_all(&work).map_err(|e| vec![e.to_string()])?;
                let manifest = work.join("pack.toml");
                let text = format!(
                    "id = {}\nname = {}\nauthor = {}\nversion = {}\nlicence = {}\nmin_app = {}\n",
                    toml_string(&head.id),
                    toml_string(&head.name),
                    toml_string(&head.author),
                    toml_string(&head.version),
                    toml_string(&head.licence),
                    toml_string(env!("CARGO_PKG_VERSION")),
                );
                std::fs::write(&manifest, text).map_err(|e| vec![e.to_string()])?;
                let sha = kbpack::write_pack(&dirs, &manifest, &out).map_err(|e| vec![e])?;
                // Open it exactly as an import would.
                let src = work.join("src");
                let files =
                    kbpack::archive::extract(&out, &src).map_err(|r| vec![r.to_string()])?;
                kbpack::check_extracted(&src, &files, &builtin, env!("CARGO_PKG_VERSION"))?;
                Ok(sha)
            }
        })
        .await
        .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        let _ = std::fs::remove_dir_all(&work);
        match result {
            Ok(sha) => Ok(sha),
            Err(reasons) => {
                let _ = std::fs::remove_file(path);
                Err(AppError::new(
                    AppErrorCode::PackRefused,
                    reasons.into_iter().take(12).collect::<Vec<_>>().join("\n"),
                ))
            }
        }
    }

    /// The starter code for the draft's signature in its reference language.
    pub fn authoring_starter(&self, d: &ProblemDraft) -> Result<String> {
        authoring::starter(d).map_err(bad)
    }

    /// The patterns the editor offers.
    pub fn authoring_patterns(&self) -> Vec<String> {
        authoring::patterns()
    }
}
