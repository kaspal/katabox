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

mod error;
mod root;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use kb_core::{JudgeEvent, Language, Report, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem};
use kb_store::{
    ContentMeta, Draft, ProblemFilter, ProblemSummary, ProblemView, Snapshot, Store,
    SubmissionSummary, UiLang,
};
use serde::Serialize;
use serde_json::{Map, Value};

pub use error::{AppError, AppErrorCode};
pub use root::{RootSource, SANDBOX_ROOT_ENV, SANDBOX_ROOT_SETTING};

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
}

/// A user-written test input for Run: argument values keyed by parameter name.
pub type CustomInput = Map<String, Value>;

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
}

impl App {
    /// Open both stores and pick the sandbox root (see [`root`] for the order).
    pub async fn open(paths: &AppPaths) -> Result<App> {
        let store = Store::open(&paths.content_db, &paths.user_db).await?;
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
        })
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

    fn env(&self) -> JudgeEnv {
        self.env.read().unwrap_or_else(|e| e.into_inner()).clone()
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
        Ok(result?)
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

    /// The problem page. The editorial is left out on purpose: it is locked until the
    /// problem is solved or after enough failed submissions; see [`App::editorial`].
    pub async fn get_problem(&self, id: &str, lang: UiLang) -> Result<ProblemView> {
        let mut view = self.store.content.get_problem(id, lang).await?;
        view.editorial = None;
        Ok(view)
    }

    /// The editorial and whether it is unlocked: once the problem is solved, or after
    /// [`EDITORIAL_FAILURES`] non-Accepted submissions (ADR 0013).
    pub async fn editorial(&self, id: &str, lang: UiLang) -> Result<EditorialState> {
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
        Ok(kb_judge::starter_code(language, &p.signature)?)
    }

    async fn problem(&self, id: &str) -> Result<Problem> {
        let d = self.store.content.judge_data(id).await?;
        Ok(Problem::from_parts(
            &d.id,
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
        let p = self.problem(problem).await?;
        let mut tests: Vec<TestCase> = p.public_tests().cloned().collect();
        if !custom.is_empty() {
            tests.extend(self.custom_tests(&p, language, custom).await?);
        }
        let report = self.judge(p, tests, language, code, on_event).await?;
        self.store
            .user
            .record_run(problem, language, code, report.verdict)
            .await?;
        Ok(report)
    }

    /// Judge `code` on every test, record the submission (with the full report) and
    /// return it with hidden tests redacted.
    pub async fn submit<F>(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        on_event: F,
    ) -> Result<SubmitResult>
    where
        F: FnMut(JudgeEvent) + Send + 'static,
    {
        let _one = self.judge_slot()?;
        let p = self.problem(problem).await?;
        let tests = p.tests.clone();
        let mut report = self.judge(p, tests, language, code, on_event).await?;
        let submission_id = self
            .store
            .user
            .record_submission(problem, language, code, &report)
            .await?;
        report.redact_hidden();
        Ok(SubmitResult {
            submission_id,
            report,
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
            kb_judge::judge_with_progress(&problem, &tests, language, &code, &env, &mut on_event)
        })
        .await??;
        Ok(report)
    }

    /// Turn custom inputs into tests whose expected outputs come from the reference
    /// solution in `language`.
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
        Ok(self
            .lsp
            .connect(problem, language, text, std::sync::Arc::new(on_event))
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
        Ok(self.store.user.list_submissions(problem).await?)
    }

    pub async fn get_submission(&self, id: i64) -> Result<SubmissionView> {
        let s = self.store.user.get_submission(id).await?;
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
        Ok(self.store.user.list_snapshots(problem).await?)
    }

    pub async fn restore_snapshot(&self, id: i64) -> Result<Snapshot> {
        Ok(self.store.user.restore_snapshot(id).await?)
    }

    pub async fn get_note(&self, problem: &str) -> Result<Option<String>> {
        Ok(self.store.user.get_note(problem).await?)
    }

    pub async fn set_note(&self, problem: &str, body: &str) -> Result<()> {
        Ok(self.store.user.set_note(problem, body).await?)
    }

    /// Reveal hint `tier` (1-based); returns every unlocked tier.
    pub async fn unlock_hint(&self, problem: &str, tier: u32) -> Result<Vec<u32>> {
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
