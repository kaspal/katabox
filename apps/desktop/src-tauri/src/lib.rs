//! KataBox desktop shell: resolves the app's folders, holds the [`kb_app::App`] and
//! exposes it as typed Tauri commands (ADR 0011). Keep business logic out of this crate;
//! it belongs in `crates/` (mostly kb-app). Every command is a short forward to kb-app
//! and returns `Result<T, AppError>`.

use std::path::{Path, PathBuf};

use kb_app::practice::{DailyProblem, ListView, PatternProgress, ReviewItem, ReviewState};
use kb_app::{
    App, AppError, AppErrorCode, AppInfo, AppPaths, ContinueItem, CustomInput, EditorialState,
    LanguagesInfo, SubmissionView, SubmitOptions, SubmitResult,
};
use kb_core::{JudgeEvent, Language, Report};
use kb_store::{
    Draft, ProblemFilter, ProblemSummary, ProblemView, Snapshot, SubmissionSummary, UiLang,
};
use serde_json::Value;
use tauri::ipc::Channel;
use tauri::path::BaseDirectory;
use tauri::{Manager, State};

type Res<T> = Result<T, AppError>;

/// The opened app, or why it could not open (every command then returns that error,
/// and the UI shows it instead of crashing).
struct AppState(Result<App, AppError>);

impl AppState {
    fn app(&self) -> Res<&App> {
        self.0.as_ref().map_err(Clone::clone)
    }
}

#[tauri::command]
#[specta::specta]
async fn app_info(state: State<'_, AppState>) -> Res<AppInfo> {
    state.app()?.app_info().await
}

#[tauri::command]
#[specta::specta]
async fn list_problems(
    state: State<'_, AppState>,
    filter: ProblemFilter,
) -> Res<Vec<ProblemSummary>> {
    state.app()?.list_problems(&filter).await
}

/// The most recently edited draft whose problem exists, for Home's "Continue".
#[tauri::command]
#[specta::specta]
async fn continue_item(state: State<'_, AppState>, ui_lang: UiLang) -> Res<Option<ContinueItem>> {
    state.app()?.continue_item(ui_lang).await
}

#[tauri::command]
#[specta::specta]
async fn get_problem(state: State<'_, AppState>, id: String, ui_lang: UiLang) -> Res<ProblemView> {
    state.app()?.get_problem(&id, ui_lang).await
}

/// The editorial and its lock state (unlocked when solved or after 3 failed submits).
#[tauri::command]
#[specta::specta]
async fn editorial(
    state: State<'_, AppState>,
    problem: String,
    ui_lang: UiLang,
) -> Res<EditorialState> {
    state.app()?.editorial(&problem, ui_lang).await
}

/// The language of the problem's most recently edited draft.
#[tauri::command]
#[specta::specta]
async fn last_language(state: State<'_, AppState>, problem: String) -> Res<Option<Language>> {
    state.app()?.last_language(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn get_stub(state: State<'_, AppState>, problem: String, language: Language) -> Res<String> {
    state.app()?.get_stub(&problem, language).await
}

#[tauri::command]
#[specta::specta]
async fn load_draft(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
) -> Res<Option<Draft>> {
    state.app()?.load_draft(&problem, language).await
}

#[tauri::command]
#[specta::specta]
async fn save_draft(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
) -> Res<()> {
    state.app()?.save_draft(&problem, language, &code).await
}

/// Public tests plus `custom` inputs; progress goes to `on_event`.
#[tauri::command]
#[specta::specta]
async fn run(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
    custom: Vec<CustomInput>,
    on_event: Channel<JudgeEvent>,
) -> Res<Report> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state
        .app()?
        .run(&problem, language, &code, custom, send)
        .await
}

/// Every test; the submission is recorded. Progress goes to `on_event`.
#[tauri::command]
#[specta::specta]
async fn submit(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
    opts: SubmitOptions,
    on_event: Channel<JudgeEvent>,
) -> Res<SubmitResult> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state
        .app()?
        .submit(&problem, language, &code, &opts, send)
        .await
}

#[tauri::command]
#[specta::specta]
async fn list_submissions(
    state: State<'_, AppState>,
    problem: String,
) -> Res<Vec<SubmissionSummary>> {
    state.app()?.list_submissions(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn get_submission(state: State<'_, AppState>, id: i64) -> Res<SubmissionView> {
    state.app()?.get_submission(id).await
}

#[tauri::command]
#[specta::specta]
async fn list_snapshots(state: State<'_, AppState>, problem: String) -> Res<Vec<Snapshot>> {
    state.app()?.list_snapshots(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn restore_snapshot(state: State<'_, AppState>, id: i64) -> Res<Snapshot> {
    state.app()?.restore_snapshot(id).await
}

#[tauri::command]
#[specta::specta]
async fn get_note(state: State<'_, AppState>, problem: String) -> Res<Option<String>> {
    state.app()?.get_note(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn set_note(state: State<'_, AppState>, problem: String, body: String) -> Res<()> {
    state.app()?.set_note(&problem, &body).await
}

#[tauri::command]
#[specta::specta]
async fn unlock_hint(state: State<'_, AppState>, problem: String, tier: u32) -> Res<Vec<u32>> {
    state.app()?.unlock_hint(&problem, tier).await
}

#[tauri::command]
#[specta::specta]
async fn unlocked_hints(state: State<'_, AppState>, problem: String) -> Res<Vec<u32>> {
    state.app()?.unlocked_hints(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn get_setting(state: State<'_, AppState>, key: String) -> Res<Option<Value>> {
    state.app()?.get_setting(&key).await
}

#[tauri::command]
#[specta::specta]
async fn set_setting(state: State<'_, AppState>, key: String, value: Value) -> Res<()> {
    state.app()?.set_setting(&key, &value).await
}

// --- Practice (ADR 0019): `today` is the user's local date, YYYY-MM-DD ---------------

/// The pattern map: every pattern with solved/total and its next problem.
#[tauri::command]
#[specta::specta]
async fn patterns(state: State<'_, AppState>, lang: UiLang) -> Res<Vec<PatternProgress>> {
    state.app()?.patterns(lang).await
}

/// What to solve after `problem` (never a solved one).
#[tauri::command]
#[specta::specta]
async fn next_problem(
    state: State<'_, AppState>,
    problem: String,
    lang: UiLang,
) -> Res<Option<ProblemSummary>> {
    state.app()?.next_problem(&problem, lang).await
}

/// Today's daily problem, picked once per local day.
#[tauri::command]
#[specta::specta]
async fn daily_problem(
    state: State<'_, AppState>,
    today: String,
    lang: UiLang,
) -> Res<Option<DailyProblem>> {
    state.app()?.daily_problem(&today, lang).await
}

/// Consecutive days with the daily problem solved.
#[tauri::command]
#[specta::specta]
async fn streak(state: State<'_, AppState>, today: String) -> Res<u32> {
    state.app()?.streak(&today).await
}

#[tauri::command]
#[specta::specta]
async fn reviews_due(
    state: State<'_, AppState>,
    today: String,
    lang: UiLang,
) -> Res<Vec<ReviewItem>> {
    state.app()?.reviews_due(&today, lang).await
}

#[tauri::command]
#[specta::specta]
async fn review_state(
    state: State<'_, AppState>,
    problem: String,
    today: String,
) -> Res<Option<ReviewState>> {
    state.app()?.review_state(&problem, &today).await
}

#[tauri::command]
#[specta::specta]
async fn review_snooze(state: State<'_, AppState>, problem: String, today: String) -> Res<()> {
    state.app()?.review_snooze(&problem, &today).await
}

#[tauri::command]
#[specta::specta]
async fn review_remove(state: State<'_, AppState>, problem: String) -> Res<()> {
    state.app()?.review_remove(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn review_add(state: State<'_, AppState>, problem: String, today: String) -> Res<()> {
    state.app()?.review_add(&problem, &today).await
}

#[tauri::command]
#[specta::specta]
async fn lists(state: State<'_, AppState>, lang: UiLang) -> Res<Vec<ListView>> {
    state.app()?.lists(lang).await
}

/// Create a list; returns its id.
#[tauri::command]
#[specta::specta]
async fn create_list(state: State<'_, AppState>, name: String) -> Res<i64> {
    state.app()?.create_list(&name).await
}

#[tauri::command]
#[specta::specta]
async fn rename_list(state: State<'_, AppState>, id: i64, name: String) -> Res<()> {
    state.app()?.rename_list(id, &name).await
}

#[tauri::command]
#[specta::specta]
async fn delete_list(state: State<'_, AppState>, id: i64) -> Res<()> {
    state.app()?.delete_list(id).await
}

#[tauri::command]
#[specta::specta]
async fn add_to_list(state: State<'_, AppState>, id: i64, problem: String) -> Res<()> {
    state.app()?.add_to_list(id, &problem).await
}

#[tauri::command]
#[specta::specta]
async fn remove_from_list(state: State<'_, AppState>, id: i64, problem: String) -> Res<()> {
    state.app()?.remove_from_list(id, &problem).await
}

/// Put a list's problems in this order (exactly the list's problems).
#[tauri::command]
#[specta::specta]
async fn reorder_list(state: State<'_, AppState>, id: i64, problems: Vec<String>) -> Res<()> {
    state.app()?.reorder_list(id, &problems).await
}

/// Ids of the lists holding `problem`.
#[tauri::command]
#[specta::specta]
async fn lists_with(state: State<'_, AppState>, problem: String) -> Res<Vec<i64>> {
    state.app()?.lists_with(&problem).await
}

/// This machine's calibration (ADR 0017), if it was measured.
#[tauri::command]
#[specta::specta]
async fn calibration(state: State<'_, AppState>) -> Res<Option<kb_app::CalibrationInfo>> {
    Ok(state.app()?.calibration())
}

/// Measure this machine again; the judge's limits follow the new factor.
#[tauri::command]
#[specta::specta]
async fn recalibrate(state: State<'_, AppState>) -> Res<kb_app::CalibrationInfo> {
    state.app()?.recalibrate().await
}

/// Start a language server for `language` on `problem`, replacing the language's previous
/// session (ADR 0014). Server messages and status changes stream over `on_event`.
#[tauri::command]
#[specta::specta]
async fn lsp_connect(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    text: String,
    on_event: Channel<kb_bridge::LspEvent>,
) -> Res<kb_bridge::LspConnection> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state
        .app()?
        .lsp_connect(&problem, language, &text, send)
        .await
}

/// Relay a JSON-RPC message from the editor's client of `session` to its language server.
#[tauri::command]
#[specta::specta]
async fn lsp_send(
    state: State<'_, AppState>,
    language: Language,
    session: u32,
    message: String,
) -> Res<()> {
    state.app()?.lsp_send(language, session, message).await
}

/// Stop `session`'s language server (the editor closed).
#[tauri::command]
#[specta::specta]
async fn lsp_disconnect(state: State<'_, AppState>, language: Language, session: u32) -> Res<()> {
    state.app()?.lsp_disconnect(language, session).await;
    Ok(())
}

/// Install state of every language, with sizes and free disk space (ADR 0015).
#[tauri::command]
#[specta::specta]
async fn languages(state: State<'_, AppState>) -> Res<LanguagesInfo> {
    Ok(state.app()?.languages())
}

/// Download and install languages; progress goes to `on_event`. Installing again after a
/// failure or a cancel resumes the downloads.
#[tauri::command]
#[specta::specta]
async fn install_languages(
    state: State<'_, AppState>,
    languages: Vec<Language>,
    on_event: Channel<kb_toolchain::InstallEvent>,
) -> Res<()> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state.app()?.install_languages(&languages, send).await
}

#[tauri::command]
#[specta::specta]
async fn cancel_install(state: State<'_, AppState>) -> Res<()> {
    state.app()?.cancel_install();
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn uninstall_language(state: State<'_, AppState>, language: Language) -> Res<()> {
    state.app()?.uninstall_language(language).await
}

/// Every command, for the invoke handler and the TypeScript bindings.
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
        app_info,
        continue_item,
        list_problems,
        get_problem,
        editorial,
        last_language,
        get_stub,
        load_draft,
        save_draft,
        run,
        submit,
        list_submissions,
        get_submission,
        list_snapshots,
        restore_snapshot,
        get_note,
        set_note,
        unlock_hint,
        unlocked_hints,
        get_setting,
        set_setting,
        calibration,
        recalibrate,
        patterns,
        next_problem,
        daily_problem,
        streak,
        reviews_due,
        review_state,
        review_snooze,
        review_remove,
        review_add,
        lists,
        create_list,
        rename_list,
        delete_list,
        add_to_list,
        remove_from_list,
        reorder_list,
        lists_with,
        lsp_connect,
        lsp_send,
        lsp_disconnect,
        languages,
        install_languages,
        cancel_install,
        uninstall_language,
    ])
}

const BINDINGS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/bindings.ts");

/// Write `apps/desktop/src/bindings.ts` (`cargo run -p katabox-desktop --bin
/// export-bindings`; debug builds of the app also do it at startup). The file is checked
/// in and CI's `pnpm build` typechecks it.
pub fn export_bindings() {
    export_bindings_from(&specta_builder());
}

fn export_bindings_from(builder: &tauri_specta::Builder<tauri::Wry>) {
    let ts = specta_typescript::Typescript::default()
        .header(
            "// Generated by tauri-specta from apps/desktop/src-tauri/src/lib.rs. Do not edit.\n\
             // Regenerate: cargo run -p katabox-desktop --bin export-bindings\n\
             // @ts-nocheck",
        )
        // Times (ms, us) and ids are 64-bit in Rust but far below 2^53 (ADR 0011).
        .bigint(specta_typescript::BigIntExportBehavior::Number);
    builder
        .export(ts, BINDINGS)
        .expect("failed to export TypeScript bindings");
}

/// Open the stores and pick the sandbox root, from Tauri's standard folders.
fn open_app(handle: &tauri::AppHandle) -> Res<App> {
    let internal =
        |what: &str, e: tauri::Error| AppError::new(AppErrorCode::Internal, format!("{what}: {e}"));
    let path = handle.path();
    // Debug builds keep their data in `<identifier>-dev` folders, so development and
    // testing never touch the user's real user.db (ADR 0012).
    let dev_data = cfg!(debug_assertions);
    let data_folder = |dir: PathBuf| {
        if dev_data {
            let mut name = dir.file_name().unwrap_or_default().to_os_string();
            name.push("-dev");
            dir.with_file_name(name)
        } else {
            dir
        }
    };
    let paths = AppPaths {
        content_db: path
            .resolve("content.db", BaseDirectory::Resource)
            .map_err(|e| internal("content.db resource", e))?,
        user_db: data_folder(
            path.app_data_dir()
                .map_err(|e| internal("app data folder", e))?,
        )
        .join("user.db"),
        local_data_dir: data_folder(
            path.app_local_data_dir()
                .map_err(|e| internal("app local data folder", e))?,
        ),
        dev_data,
        // Development builds use the fetch script's root when nothing else is set.
        // (`<repo>/apps/desktop/src-tauri` → `<repo>/target/katabox`.)
        dev_root: cfg!(debug_assertions)
            .then(|| Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(3))
            .flatten()
            .map(|repo| repo.join("target").join("katabox")),
    };
    tauri::async_runtime::block_on(App::open(&paths))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_app() {
    #[cfg(target_os = "linux")]
    if let Ok(exe) = std::env::current_exe() {
        kb_runner::set_helper_path(exe);
    }

    let builder = specta_builder();
    #[cfg(debug_assertions)]
    export_bindings_from(&builder);

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            let state = open_app(app.handle());
            if let Err(e) = &state {
                eprintln!("KataBox could not start: {e}");
            }
            app.manage(AppState(state));
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building KataBox")
        .run(|handle, event| {
            // Stop language servers cleanly on exit (their Job Objects would kill them anyway).
            if let tauri::RunEvent::Exit = event
                && let Some(state) = handle.try_state::<AppState>()
                && let Ok(app) = state.app()
            {
                tauri::async_runtime::block_on(app.lsp_shutdown());
            }
        });
}
