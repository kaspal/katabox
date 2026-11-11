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

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AppInfo {
    pub app_version: String,
    pub content: ContentMeta,
    pub user_schema_version: i64,
    pub sandbox_root: String,
    pub sandbox_root_source: RootSource,
    /// Languages the judge supports, with their toolchain status.
    pub toolchains: Vec<ToolchainInfo>,
    /// `windows`, `linux`, ...
    pub platform: String,
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
    env: JudgeEnv,
    root: PathBuf,
    root_source: RootSource,
    judging: tokio::sync::Mutex<()>,
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
        Ok(App {
            env: JudgeEnv::dev(&root),
            store,
            root,
            root_source,
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
        let toolchains = kb_judge::languages::SUPPORTED
            .into_iter()
            .map(|language| match self.env.toolchain_program(language) {
                Ok(p) => ToolchainInfo {
                    language,
                    present: true,
                    program: Some(p.display().to_string()),
                    problem: None,
                },
                Err(e) => ToolchainInfo {
                    language,
                    present: false,
                    program: None,
                    problem: Some(e.to_string()),
                },
            })
            .collect();
        Ok(AppInfo {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            content: self.store.content.meta().await?,
            user_schema_version: self.store.user.schema_version().await?,
            sandbox_root: self.root.display().to_string(),
            sandbox_root_source: self.root_source,
            toolchains,
            platform: std::env::consts::OS.to_string(),
        })
    }

    // --- Problems -------------------------------------------------------------------

    pub async fn list_problems(&self, filter: &ProblemFilter) -> Result<Vec<ProblemSummary>> {
        Ok(self.store.list_problems(filter).await?)
    }

    pub async fn get_problem(&self, id: &str, lang: UiLang) -> Result<ProblemView> {
        Ok(self.store.content.get_problem(id, lang).await?)
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
        self.store.user.record_run(problem, language, code).await?;
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
        let env = self.env.clone();
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

#[cfg(test)]
pub(crate) mod testutil;

#[cfg(test)]
mod tests;
