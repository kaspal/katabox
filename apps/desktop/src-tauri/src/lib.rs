//! KataBox desktop shell: resolves the app's folders, holds the [`kb_app::App`] and
//! exposes it as typed Tauri commands (ADR 0011). Keep business logic out of this crate;
//! it belongs in `crates/` (mostly kb-app). Every command is a short forward to kb-app
//! and returns `Result<T, AppError>`.

use std::path::PathBuf;

use kb_app::{
    App, AppError, AppErrorCode, AppInfo, AppPaths, CustomInput, SubmissionView, SubmitResult,
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

#[tauri::command]
#[specta::specta]
async fn get_problem(state: State<'_, AppState>, id: String, ui_lang: UiLang) -> Res<ProblemView> {
    state.app()?.get_problem(&id, ui_lang).await
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
    on_event: Channel<JudgeEvent>,
) -> Res<SubmitResult> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state.app()?.submit(&problem, language, &code, send).await
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

/// Every command, for the invoke handler and the TypeScript bindings.
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
        app_info,
        list_problems,
        get_problem,
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
    let paths = AppPaths {
        content_db: path
            .resolve("content.db", BaseDirectory::Resource)
            .map_err(|e| internal("content.db resource", e))?,
        user_db: path
            .app_data_dir()
            .map_err(|e| internal("app data folder", e))?
            .join("user.db"),
        local_data_dir: path
            .app_local_data_dir()
            .map_err(|e| internal("app local data folder", e))?,
        // Development builds use the fetch script's root when nothing else is set.
        dev_root: cfg!(debug_assertions)
            .then(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../target/katabox")),
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
        .run(tauri::generate_context!())
        .expect("error while running KataBox");
}
