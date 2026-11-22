//! KataBox desktop shell: resolves the app's folders, holds the [`kb_app::App`] and
//! exposes it as typed Tauri commands (ADR 0011). Keep business logic out of this crate;
//! it belongs in `crates/` (mostly kb-app). Every command is a short forward to kb-app
//! and returns `Result<T, AppError>`.

mod window_state;

use std::path::{Path, PathBuf};

use kb_app::comforts::{SearchResults, SnippetInput};
use kb_app::debug::{DebugAction, DebugInspect, DebugStart, DebugView};
use kb_app::interview::{InterviewResult, InterviewSetup, InterviewView};
use kb_app::journal::{JournalOverview, MistakeView};
use kb_app::kata::{KataOptions, KataOverview, KataStart, KataView};
use kb_app::plan_app::{PlanIcsLabels, PlanSetup, PlanView};
use kb_app::practice::{
    DailyProblem, ListView, PatternProgress, ReviewItem, ReviewState, ReviewSummary,
    SolvedBreakdown,
};
use kb_app::profile::ProfileView;
use kb_app::rating::RatingsView;
use kb_app::reading::{ReadingAnswer, ReadingResult};
use kb_app::transfer::{ImportPreview, ImportResult, TransferCounts};
use kb_app::{
    App, AppError, AppErrorCode, AppInfo, AppPaths, ContinueItem, CustomInput, EditorialState,
    LanguagesInfo, SubmissionView, SubmitOptions, SubmitResult,
};
use kb_app::{LanguageCount, LearnTopicView};
use kb_core::debug::EvalResult;
use kb_core::{JudgeEvent, Language, Report};
use kb_store::journal::ReflectionRow;
use kb_store::kata::KataAttemptRow;
use kb_store::reading_answers::ReadingAnswerRow;
use kb_store::snippets::SnippetRow;
use kb_store::{AcceptedSolution, ActivityRow, LearnPath, LearnSubject, LearnTopicSummary};
use kb_store::{
    Draft, ProblemFilter, ProblemSummary, ProblemView, Snapshot, SubmissionSummary, UiLang,
};
use kb_store::{ReadingLink, ReadingSetSummary, ReadingSetView};
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

/// The Learn topics of each practice pattern (Library and Progress link to them).
#[tauri::command]
#[specta::specta]
async fn learn_by_pattern(
    state: State<'_, AppState>,
    ui_lang: UiLang,
) -> Res<Vec<kb_app::PatternTopics>> {
    state.app()?.learn_by_pattern(ui_lang).await
}

/// Home's "Review a topic": a viewed topic (`learn.viewed`, in order) with an unsolved
/// linked problem.
#[tauri::command]
#[specta::specta]
async fn topic_to_review(
    state: State<'_, AppState>,
    viewed: Vec<String>,
    ui_lang: UiLang,
) -> Res<Option<kb_app::TopicReview>> {
    state.app()?.topic_to_review(&viewed, ui_lang).await
}

/// The welcome flow's three placement problems.
#[tauri::command]
#[specta::specta]
async fn placement_problems(
    state: State<'_, AppState>,
    ui_lang: UiLang,
) -> Res<Vec<ProblemSummary>> {
    state.app()?.placement_problems(ui_lang).await
}

/// Finish the welcome flow: sets where practice starts (from the placement's solves, or
/// fundamentals) and returns the problem to open first.
#[tauri::command]
#[specta::specta]
async fn finish_onboarding(
    state: State<'_, AppState>,
    placement: Option<Vec<String>>,
    ui_lang: UiLang,
) -> Res<Option<ProblemSummary>> {
    state.app()?.finish_onboarding(placement, ui_lang).await
}

/// What "Copy diagnostics" shows: versions and flags, never paths or user data.
#[tauri::command]
#[specta::specta]
async fn diagnostics(state: State<'_, AppState>) -> Res<kb_app::Diagnostics> {
    let mut d = state.app()?.diagnostics().await?;
    if cfg!(feature = "updater") {
        d.features.push("updater".into());
    }
    Ok(d)
}

#[tauri::command]
#[specta::specta]
async fn list_problems(
    state: State<'_, AppState>,
    filter: ProblemFilter,
) -> Res<Vec<ProblemSummary>> {
    state.app()?.list_problems(&filter).await
}

/// Every Learn subject with its topics (ADR 0029).
#[tauri::command]
#[specta::specta]
async fn learn_catalog(state: State<'_, AppState>, ui_lang: UiLang) -> Res<Vec<LearnSubject>> {
    state.app()?.learn_catalog(ui_lang).await
}

/// The learning paths on Learn home (Phase 6).
#[tauri::command]
#[specta::specta]
async fn learn_paths(state: State<'_, AppState>, ui_lang: UiLang) -> Res<Vec<LearnPath>> {
    state.app()?.learn_paths(ui_lang).await
}

/// One Learn topic page with its linked problems.
#[tauri::command]
#[specta::specta]
async fn learn_topic(
    state: State<'_, AppState>,
    id: String,
    ui_lang: UiLang,
) -> Res<LearnTopicView> {
    state.app()?.learn_topic(&id, ui_lang).await
}

/// Learn topics matching free text in either language.
#[tauri::command]
#[specta::specta]
async fn search_learn(
    state: State<'_, AppState>,
    query: String,
    ui_lang: UiLang,
) -> Res<Vec<LearnTopicSummary>> {
    state.app()?.search_learn(&query, ui_lang).await
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

/// Shrink a failing input to a smaller one that still fails (ADR 0030); progress goes to
/// `on_event` after each round.
#[tauri::command]
#[specta::specta]
async fn shrink(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
    input: CustomInput,
    on_event: Channel<kb_app::ShrinkEvent>,
) -> Res<kb_app::ShrinkResult> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state
        .app()?
        .shrink(&problem, language, &code, input, send)
        .await
}

/// Stop the running shrink after its current sandbox run.
#[tauri::command]
#[specta::specta]
async fn cancel_shrink(state: State<'_, AppState>) -> Res<()> {
    state.app()?.cancel_shrink();
    Ok(())
}

/// Check custom inputs against the signature and validate.py (expansion M9).
#[tauri::command]
#[specta::specta]
async fn check_custom(
    state: State<'_, AppState>,
    problem: String,
    inputs: Vec<CustomInput>,
) -> Res<kb_app::CustomCheck> {
    state.app()?.check_custom(&problem, inputs).await
}

/// The stress lab (expansion M8): `code` against `brute` on generated inputs.
#[tauri::command]
#[specta::specta]
async fn stress_test(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
    brute: String,
    on_event: Channel<kb_app::stress::StressEvent>,
) -> Res<kb_app::stress::StressResult> {
    let send = move |e| {
        let _ = on_event.send(e);
    };
    state
        .app()?
        .stress(&problem, language, &code, &brute, send)
        .await
}

#[tauri::command]
#[specta::specta]
async fn cancel_stress(state: State<'_, AppState>) -> Res<()> {
    state.app()?.cancel_stress();
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn stress_brute(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
) -> Res<Option<String>> {
    state.app()?.stress_brute(&problem, language).await
}

#[tauri::command]
#[specta::specta]
async fn save_stress_brute(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
) -> Res<()> {
    state
        .app()?
        .save_stress_brute(&problem, language, &code)
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

/// Measure the solution at growing sizes and estimate its complexity (ADR 0024).
#[tauri::command]
#[specta::specta]
async fn profile_solution(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
    code: String,
) -> Res<ProfileView> {
    state
        .app()?
        .profile_solution(&problem, language, &code)
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

/// Progress → heatmap: Submits since `since_ms` (UTC ms); the UI buckets local days.
#[tauri::command]
#[specta::specta]
async fn activity(state: State<'_, AppState>, since_ms: i64) -> Res<Vec<ActivityRow>> {
    state.app()?.activity(since_ms).await
}

/// Progress → solved by difficulty and by topic.
#[tauri::command]
#[specta::specta]
async fn solved_breakdown(state: State<'_, AppState>, lang: UiLang) -> Res<SolvedBreakdown> {
    state.app()?.solved_breakdown(lang).await
}

/// Progress → the review queue at a glance.
#[tauri::command]
#[specta::specta]
async fn review_summary(state: State<'_, AppState>, today: String) -> Res<ReviewSummary> {
    state.app()?.review_summary(&today).await
}

/// Settings → Appearance → "Reset window size and position": the default size, centred,
/// saved as the window's new bounds.
#[tauri::command]
#[specta::specta]
async fn reset_window(window: tauri::WebviewWindow) -> Res<()> {
    tauri::async_runtime::spawn_blocking(move || window_state::reset(&window))
        .await
        .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))
}

/// Settings → About → Third-party notices (the bundled text; `None` in dev builds).
#[tauri::command]
#[specta::specta]
async fn third_party_notices(app: tauri::AppHandle) -> Res<Option<String>> {
    let path = app
        .path()
        .resolve("THIRD_PARTY_NOTICES.txt", BaseDirectory::Resource)
        .map_err(|e| AppError::new(AppErrorCode::Internal, format!("notices resource: {e}")))?;
    tauri::async_runtime::spawn_blocking(move || kb_app::third_party_notices(&path))
        .await
        .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?
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

// --- Practice modes (ADR 0020) --------------------------------------------------------

/// The workspace opened `problem` (mock interviews pick never-opened problems).
#[tauri::command]
#[specta::specta]
async fn mark_opened(state: State<'_, AppState>, problem: String) -> Res<()> {
    state.app()?.mark_opened(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn start_interview(
    state: State<'_, AppState>,
    setup: InterviewSetup,
    lang: UiLang,
) -> Res<InterviewView> {
    state.app()?.start_interview(&setup, lang).await
}

/// The running mock interview, if any (ended first when its time is up).
#[tauri::command]
#[specta::specta]
async fn interview_active(state: State<'_, AppState>, lang: UiLang) -> Res<Option<InterviewView>> {
    state.app()?.interview_active(lang).await
}

#[tauri::command]
#[specta::specta]
async fn end_interview(state: State<'_, AppState>, lang: UiLang) -> Res<InterviewResult> {
    state.app()?.end_interview(lang).await
}

/// Past mock interviews, newest first.
#[tauri::command]
#[specta::specta]
async fn interviews(state: State<'_, AppState>, lang: UiLang) -> Res<Vec<InterviewResult>> {
    state.app()?.interviews(lang).await
}

// --- Reading exercises (expansion M7, ADR 0035) --------------------------------------

#[tauri::command]
#[specta::specta]
async fn reading_sets(state: State<'_, AppState>, ui_lang: UiLang) -> Res<Vec<ReadingSetSummary>> {
    state.app()?.reading_sets(ui_lang).await
}

#[tauri::command]
#[specta::specta]
async fn reading_set(
    state: State<'_, AppState>,
    id: String,
    ui_lang: UiLang,
) -> Res<ReadingSetView> {
    state.app()?.reading_set(&id, ui_lang).await
}

#[tauri::command]
#[specta::specta]
async fn reading_answers(state: State<'_, AppState>) -> Res<Vec<ReadingAnswerRow>> {
    state.app()?.reading_answers().await
}

#[tauri::command]
#[specta::specta]
async fn reading_links(
    state: State<'_, AppState>,
    topic: String,
    ui_lang: UiLang,
) -> Res<Vec<ReadingLink>> {
    state.app()?.reading_links(&topic, ui_lang).await
}

#[tauri::command]
#[specta::specta]
async fn answer_reading(
    state: State<'_, AppState>,
    item: String,
    answer: ReadingAnswer,
    ui_lang: UiLang,
) -> Res<ReadingResult> {
    state.app()?.answer_reading(&item, answer, ui_lang).await
}

// --- Snippets and search (expansion M4, ADR 0034) ------------------------------------

#[tauri::command]
#[specta::specta]
async fn snippets(state: State<'_, AppState>) -> Res<Vec<SnippetRow>> {
    state.app()?.snippets().await
}

#[tauri::command]
#[specta::specta]
async fn save_snippet(state: State<'_, AppState>, snippet: SnippetInput) -> Res<SnippetRow> {
    state.app()?.save_snippet(&snippet).await
}

#[tauri::command]
#[specta::specta]
async fn delete_snippet(state: State<'_, AppState>, uid: String) -> Res<()> {
    state.app()?.delete_snippet(&uid).await
}

/// "Search everything": problems, Learn topics and snippets.
#[tauri::command]
#[specta::specta]
async fn search_all(state: State<'_, AppState>, query: String, lang: UiLang) -> Res<SearchResults> {
    state.app()?.search_all(&query, lang).await
}

// --- Mistake journal and reflections (expansion M3, ADR 0034) ------------------------

/// Progress → Mistakes: counts per tag with the latest examples.
#[tauri::command]
#[specta::specta]
async fn journal_overview(state: State<'_, AppState>, lang: UiLang) -> Res<JournalOverview> {
    state.app()?.journal_overview(lang).await
}

/// A failed submission's tags and note (`None` for an Accepted one).
#[tauri::command]
#[specta::specta]
async fn submission_mistake(
    state: State<'_, AppState>,
    submission: i64,
) -> Res<Option<MistakeView>> {
    state.app()?.submission_mistake(submission).await
}

#[tauri::command]
#[specta::specta]
async fn set_submission_mistake(
    state: State<'_, AppState>,
    submission: i64,
    tags: Vec<String>,
    note: String,
) -> Res<()> {
    state
        .app()?
        .set_submission_mistake(submission, &tags, &note)
        .await
}

/// The shrinker found a smaller failing input for this submission's failure.
#[tauri::command]
#[specta::specta]
async fn tag_shrunk(state: State<'_, AppState>, submission: i64) -> Res<()> {
    state.app()?.tag_shrunk(submission).await
}

#[tauri::command]
#[specta::specta]
async fn reflection(state: State<'_, AppState>, problem: String) -> Res<Option<ReflectionRow>> {
    state.app()?.reflection(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn set_reflection(
    state: State<'_, AppState>,
    problem: String,
    why: String,
    time_class: Option<String>,
    space_class: Option<String>,
) -> Res<()> {
    state
        .app()?
        .set_reflection(
            &problem,
            &why,
            time_class.as_deref(),
            space_class.as_deref(),
        )
        .await
}

// --- Skill rating and study plans (expansion M2, ADR 0033) ---------------------------

/// Ratings per pattern and overall (replayed from the history; estimated).
#[tauri::command]
#[specta::specta]
async fn ratings(state: State<'_, AppState>) -> Res<RatingsView> {
    state.app()?.ratings().await
}

/// The active study plan, if any.
#[tauri::command]
#[specta::specta]
async fn plan_view(state: State<'_, AppState>, lang: UiLang) -> Res<Option<PlanView>> {
    state.app()?.plan_view(lang).await
}

#[tauri::command]
#[specta::specta]
async fn create_plan(state: State<'_, AppState>, setup: PlanSetup, lang: UiLang) -> Res<PlanView> {
    state.app()?.create_plan(&setup, lang).await
}

/// Mark a plan item `pending`, `done` or `skipped`.
#[tauri::command]
#[specta::specta]
async fn set_plan_item(
    state: State<'_, AppState>,
    plan: i64,
    position: u32,
    status: String,
) -> Res<()> {
    state.app()?.set_plan_item(plan, position, &status).await
}

/// Move a plan item (and the pending items after it) one day later.
#[tauri::command]
#[specta::specta]
async fn reschedule_plan_item(state: State<'_, AppState>, plan: i64, position: u32) -> Res<()> {
    state.app()?.reschedule_plan_item(plan, position).await
}

#[tauri::command]
#[specta::specta]
async fn archive_plan(state: State<'_, AppState>, plan: i64) -> Res<()> {
    state.app()?.archive_plan(plan).await
}

/// Progress → Plan → Export to calendar: a Save dialog, then the plan as an `.ics`
/// file. `None` when cancelled, else the file name.
#[tauri::command]
#[specta::specta]
async fn export_plan_ics(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    labels: PlanIcsLabels,
    lang: UiLang,
) -> Res<Option<String>> {
    let kb = state.app()?;
    let name = format!("katabox-plan-{}.ics", chrono_free_today());
    let Some(path) = pick_file(app, "Calendar", &["ics"], Some(name)).await? else {
        return Ok(None);
    };
    kb.export_plan_ics(&path, &labels, lang).await?;
    Ok(path.file_name().map(|n| n.to_string_lossy().into_owned()))
}

// --- Kata drills (expansion M1, ADR 0033) ---------------------------------------------

/// Today's daily kata, the patterns and lists a kata can start from, and the running one.
#[tauri::command]
#[specta::specta]
async fn kata_options(state: State<'_, AppState>, today: String, lang: UiLang) -> Res<KataOptions> {
    state.app()?.kata_options(&today, lang).await
}

#[tauri::command]
#[specta::specta]
async fn start_kata(state: State<'_, AppState>, start: KataStart, lang: UiLang) -> Res<KataView> {
    state.app()?.start_kata(&start, lang).await
}

/// The running kata, if any (ended first once it expired).
#[tauri::command]
#[specta::specta]
async fn kata_state(state: State<'_, AppState>, lang: UiLang) -> Res<Option<KataView>> {
    state.app()?.kata_state(lang).await
}

/// The workspace opened `problem` in the running kata: its clock starts (once).
#[tauri::command]
#[specta::specta]
async fn kata_open(state: State<'_, AppState>, problem: String) -> Res<KataAttemptRow> {
    state.app()?.kata_open(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn end_kata(state: State<'_, AppState>, lang: UiLang) -> Res<Option<KataView>> {
    state.app()?.end_kata(lang).await
}

/// Personal bests, the rusty list, the kata streak and the latest sessions.
#[tauri::command]
#[specta::specta]
async fn kata_overview(
    state: State<'_, AppState>,
    today: String,
    lang: UiLang,
) -> Res<KataOverview> {
    state.app()?.kata_overview(&today, lang).await
}

#[tauri::command]
#[specta::specta]
async fn accepted_solution(
    state: State<'_, AppState>,
    problem: String,
    language: Language,
) -> Res<Option<AcceptedSolution>> {
    state.app()?.accepted_solution(&problem, language).await
}

#[tauri::command]
#[specta::specta]
async fn solved_languages(state: State<'_, AppState>, problem: String) -> Res<Vec<Language>> {
    state.app()?.solved_languages(&problem).await
}

#[tauri::command]
#[specta::specta]
async fn solved_by_language(state: State<'_, AppState>) -> Res<Vec<LanguageCount>> {
    state.app()?.solved_by_language().await
}

/// A file path from a native dialog, opened on a blocking thread (`None`: cancelled).
async fn pick_path(app: tauri::AppHandle, save: bool) -> Res<Option<PathBuf>> {
    let name = save.then(|| format!("katabox-progress-{}.kbx.json", chrono_free_today()));
    pick_file(app, "KataBox progress", &["json"], name).await
}

/// A native Open dialog (`save_as: None`) or Save dialog (`Some(default name)`) with one
/// filter, on a blocking thread (`None`: cancelled). The webview has no dialog
/// permission: dialogs are opened here only.
async fn pick_file(
    app: tauri::AppHandle,
    filter: &'static str,
    extensions: &'static [&'static str],
    save_as: Option<String>,
) -> Res<Option<PathBuf>> {
    use tauri_plugin_dialog::DialogExt;
    let picked = tauri::async_runtime::spawn_blocking(move || {
        let dialog = app.dialog().file().add_filter(filter, extensions);
        match save_as {
            Some(name) => dialog.set_file_name(name).blocking_save_file(),
            None => dialog.blocking_pick_file(),
        }
    })
    .await
    .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
    picked
        .map(|p| {
            p.into_path()
                .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))
        })
        .transpose()
}

/// Today's UTC date for the default file name (the name is only a suggestion).
fn chrono_free_today() -> String {
    let days = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() / 86_400);
    kb_core::Day::from_days(i64::try_from(days).unwrap_or(0)).to_string()
}

/// Settings → Data → Export: a Save dialog, then every record to one `.kbx.json`
/// file. `None` when cancelled.
#[tauri::command]
#[specta::specta]
async fn export_progress(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Res<Option<TransferCounts>> {
    let kb = state.app()?;
    let Some(path) = pick_path(app, true).await? else {
        return Ok(None);
    };
    kb.export_to(&path).await.map(Some)
}

/// Settings → Data → Import: an Open dialog, then the file is read and checked and kept
/// pending. `None` when cancelled.
#[tauri::command]
#[specta::specta]
async fn import_preview(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Res<Option<ImportPreview>> {
    let kb = state.app()?;
    let Some(path) = pick_path(app, false).await? else {
        return Ok(None);
    };
    kb.import_preview_from(&path).await.map(Some)
}

/// Merge the pending import (newer records win).
#[tauri::command]
#[specta::specta]
async fn import_apply(state: State<'_, AppState>, id: u32) -> Res<ImportResult> {
    state.app()?.import_apply(id).await
}

#[tauri::command]
#[specta::specta]
async fn import_cancel(state: State<'_, AppState>, id: u32) -> Res<()> {
    state.app()?.import_cancel(id);
    Ok(())
}

// --- Debugger (ADR 0022): replay runs in the sandbox, one session at a time ----------

/// Languages the debugger supports.
#[tauri::command]
#[specta::specta]
fn debug_languages() -> Vec<Language> {
    kb_app::debug::languages()
}

/// Record one test and stop at its first breakpoint (or show the result).
#[tauri::command]
#[specta::specta]
async fn debug_start(state: State<'_, AppState>, start: DebugStart) -> Res<DebugView> {
    state.app()?.debug_start(start).await
}

#[tauri::command]
#[specta::specta]
async fn debug_action(
    state: State<'_, AppState>,
    session: u32,
    action: DebugAction,
) -> Res<DebugView> {
    state.app()?.debug_action(session, action).await
}

#[tauri::command]
#[specta::specta]
async fn debug_set_breakpoints(
    state: State<'_, AppState>,
    session: u32,
    lines: Vec<u32>,
) -> Res<()> {
    state.app()?.debug_set_breakpoints(session, lines)
}

/// Show the current step again with another frame, expansions or watches.
#[tauri::command]
#[specta::specta]
async fn debug_inspect(
    state: State<'_, AppState>,
    session: u32,
    inspect: DebugInspect,
) -> Res<DebugView> {
    state.app()?.debug_inspect(session, inspect).await
}

/// Evaluate a console expression in a frame at the current step (never changes the run).
#[tauri::command]
#[specta::specta]
async fn debug_evaluate(
    state: State<'_, AppState>,
    session: u32,
    frame: u32,
    expr: String,
) -> Res<EvalResult> {
    state.app()?.debug_evaluate(session, frame, expr).await
}

#[tauri::command]
#[specta::specta]
async fn debug_end(state: State<'_, AppState>, session: u32) -> Res<()> {
    state.app()?.debug_end(session);
    Ok(())
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
        shrink,
        cancel_shrink,
        diagnostics,
        third_party_notices,
        reset_window,
        activity,
        solved_breakdown,
        review_summary,
        continue_item,
        list_problems,
        get_problem,
        learn_catalog,
        learn_paths,
        learn_by_pattern,
        placement_problems,
        finish_onboarding,
        topic_to_review,
        learn_topic,
        search_learn,
        editorial,
        last_language,
        get_stub,
        load_draft,
        save_draft,
        run,
        submit,
        profile_solution,
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
        mark_opened,
        start_interview,
        interview_active,
        end_interview,
        interviews,
        check_custom,
        stress_test,
        cancel_stress,
        stress_brute,
        save_stress_brute,
        reading_sets,
        reading_set,
        reading_answers,
        reading_links,
        answer_reading,
        snippets,
        save_snippet,
        delete_snippet,
        search_all,
        journal_overview,
        submission_mistake,
        set_submission_mistake,
        tag_shrunk,
        reflection,
        set_reflection,
        ratings,
        plan_view,
        create_plan,
        set_plan_item,
        reschedule_plan_item,
        archive_plan,
        export_plan_ics,
        kata_options,
        start_kata,
        kata_state,
        kata_open,
        end_kata,
        kata_overview,
        accepted_solution,
        solved_languages,
        solved_by_language,
        export_progress,
        import_preview,
        import_apply,
        import_cancel,
        debug_languages,
        debug_start,
        debug_action,
        debug_set_breakpoints,
        debug_inspect,
        debug_evaluate,
        debug_end,
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
             // Regenerate: cargo run -p katabox-desktop --features bindings --bin export-bindings\n\
             // @ts-nocheck",
        )
        // Times (ms, us) and ids are 64-bit in Rust but far below 2^53 (ADR 0011).
        .bigint(specta_typescript::BigIntExportBehavior::Number);
    builder
        .export(ts, BINDINGS)
        .expect("failed to export TypeScript bindings");
}

/// Open the stores and pick the sandbox root, from Tauri's standard folders.
/// Debug builds keep their data in `<identifier>-dev` folders, so development and
/// testing never touch the user's real user.db (ADR 0012).
fn data_folder(dir: PathBuf) -> PathBuf {
    if cfg!(debug_assertions) {
        let mut name = dir.file_name().unwrap_or_default().to_os_string();
        name.push("-dev");
        dir.with_file_name(name)
    } else {
        dir
    }
}

/// `katabox-desktop.exe --uninstall-cleanup`, run by the NSIS and MSI uninstallers (ADR 0021):
/// revoke the sandbox's folder grants and delete the AppContainer profile through
/// kb-runner's cleanup. Never deletes files; exits 0 when there is nothing to clean.
pub fn uninstall_cleanup() -> std::process::ExitCode {
    // Tauri's app_data_dir and app_local_data_dir: `<data dir>/<identifier>`.
    let id = env!("KATABOX_IDENTIFIER");
    let (Some(roaming), Some(local)) = (dirs::data_dir(), dirs::data_local_dir()) else {
        return std::process::ExitCode::SUCCESS;
    };
    let user_db = data_folder(roaming.join(id)).join("user.db");
    let local = data_folder(local.join(id));
    match tauri::async_runtime::block_on(kb_app::uninstall::cleanup_sandbox(&local, &user_db)) {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("KataBox cleanup: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn open_app(handle: &tauri::AppHandle) -> Res<App> {
    let internal =
        |what: &str, e: tauri::Error| AppError::new(AppErrorCode::Internal, format!("{what}: {e}"));
    let path = handle.path();
    let dev_data = cfg!(debug_assertions);
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

    let app = tauri::Builder::default()
        // File pickers for Export/Import, used only from Rust commands (ADR 0020): no
        // dialog permission is granted to the webview.
        .plugin(tauri_plugin_dialog::init());
    // Auto-update is prepared but off (ADR 0021): only a build with `--features updater`
    // (and tauri.updater.conf.json's key and endpoint) registers the plugin.
    #[cfg(feature = "updater")]
    let app = app.plugin(tauri_plugin_updater::Builder::new().build());
    app.invoke_handler(builder.invoke_handler())
        .setup(|app| {
            let state = open_app(app.handle());
            if let Err(e) = &state {
                eprintln!("KataBox could not start: {e}");
            }
            app.manage(AppState(state));
            // The main window starts hidden: place it where it was, then show it.
            app.manage(window_state::Saved::default());
            if let Some(window) = app.get_webview_window("main") {
                window_state::restore(&window);
                window_state::watch(&window);
            }
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
