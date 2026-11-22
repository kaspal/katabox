//! The KataBox app service (ADR 0011): everything behind the Tauri commands.
//!
//! [`App`] owns the stores (kb-store), the judge environment (kb-judge) and the sandbox
//! root, and implements each command as a plain async method returning
//! `Result<T, AppError>`. `apps/desktop/src-tauri` only resolves paths, holds the `App`
//! and forwards calls, so all of this is testable without a window.
//!
//! - Judging runs on a blocking thread (`spawn_blocking`); one Run or Submit at a time
//!   (a second gets [`AppErrorCode::JudgeBusy`]).
//! - Progress arrives as [`JudgeEvent`]s through a callback (a Tauri channel in the app).
//! - Responses redact what hidden tests returned and printed; user.db keeps the full
//!   Report.

pub mod authoring;
pub mod backup;
mod calibration;
pub mod classroom;
pub mod comforts;
pub mod debug;
pub mod diagnostics;
mod error;
pub mod focus;
pub mod interview;
pub mod journal;
pub mod kata;
pub mod learn_links;
pub mod packs;
pub mod plan;
pub mod plan_app;
pub mod practice;
pub mod profile;
pub mod rating;
pub mod reading;
mod root;
pub mod shrink;
pub mod splash;
pub mod stress;
pub mod transfer;
pub mod uninstall;
pub mod window;

/// The first line of the placeholder THIRD_PARTY_NOTICES.txt that development builds get
/// (`apps/desktop/src-tauri/build.rs`); release builds always carry the real file.
pub const NOTICES_PLACEHOLDER: &str = "KATABOX_NOTICES_PLACEHOLDER";
/// A sanity cap on the notices file (the real one is about 3 MB).
const NOTICES_MAX_BYTES: u64 = 32 * 1024 * 1024;

/// Settings → About → Third-party notices: the bundled text, or `None` in development
/// builds (no file, or the placeholder).
pub fn third_party_notices(path: &std::path::Path) -> Result<Option<String>> {
    let io = |e: std::io::Error| AppError::new(AppErrorCode::Internal, format!("notices: {e}"));
    match std::fs::metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(io(e)),
        Ok(m) if m.len() > NOTICES_MAX_BYTES => {
            return Err(AppError::new(
                AppErrorCode::Internal,
                "notices file too large",
            ));
        }
        Ok(_) => {}
    }
    let text = std::fs::read_to_string(path).map_err(io)?;
    Ok((!text.starts_with(NOTICES_PLACEHOLDER)).then_some(text))
}

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use kb_core::{JudgeEvent, Language, Report, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem};
use kb_store::{
    ContentMeta, Draft, LearnPath, LearnSubject, LearnTopic, LearnTopicSummary, ProblemFilter,
    ProblemSummary, ProblemView, Snapshot, Store, SubmissionSummary, UiLang,
};
use serde::Serialize;
use serde_json::{Map, Value};

pub use calibration::{CALIBRATION_SETTING, CalibrationInfo};
pub use diagnostics::{Diagnostics, RuntimeDiag};
pub use error::{AppError, AppErrorCode};
pub use learn_links::{PatternTopics, TopicReview};
pub use root::{RootSource, SANDBOX_ROOT_ENV, SANDBOX_ROOT_SETTING};
pub use shrink::{ShrinkEvent, ShrinkResult, ShrinkStop};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// Where the app keeps its files; resolved by the Tauri shell.
#[derive(Debug, Clone)]
pub struct AppPaths {
    /// The bundled, read-only content.db.
    pub content_db: PathBuf,
    /// user.db in the app data folder (roaming on Windows).
    pub user_db: PathBuf,
    /// The app's local data folder; the default sandbox root is `<this>/sandbox`.
    pub local_data_dir: PathBuf,
    /// Development only: the fetch script's root (`target/katabox`), used when it exists
    /// and nothing else is configured.
    pub dev_root: Option<PathBuf>,
    /// The folders above are the development ones (debug builds), not the user's real
    /// data. Shown as "DEV DATA" in the UI.
    pub dev_data: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ToolchainInfo {
    pub language: Language,
    pub present: bool,
    /// The program the judge starts (the compiler for Rust), when present.
    pub program: Option<String>,
    /// Why it is missing, when not present.
    pub problem: Option<String>,
}

/// Whether a language server can run for a language here, and if not, why.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LanguageServerInfo {
    pub language: Language,
    pub available: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AppInfo {
    pub app_version: String,
    /// Development data folders are in use (debug builds).
    pub dev_data: bool,
    /// Folder holding user.db.
    pub data_dir: String,
    pub content: ContentMeta,
    pub user_schema_version: i64,
    pub sandbox_root: String,
    pub sandbox_root_source: RootSource,
    /// Languages the judge supports, with their toolchain status.
    pub toolchains: Vec<ToolchainInfo>,
    /// Language servers (completion, hover, diagnostics), per judge language.
    pub language_servers: Vec<LanguageServerInfo>,
    /// `windows`, `linux`, ...
    pub platform: String,
}

/// What the first-run screen and Settings → Languages show.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LanguagesInfo {
    /// Python, JavaScript/TypeScript, PHP, Rust: each a runtime plus its language server.
    pub languages: Vec<kb_toolchain::LanguageStatus>,
    /// Free bytes on the sandbox root's volume, when known.
    pub free_bytes: Option<u64>,
    /// KataBox can install languages on this OS and CPU.
    pub platform_supported: bool,
}

/// The draft to continue on Home: the most recently edited one whose problem exists.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ContinueItem {
    pub problem_id: String,
    /// Title in the requested UI language (English fallback).
    pub title: String,
    pub language: Language,
    pub updated_at: i64,
}

/// Failed submissions after which the editorial unlocks.
pub const EDITORIAL_FAILURES: u32 = 3;

/// Why the editorial is unlocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum EditorialReason {
    /// The problem was solved.
    Accepted,
    /// Enough submissions failed.
    Attempts,
}

/// A Learn topic page (ADR 0029): the topic, and its linked problems in display order
/// (ids the catalogue no longer has are left out).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnTopicView {
    pub topic: LearnTopic,
    pub problems: Vec<ProblemSummary>,
}

/// The editorial tab's state.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EditorialState {
    pub unlocked: bool,
    pub reason: Option<EditorialReason>,
    /// Non-Accepted submissions so far.
    pub failed_submits: u32,
    /// Failed submissions needed to unlock without solving.
    pub required_failures: u32,
    /// The problem has an editorial at all.
    pub available: bool,
    /// The editorial (Markdown), only when unlocked.
    pub text: Option<String>,
}

/// A submission as the UI sees it: hidden tests redacted.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubmissionView {
    #[serde(flatten)]
    pub summary: SubmissionSummary,
    pub code: String,
    pub report: Report,
}

/// The result of Submit: the stored submission's id and its (redacted) report.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubmitResult {
    pub submission_id: i64,
    pub report: Report,
    /// Reviews, the daily problem, the streak and what to solve next (ADR 0019).
    pub practice: practice::PracticeOutcome,
    /// A kata Submit (expansion M1): the attempt, and whether it beat the best time.
    /// Such a Submit stores no submission (`submission_id` is 0).
    pub kata: Option<kata::KataOutcome>,
}

/// The practice context of a Submit (ADR 0019).
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubmitOptions {
    /// The user's local date, `YYYY-MM-DD`.
    pub today: String,
    /// Submitted from review mode: the result moves the review schedule.
    pub review: bool,
    /// Language of the suggested next problem's title.
    pub lang: UiLang,
    /// Submitted in the running kata that started at this time (UTC ms): recorded as a
    /// kata attempt only (expansion M1).
    pub kata: Option<i64>,
}

/// A user-written test input for Run: argument values keyed by parameter name.
pub type CustomInput = Map<String, Value>;

/// What "Check" found for each custom input (expansion M9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CustomCheck {
    /// Per input: `None` when it is valid, else why not.
    pub problems: Vec<Option<String>>,
    /// The problem's validate.py ran (it needs the Python runtime).
    pub constraints_checked: bool,
}

#[derive(Debug)]
pub struct App {
    store: Store,
    /// Built from kb-toolchain's receipts; rebuilt after every install or uninstall.
    env: RwLock<JudgeEnv>,
    /// Installs languages into the sandbox root (ADR 0015).
    toolchains: Arc<kb_toolchain::Manager>,
    root: PathBuf,
    root_source: RootSource,
    dev_data: bool,
    data_dir: PathBuf,
    judging: tokio::sync::Mutex<()>,
    /// Language servers (ADR 0014): tools from the sandbox root, projects and state in
    /// the app's local data folder.
    lsp: kb_bridge::LspManager,
    /// This machine's calibration (ADR 0017), from user.db; `None` until measured.
    calibration: RwLock<Option<CalibrationInfo>>,
    /// Tests only: a fixed "now" (UTC ms) for interview timing.
    clock: std::sync::Mutex<Option<i64>>,
    /// An import read by `import_preview`, waiting for `import_apply` (ADR 0020).
    pending_import: std::sync::Mutex<Option<transfer::PendingImport>>,
    /// The debug session (ADR 0022): a recorded trace and the debugger's position in it.
    debug: std::sync::Mutex<Option<debug::Session>>,
    /// Stop requested for the running shrink (ADR 0030).
    shrink_cancel: std::sync::atomic::AtomicBool,
    stress_cancel: std::sync::atomic::AtomicBool,
    /// The app's local data folder (packs live under it; expansion M11).
    local_dir: PathBuf,
    pending_pack: std::sync::Mutex<Option<packs::PendingPack>>,
    /// The skill ratings replayed from the history (expansion M2); dropped on every
    /// Submit, kata Submit and import.
    ratings: std::sync::Mutex<Option<Arc<rating::Ratings>>>,
}

/// Solved problems per judge language (cross-language practice, ADR 0020).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LanguageCount {
    pub language: Language,
    pub solved: u32,
}

impl App {
    /// Open both stores and pick the sandbox root (see [`root`] for the order).
    pub async fn open(paths: &AppPaths) -> Result<App> {
        let mut store = Store::open(&paths.content_db, &paths.user_db).await?;
        // Installed packs (expansion M11) are merged into the content the app reads.
        packs::open_content(&mut store, &paths.content_db, &paths.local_data_dir).await?;
        let setting = store
            .user
            .get_setting(SANDBOX_ROOT_SETTING)
            .await?
            .and_then(|v| v.as_str().map(PathBuf::from));
        let (root, root_source) = root::resolve(
            std::env::var_os(SANDBOX_ROOT_ENV).map(PathBuf::from),
            setting,
            paths.dev_root.as_deref(),
            &paths.local_data_dir,
        )?;
        // A stored calibration that no longer parses is measured again.
        let calibration = store
            .user
            .get_setting(CALIBRATION_SETTING)
            .await?
            .and_then(|v| serde_json::from_value::<CalibrationInfo>(v).ok());
        let toolchains = Arc::new(kb_toolchain::Manager::open(&root)?);
        // Leftovers of an interrupted install go first (skipped while the CLI installs).
        match toolchains.purge() {
            Ok(()) | Err(kb_toolchain::Error::Busy) => {}
            Err(e) => return Err(e.into()),
        }
        let installed = toolchains.clone();
        Ok(App {
            lsp: kb_bridge::LspManager::new(&root, &paths.local_data_dir)
                .with_installed(Arc::new(move |l| installed.servers_installed(l))),
            env: RwLock::new(judge_env(&toolchains)),
            toolchains,
            store,
            root,
            root_source,
            dev_data: paths.dev_data,
            data_dir: paths
                .user_db
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default(),
            judging: tokio::sync::Mutex::new(()),
            shrink_cancel: std::sync::atomic::AtomicBool::new(false),
            stress_cancel: std::sync::atomic::AtomicBool::new(false),
            local_dir: paths.local_data_dir.clone(),
            pending_pack: std::sync::Mutex::new(None),
            calibration: RwLock::new(calibration),
            clock: std::sync::Mutex::new(None),
            pending_import: std::sync::Mutex::new(None),
            ratings: std::sync::Mutex::new(None),
            debug: std::sync::Mutex::new(None),
        })
    }

    /// "Now" in UTC milliseconds (wall clock; tests may fix it).
    pub(crate) fn now_ms(&self) -> i64 {
        if let Some(t) = *self.clock.lock().unwrap_or_else(|e| e.into_inner()) {
            return t;
        }
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
    }

    /// Tests only: fix the clock (`None`: the wall clock again).
    #[doc(hidden)]
    pub fn set_clock(&self, ms: Option<i64>) {
        *self.clock.lock().unwrap_or_else(|e| e.into_inner()) = ms;
    }

    pub fn sandbox_root(&self) -> &Path {
        &self.root
    }

    pub async fn close(&self) {
        self.store.close().await;
    }

    pub async fn app_info(&self) -> Result<AppInfo> {
        let env = self.env();
        let toolchains = kb_judge::languages::SUPPORTED
            .into_iter()
            .map(|language| match env.toolchain_program(language) {
                Ok(p) if self.toolchains.runtime_installed(language) => ToolchainInfo {
                    language,
                    present: true,
                    program: Some(p.display().to_string()),
                    problem: None,
                },
                Ok(_) => ToolchainInfo {
                    language,
                    present: false,
                    program: None,
                    problem: Some("not installed".into()),
                },
                Err(e) => ToolchainInfo {
                    language,
                    present: false,
                    program: None,
                    problem: Some(if self.toolchains.runtime_installed(language) {
                        e.to_string()
                    } else {
                        "not installed".into()
                    }),
                },
            })
            .collect();
        Ok(AppInfo {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            dev_data: self.dev_data,
            data_dir: self.data_dir.display().to_string(),
            content: self.store.content.meta().await?,
            user_schema_version: self.store.user.schema_version().await?,
            sandbox_root: self.root.display().to_string(),
            sandbox_root_source: self.root_source,
            toolchains,
            language_servers: kb_judge::languages::SUPPORTED
                .into_iter()
                .map(|language| {
                    let reason = self.lsp.unavailable_reason(language);
                    LanguageServerInfo {
                        language,
                        available: reason.is_none(),
                        reason,
                    }
                })
                .collect(),
            platform: std::env::consts::OS.to_string(),
        })
    }

    /// The judge environment, at this machine's time scale.
    fn env(&self) -> JudgeEnv {
        let mut env = self.env.read().unwrap_or_else(|e| e.into_inner()).clone();
        env.time_scale = calibration::scale_of(self.calibration().as_ref());
        env
    }

    fn refresh_env(&self) {
        *self.env.write().unwrap_or_else(|e| e.into_inner()) = judge_env(&self.toolchains);
    }

    // --- Languages (ADR 0015) ---------------------------------------------------------

    pub fn languages(&self) -> LanguagesInfo {
        LanguagesInfo {
            languages: self.toolchains.status(),
            free_bytes: self.toolchains.free_space(),
            platform_supported: self.toolchains.platform().is_some(),
        }
    }

    /// Download and install `languages` (runtime and language server each; Node comes
    /// with TypeScript and Python's server). Progress goes to `on_event`. A failed or
    /// cancelled install keeps its partial downloads, so installing again resumes.
    pub async fn install_languages<F>(&self, languages: &[Language], on_event: F) -> Result<()>
    where
        F: Fn(kb_toolchain::InstallEvent) + Send + Sync,
    {
        let downloader = kb_toolchain::Downloader::system()?;
        let result = self
            .toolchains
            .install(
                languages,
                kb_toolchain::InstallOptions::default(),
                &downloader,
                &on_event,
            )
            .await;
        self.refresh_env();
        result?;
        // The first runtime installed is the first chance to measure this machine; skipped
        // while a Run or Submit holds the judge (the next one measures instead).
        if let Ok(_one) = self.judge_slot() {
            self.ensure_calibrated().await;
        }
        Ok(())
    }

    pub fn cancel_install(&self) {
        self.toolchains.cancel();
    }

    /// Remove a language (Node stays while another installed language needs it). Refused
    /// with `judge_busy` while a Run or Submit is in progress.
    pub async fn uninstall_language(&self, language: Language) -> Result<()> {
        let _judge = self.judge_slot()?;
        // 1. Receipts go first: from now on nothing counts the language as installed.
        let u = self.toolchains.uninstall_prepare(language)?;
        self.refresh_env();
        // 2. Stop the language servers whose files are about to go (Windows file locks).
        for l in kb_judge::languages::SUPPORTED {
            if !self.toolchains.servers_installed(l) {
                self.lsp.stop_language(l).await;
            }
        }
        // 3. Then the folders.
        self.toolchains.uninstall_finish(u)?;
        Ok(())
    }

    // --- Problems -------------------------------------------------------------------

    pub async fn list_problems(&self, filter: &ProblemFilter) -> Result<Vec<ProblemSummary>> {
        Ok(self.store.list_problems(filter).await?)
    }

    /// Every Learn subject with its topics (ADR 0029).
    pub async fn learn_catalog(&self, lang: UiLang) -> Result<Vec<LearnSubject>> {
        Ok(self.store.content.learn_catalog(lang).await?)
    }

    /// The learning paths on Learn home (Phase 6).
    pub async fn learn_paths(&self, lang: UiLang) -> Result<Vec<LearnPath>> {
        Ok(self.store.content.learn_paths(lang).await?)
    }

    /// A Learn topic page with its linked problems (status included).
    pub async fn learn_topic(&self, id: &str, lang: UiLang) -> Result<LearnTopicView> {
        let topic = self.store.content.learn_topic(id, lang).await?;
        let all = self
            .store
            .list_problems(&ProblemFilter {
                lang: Some(lang),
                ..Default::default()
            })
            .await?;
        let problems = topic
            .problems
            .iter()
            .filter_map(|id| all.iter().find(|p| &p.id == id).cloned())
            .collect();
        Ok(LearnTopicView { topic, problems })
    }

    /// Learn topics matching `query` in either language.
    pub async fn search_learn(&self, query: &str, lang: UiLang) -> Result<Vec<LearnTopicSummary>> {
        Ok(self.store.content.search_learn(query, lang).await?)
    }

    /// The problem page. The editorial is left out on purpose: it is locked until the
    /// problem is solved or after enough failed submissions; see [`App::editorial`].
    pub async fn get_problem(&self, id: &str, lang: UiLang) -> Result<ProblemView> {
        let mut view = self.store.content.get_problem(id, lang).await?;
        view.editorial = None;
        // Mock interview (ADR 0020) and kata (expansion M1): hint texts never reach the UI.
        // So is an assignment without hints (expansion M12).
        if self.in_interview(id).await?
            || self.in_kata(id).await?
            || self.assignment_locks_help(id).await?
        {
            view.hints.clear();
        }
        // Follow-ups (expansion M5) open once their base is solved.
        if !view.followups.is_empty() {
            let solved = self
                .store
                .user
                .problem_status(id)
                .await?
                .is_some_and(|s| s.status == kb_store::ProblemStatus::Solved);
            for f in &mut view.followups {
                f.unlocked = solved;
            }
        }
        view.time_scale = calibration::scale_of(self.calibration().as_ref());
        view.time_limits = calibration::time_limits(view.time_limit_ms, view.time_scale);
        Ok(view)
    }

    /// The editorial and whether it is unlocked: once the problem is solved, or after
    /// [`EDITORIAL_FAILURES`] non-Accepted submissions (ADR 0013).
    pub async fn editorial(&self, id: &str, lang: UiLang) -> Result<EditorialState> {
        self.ensure_unlocked(id).await?;
        let view = self.store.content.get_problem(id, lang).await?;
        let subs = self.store.user.list_submissions(id).await?;
        let solved = subs.iter().any(|s| s.verdict.is_accepted());
        let failed = u32::try_from(subs.iter().filter(|s| !s.verdict.is_accepted()).count())
            .unwrap_or(u32::MAX);
        let reason = if solved {
            Some(EditorialReason::Accepted)
        } else if failed >= EDITORIAL_FAILURES {
            Some(EditorialReason::Attempts)
        } else {
            None
        };
        Ok(EditorialState {
            unlocked: reason.is_some(),
            reason,
            failed_submits: failed,
            required_failures: EDITORIAL_FAILURES,
            available: view.editorial.is_some(),
            text: reason.and(view.editorial),
        })
    }

    /// The language of the most recently edited draft of `problem`, if any.
    pub async fn last_language(&self, problem: &str) -> Result<Option<Language>> {
        Ok(self
            .store
            .user
            .latest_draft_for(problem)
            .await?
            .map(|d| d.language))
    }

    /// The starter code for `problem` in `language`.
    pub async fn get_stub(&self, problem: &str, language: Language) -> Result<String> {
        let p = self.problem(problem).await?;
        ensure_language_fits(&p, language)?;
        // An SQL problem starts from one SELECT over its first table (ADR 0031).
        if let Some(schema) = &p.sql {
            return Ok(kb_judge::sql::starter(schema));
        }
        // A program problem starts from a whole program reading stdin (ADR 0028).
        if p.mode == kb_core::ProblemMode::Program {
            return Ok(kb_judge::program_starter(language)?);
        }
        // A fix-the-bug problem starts from its buggy code (expansion M6); a language
        // without a buggy version gets the ordinary starter.
        if let Some(buggy) = self.store.content.starter(problem, language).await? {
            return Ok(buggy);
        }
        Ok(kb_judge::starter_code(language, &p.signature)?)
    }

    async fn problem(&self, id: &str) -> Result<Problem> {
        let d = self.store.content.judge_data(id).await?;
        if d.mode == kb_core::ProblemMode::Sql {
            let schema = d.schema_sql.as_deref().ok_or_else(|| {
                AppError::new(AppErrorCode::Store, format!("{id} has no schema.sql"))
            })?;
            return Ok(Problem::from_sql_parts(
                &d.id,
                schema,
                &d.checker,
                d.time_limit_ms,
                d.memory_mb,
                d.tests,
            )?);
        }
        Ok(Problem::from_parts(
            &d.id,
            d.mode,
            &d.signature,
            &d.checker,
            d.time_limit_ms,
            d.memory_mb,
            d.tests,
        )?)
    }

    // --- Drafts ---------------------------------------------------------------------

    pub async fn load_draft(&self, problem: &str, language: Language) -> Result<Option<Draft>> {
        Ok(self.store.user.load_draft(problem, language).await?)
    }

    pub async fn save_draft(&self, problem: &str, language: Language, code: &str) -> Result<()> {
        // A kata re-solves from memory: its code never becomes a draft (expansion M1).
        if self.in_kata(problem).await? {
            return Ok(());
        }
        Ok(self.store.user.save_draft(problem, language, code).await?)
    }

    /// The most recently edited draft whose problem is still in content.db.
    pub async fn continue_item(&self, lang: UiLang) -> Result<Option<ContinueItem>> {
        for d in self.store.user.recent_drafts(20).await? {
            match self.store.content.get_problem(&d.problem_id, lang).await {
                Ok(p) => {
                    return Ok(Some(ContinueItem {
                        problem_id: d.problem_id,
                        title: p.title,
                        language: d.language,
                        updated_at: d.updated_at,
                    }));
                }
                Err(kb_store::StoreError::NotFound(_)) => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(None)
    }

    // --- Run and Submit -------------------------------------------------------------

    /// Judge `code` on the public tests plus `custom` inputs, and snapshot it. The
    /// expected output of each custom input comes from the reference solution in the
    /// same language, run first.
    pub async fn run<F>(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        custom: Vec<CustomInput>,
        on_event: F,
    ) -> Result<Report>
    where
        F: FnMut(JudgeEvent) + Send + 'static,
    {
        let _one = self.judge_slot()?;
        self.ensure_interview_language(problem, language).await?;
        self.ensure_calibrated().await;
        let p = self.problem(problem).await?;
        ensure_language_fits(&p, language)?;
        if p.mode == kb_core::ProblemMode::Sql && !custom.is_empty() {
            return Err(AppError::new(
                AppErrorCode::InvalidCustomInput,
                "custom tests are not available for SQL problems",
            ));
        }
        let mut tests: Vec<TestCase> = p.public_tests().cloned().collect();
        if !custom.is_empty() {
            tests.extend(self.custom_tests(&p, language, custom).await?);
        }
        let report = self.judge(p, tests, language, code, on_event).await?;
        // Kata runs leave no History snapshot (expansion M1).
        if !self.in_kata(problem).await? {
            self.store
                .user
                .record_run(problem, language, code, report.verdict)
                .await?;
        }
        Ok(report)
    }

    /// Judge `code` on every test, record the submission (with the full report) and
    /// return it with hidden tests redacted.
    pub async fn submit<F>(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        opts: &SubmitOptions,
        on_event: F,
    ) -> Result<SubmitResult>
    where
        F: FnMut(JudgeEvent) + Send + 'static,
    {
        let today = practice::parse_day(&opts.today)?;
        let _one = self.judge_slot()?;
        self.ensure_interview_language(problem, language).await?;
        self.ensure_calibrated().await;
        let p = self.problem(problem).await?;
        ensure_language_fits(&p, language)?;
        let kata = match opts.kata {
            Some(session) => Some(self.kata_for(session, problem).await?),
            None => None,
        };
        let tests = p.tests.clone();
        let mut report = self.judge(p, tests, language, code, on_event).await?;
        if let Some(s) = kata {
            // A kata Submit is kept apart: no submission, status, review or daily change.
            let outcome = self
                .record_kata(s.started_at, problem, language, code, report.verdict)
                .await?;
            report.redact_hidden();
            return Ok(SubmitResult {
                submission_id: 0,
                report,
                practice: practice::PracticeOutcome::default(),
                kata: Some(outcome),
            });
        }
        let was_solved = self
            .store
            .user
            .problem_status(problem)
            .await?
            .is_some_and(|s| s.status == kb_store::ProblemStatus::Solved);
        let submission_id = self
            .store
            .user
            .record_submission(problem, language, code, &report)
            .await?;
        self.forget_ratings();
        if !report.verdict.is_accepted() {
            // The mistake journal (expansion M3): tag the failure now.
            self.tag_new_failures().await?;
        }
        let practice = self
            .after_submit(
                problem,
                report.verdict,
                was_solved,
                today,
                opts.review,
                opts.lang,
            )
            .await?;
        report.redact_hidden();
        Ok(SubmitResult {
            submission_id,
            report,
            practice,
            kata: None,
        })
    }

    fn judge_slot(&self) -> Result<tokio::sync::MutexGuard<'_, ()>> {
        self.judging.try_lock().map_err(|_| {
            AppError::new(
                AppErrorCode::JudgeBusy,
                "a Run or Submit is already in progress",
            )
        })
    }

    /// Run the judge on a blocking thread.
    async fn judge<F>(
        &self,
        problem: Problem,
        tests: Vec<TestCase>,
        language: Language,
        code: &str,
        mut on_event: F,
    ) -> Result<Report>
    where
        F: FnMut(JudgeEvent) + Send + 'static,
    {
        let env = self.env();
        let code = code.to_string();
        let report = tokio::task::spawn_blocking(move || {
            if problem.mode == kb_core::ProblemMode::Sql {
                // One SELECT over the problem's tables (ADR 0031).
                kb_judge::sql::judge_sql(&problem, &tests, &code, &env, &mut on_event)
            } else {
                kb_judge::judge_with_progress(
                    &problem,
                    &tests,
                    language,
                    &code,
                    &env,
                    &mut on_event,
                )
            }
        })
        .await??;
        Ok(report)
    }

    /// Turn custom inputs into tests whose expected outputs come from the reference
    /// solution in `language`.
    /// Check custom inputs without running any solution (expansion M9's "Check"): the
    /// signature's types, then the problem's validate.py in the sandbox (one run) when the
    /// Python runtime is installed. One entry per input: `None` when valid, else why not.
    pub async fn check_custom(
        &self,
        problem: &str,
        inputs: Vec<CustomInput>,
    ) -> Result<CustomCheck> {
        let _one = self.judge_slot()?;
        if inputs.len() > 32 {
            return Err(AppError::new(AppErrorCode::InvalidInput, "too many inputs"));
        }
        let p = self.problem(problem).await?;
        let mut problems: Vec<Option<String>> = inputs
            .iter()
            .map(|x| kb_judge::check_input(x, &p.signature).err())
            .collect();
        let python = self.toolchains.runtime_installed(Language::Python);
        let validator = if python {
            self.store.content.validator(&p.id).await?
        } else {
            None
        };
        let constraints_checked = validator.is_some();
        if let Some(v) = validator {
            let typed: Vec<(usize, Map<String, Value>)> = inputs
                .into_iter()
                .enumerate()
                .filter(|(i, _)| problems[*i].is_none())
                .collect();
            if !typed.is_empty() {
                let env = self.env();
                let given: Vec<Map<String, Value>> = typed.iter().map(|(_, x)| x.clone()).collect();
                let answers = tokio::task::spawn_blocking(move || {
                    let refs: Vec<&Map<String, Value>> = given.iter().collect();
                    let limits = kb_judge::ScriptLimits {
                        cpu_time_ms: 10_000,
                        memory_mb: 512,
                        max_output_bytes: 1024 * 1024,
                    };
                    kb_judge::validate_inputs(&env, &v, &refs, limits)
                })
                .await??;
                for ((i, _), a) in typed.iter().zip(answers) {
                    problems[*i] = a;
                }
            }
        }
        Ok(CustomCheck {
            problems,
            constraints_checked,
        })
    }

    async fn custom_tests(
        &self,
        p: &Problem,
        language: Language,
        custom: Vec<CustomInput>,
    ) -> Result<Vec<TestCase>> {
        let invalid = |i: usize, why: String| {
            AppError::new(
                AppErrorCode::InvalidCustomInput,
                format!("custom input {}: {why}", i + 1),
            )
        };
        let mut tests = Vec::with_capacity(custom.len());
        for (i, input) in custom.into_iter().enumerate() {
            kb_judge::check_input(&input, &p.signature).map_err(|e| invalid(i, e))?;
            tests.push(TestCase {
                kind: TestKind::Custom,
                index: u32::try_from(i + 1).unwrap_or(u32::MAX),
                input,
                expected: Value::Null,
            });
        }
        // The problem's constraints (validate.py, ADR 0016), run in the sandbox. It needs
        // Python; without it the reference below still rejects inputs it cannot handle.
        if let Some(validator) = self.store.content.validator(&p.id).await?
            && self.toolchains.runtime_installed(Language::Python)
        {
            let env = self.env();
            let inputs: Vec<Map<String, Value>> = tests.iter().map(|t| t.input.clone()).collect();
            let answers = tokio::task::spawn_blocking(move || {
                let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
                let limits = kb_judge::ScriptLimits {
                    cpu_time_ms: 10_000,
                    memory_mb: 512,
                    max_output_bytes: 1024 * 1024,
                };
                kb_judge::validate_inputs(&env, &validator, &refs, limits)
            })
            .await??;
            if let Some((i, msg)) = answers
                .iter()
                .enumerate()
                .find_map(|(i, a)| a.as_ref().map(|m| (i, m)))
            {
                return Err(invalid(i, msg.clone()));
            }
        }
        let reference = self
            .store
            .content
            .reference_solution(&p.id, language)
            .await?
            .ok_or_else(|| {
                AppError::new(
                    AppErrorCode::UnsupportedLanguage,
                    format!("{} has no {language} reference solution", p.id),
                )
            })?;
        let report = self
            .judge(p.clone(), tests.clone(), language, &reference, |_| {})
            .await?;
        if report.verdict == Verdict::CompilationError {
            return Err(AppError::new(
                AppErrorCode::JudgeInternal,
                format!("the {language} reference solution does not compile"),
            ));
        }
        for (i, t) in tests.iter_mut().enumerate() {
            // The reference returned normally: its value is the expected output (the
            // checker compared it with null, so the verdict itself is meaningless).
            let outcome = report.tests.get(i);
            match outcome {
                // A program's output kept in the report is cut at 64 KB (ADR 0028): too
                // long to be a custom test's expected output.
                Some(o)
                    if o.actual
                        .as_ref()
                        .and_then(Value::as_str)
                        .is_some_and(kb_judge::program::was_truncated) =>
                {
                    return Err(invalid(
                        i,
                        "the expected output is too long for a custom test (over 64 KB)".into(),
                    ));
                }
                Some(o)
                    if matches!(o.result.verdict, Verdict::Accepted | Verdict::WrongAnswer)
                        && o.actual.is_some() =>
                {
                    t.expected = o.actual.clone().unwrap_or(Value::Null);
                }
                Some(o) => {
                    let why = o
                        .result
                        .message
                        .as_ref()
                        .map(|m| m.text.clone())
                        .unwrap_or_else(|| format!("{:?}", o.result.verdict));
                    return Err(invalid(
                        i,
                        format!("the reference solution could not handle it: {why}"),
                    ));
                }
                None => {
                    return Err(invalid(
                        i,
                        "the reference solution stopped before reaching it".into(),
                    ));
                }
            }
        }
        Ok(tests)
    }

    // --- Language servers (ADR 0014) ------------------------------------------------

    /// Start a language server for `language` on `problem`, replacing the language's
    /// previous session; `text` seeds the solution file. Messages and status changes go
    /// to `on_event`.
    pub async fn lsp_connect<F>(
        &self,
        problem: &str,
        language: Language,
        text: &str,
        on_event: F,
    ) -> Result<kb_bridge::LspConnection>
    where
        F: Fn(kb_bridge::LspEvent) + Send + Sync + 'static,
    {
        // The bridge validates the id; an unknown problem is simply not a program one.
        let program = matches!(
            self.store.content.problem_mode(problem).await,
            Ok(kb_core::ProblemMode::Program)
        );
        Ok(self
            .lsp
            .connect_with(
                problem,
                language,
                text,
                program,
                std::sync::Arc::new(on_event),
            )
            .await?)
    }

    /// Relay a JSON-RPC message from the editor's client of `session` to the server
    /// (through the bridge's policy).
    pub async fn lsp_send(&self, language: Language, session: u32, message: String) -> Result<()> {
        Ok(self.lsp.send(language, session, message).await?)
    }

    /// Stop `session` (the editor closed); a no-op once a newer connect replaced it.
    pub async fn lsp_disconnect(&self, language: Language, session: u32) {
        self.lsp.disconnect(language, session).await;
    }

    /// Stop every language server (app exit).
    pub async fn lsp_shutdown(&self) {
        self.lsp.shutdown().await;
    }

    // --- Submissions, history, notes, hints, settings ------------------------------

    pub async fn list_submissions(&self, problem: &str) -> Result<Vec<SubmissionSummary>> {
        self.ensure_unlocked(problem).await?;
        Ok(self.store.user.list_submissions(problem).await?)
    }

    pub async fn get_submission(&self, id: i64) -> Result<SubmissionView> {
        let s = self.store.user.get_submission(id).await?;
        self.ensure_unlocked(&s.summary.problem_id).await?;
        let mut report: Report = serde_json::from_value(s.report).map_err(|e| {
            AppError::new(
                AppErrorCode::Store,
                format!("submission {id} has an unreadable report: {e}"),
            )
        })?;
        report.redact_hidden();
        Ok(SubmissionView {
            summary: s.summary,
            code: s.code,
            report,
        })
    }

    pub async fn list_snapshots(&self, problem: &str) -> Result<Vec<Snapshot>> {
        self.ensure_unlocked(problem).await?;
        Ok(self.store.user.list_snapshots(problem).await?)
    }

    pub async fn restore_snapshot(&self, id: i64) -> Result<Snapshot> {
        if let Some(problem) = self.store.user.snapshot_problem(id).await? {
            self.ensure_unlocked(&problem).await?;
        }
        Ok(self.store.user.restore_snapshot(id).await?)
    }

    // --- Cross-language practice (ADR 0020) ----------------------------------------

    /// The latest Accepted submission of `problem` in `language` (locked in interviews).
    pub async fn accepted_solution(
        &self,
        problem: &str,
        language: Language,
    ) -> Result<Option<kb_store::AcceptedSolution>> {
        self.ensure_unlocked(problem).await?;
        Ok(self.store.user.accepted_solution(problem, language).await?)
    }

    /// Languages `problem` was solved in.
    pub async fn solved_languages(&self, problem: &str) -> Result<Vec<Language>> {
        Ok(self.store.user.solved_languages(problem).await?)
    }

    /// Distinct problems solved per judge language (zeros included).
    pub async fn solved_by_language(&self) -> Result<Vec<LanguageCount>> {
        let by = self.store.user.solved_by_language().await?;
        Ok(kb_judge::languages::SUPPORTED
            .into_iter()
            .map(|language| LanguageCount {
                language,
                solved: by
                    .iter()
                    .find(|(l, _)| *l == language)
                    .map_or(0, |(_, ps)| u32::try_from(ps.len()).unwrap_or(u32::MAX)),
            })
            .collect())
    }

    /// In a mock interview, only the session's languages may be judged on its problems;
    /// in a running assignment, only its languages (expansion M12).
    async fn ensure_interview_language(&self, problem: &str, language: Language) -> Result<()> {
        self.ensure_assignment_language(problem, language).await?;
        match self.running_session().await? {
            Some(s)
                if s.problem_ids.iter().any(|p| p == problem)
                    && !s.languages.contains(&language) =>
            {
                Err(AppError::new(
                    AppErrorCode::InvalidInput,
                    format!("{language} is not one of this mock interview's languages"),
                ))
            }
            _ => Ok(()),
        }
    }

    pub async fn get_note(&self, problem: &str) -> Result<Option<String>> {
        // A kata re-solves from memory: the notes wait until it ends (expansion M1).
        if self.in_kata(problem).await? {
            return Err(kata::locked());
        }
        Ok(self.store.user.get_note(problem).await?)
    }

    pub async fn set_note(&self, problem: &str, body: &str) -> Result<()> {
        Ok(self.store.user.set_note(problem, body).await?)
    }

    /// Reveal hint `tier` (1-based); returns every unlocked tier.
    pub async fn unlock_hint(&self, problem: &str, tier: u32) -> Result<Vec<u32>> {
        self.ensure_unlocked(problem).await?;
        if tier == 0 {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "hint tiers start at 1",
            ));
        }
        self.store.user.unlock_hint(problem, tier).await?;
        self.unlocked_hints(problem).await
    }

    pub async fn unlocked_hints(&self, problem: &str) -> Result<Vec<u32>> {
        self.ensure_unlocked(problem).await?;
        Ok(self.store.user.unlocked_hints(problem).await?)
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.store.user.get_setting(key).await?)
    }

    pub async fn set_setting(&self, key: &str, value: &Value) -> Result<()> {
        Ok(self.store.user.set_setting(key, value).await?)
    }
}

/// The judge's runtimes: only languages whose runtime has a valid install receipt.
fn judge_env(toolchains: &kb_toolchain::Manager) -> JudgeEnv {
    let mut env = JudgeEnv::new(toolchains.root());
    let manifest = kb_toolchain::Manifest::builtin();
    for language in kb_judge::languages::SUPPORTED {
        if !toolchains.runtime_installed(language) {
            continue;
        }
        let dir = manifest
            .language(language)
            .and_then(|u| u.runtime.first())
            .and_then(|id| toolchains.layout().component_dir(id));
        if let Some(dir) = dir {
            env = env.with_toolchain(language, dir);
        }
    }
    env
}

#[cfg(test)]
pub(crate) mod testutil;

#[cfg(test)]
mod tests;

/// SQL problems take only SQL, and SQL is only for SQL problems (ADR 0031).
fn ensure_language_fits(problem: &Problem, language: Language) -> Result<()> {
    let sql_problem = problem.mode == kb_core::ProblemMode::Sql;
    if sql_problem != (language == Language::Sql) {
        return Err(AppError::new(
            AppErrorCode::UnsupportedLanguage,
            if sql_problem {
                format!("{} is an SQL problem: write one SQL query", problem.id)
            } else {
                format!("{} takes code, not SQL", problem.id)
            },
        ));
    }
    Ok(())
}
