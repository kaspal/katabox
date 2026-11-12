//! App tests that need no toolchain (all platforms). Judging end to end is in
//! `tests/judge_windows.rs`.

use super::*;
use crate::testutil::paths;
use kb_core::{Message, MessageCode, TestOutcome, TestResult};
use kb_store::StatusFilter;
use serde_json::json;

async fn app(name: &str) -> App {
    App::open(&paths(name)).await.unwrap()
}

#[tokio::test]
async fn opens_with_an_app_data_root() {
    let p = paths("open");
    let app = App::open(&p).await.unwrap();
    assert_eq!(app.root_source, RootSource::AppData);
    assert_eq!(app.sandbox_root(), p.local_data_dir.join("sandbox"));
    assert!(p.user_db.is_file());

    let info = app.app_info().await.unwrap();
    assert_eq!(info.content.pack_id, "test");
    assert_eq!(info.user_schema_version, 3); // migrations 0001-0003
    assert_eq!(info.sandbox_root_source, RootSource::AppData);
    assert!(!info.dev_data);
    assert_eq!(
        info.data_dir,
        p.user_db.parent().unwrap().display().to_string()
    );
    let langs: Vec<Language> = info.toolchains.iter().map(|t| t.language).collect();
    assert_eq!(langs, kb_judge::languages::SUPPORTED);
    // A fresh root has no toolchains.
    assert!(
        info.toolchains
            .iter()
            .all(|t| !t.present && t.problem.is_some())
    );
    app.close().await;
}

#[tokio::test]
async fn a_missing_content_db_is_a_typed_error() {
    let mut p = paths("nocontent");
    p.content_db = p.local_data_dir.join("missing.db");
    let err = App::open(&p).await.unwrap_err();
    assert_eq!(err.code, AppErrorCode::NotFound);
}

#[tokio::test]
async fn problems_and_stubs() {
    let app = app("problems").await;
    let all = app.list_problems(&ProblemFilter::default()).await.unwrap();
    assert!(all.iter().any(|p| p.id == "two-sum"));
    let todo = app
        .list_problems(&ProblemFilter {
            status: Some(StatusFilter::Todo),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(todo.len(), all.len());

    let es = app.get_problem("two-sum", UiLang::Es).await.unwrap();
    assert_eq!(es.title, "Suma de dos");
    let err = app.get_problem("nope", UiLang::En).await.unwrap_err();
    assert_eq!(err.code, AppErrorCode::NotFound);

    for (lang, decl) in [
        (Language::Python, "def two_sum("),
        (Language::TypeScript, "function twoSum("),
        (Language::Php, "function twoSum("),
        (Language::Rust, "fn two_sum("),
    ] {
        let stub = app.get_stub("two-sum", lang).await.unwrap();
        assert!(stub.contains(decl), "{lang}: {stub}");
    }
    let err = app
        .get_stub("two-sum", Language::JavaScript)
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::UnsupportedLanguage, "{err}");
}

#[tokio::test]
async fn judge_errors_are_typed() {
    let app = app("errors").await;
    // No toolchains in a fresh root.
    let err = app
        .submit("two-sum", Language::Python, "x", &submit_opts(), |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::ToolchainMissing, "{err}");
    let err = app
        .run("two-sum", Language::JavaScript, "x", vec![], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::UnsupportedLanguage, "{err}");
    // A custom input with the wrong keys is refused before anything runs.
    let bad = json!({"nums": [1, 2]}).as_object().unwrap().clone();
    let err = app
        .run("two-sum", Language::Python, "x", vec![bad], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidCustomInput);
    assert!(err.message.contains("custom input 1") && err.message.contains("target"));
    let err = app
        .submit("nope", Language::Python, "x", &submit_opts(), |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::NotFound);
    // Nothing was recorded for failed judge calls.
    assert!(app.list_submissions("two-sum").await.unwrap().is_empty());
}

#[tokio::test]
async fn one_judge_at_a_time() {
    let app = app("busy").await;
    let _held = app.judge_slot().unwrap();
    let err = app
        .submit("two-sum", Language::Python, "x", &submit_opts(), |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::JudgeBusy);
    let err = app
        .run("two-sum", Language::Python, "x", vec![], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::JudgeBusy);
}

#[tokio::test]
async fn stored_submissions_come_back_redacted() {
    let app = app("redact").await;
    let outcome = |kind, verdict| TestOutcome {
        result: TestResult {
            index: 1,
            kind,
            verdict,
            time_us: 3,
            memory_kb: 4,
            message: None,
        },
        actual: Some(json!([0, 1])),
        stdout: "printed".into(),
    };
    let report = Report {
        verdict: Verdict::Accepted,
        tests: vec![
            outcome(TestKind::Public, Verdict::Accepted),
            outcome(TestKind::Hidden, Verdict::Accepted),
        ],
        total_tests: 2,
        first_failure: None,
        compile_output: Some(Message::new(MessageCode::CompileError, "none")),
        time_us: 6,
        cpu_us: 0,
        memory_kb: 4,
        compile: None,
    };
    let id = app
        .store
        .user
        .record_submission("two-sum", Language::Python, "code", &report)
        .await
        .unwrap();
    let view = app.get_submission(id).await.unwrap();
    assert_eq!(view.code, "code");
    assert_eq!(view.summary.verdict, Verdict::Accepted);
    assert_eq!(view.report.tests[0].actual, Some(json!([0, 1])));
    assert_eq!(view.report.tests[1].actual, None);
    assert_eq!(view.report.tests[1].stdout, "");
    // user.db keeps the full report.
    let raw = app.store.user.get_submission(id).await.unwrap();
    assert_eq!(raw.report["tests"][1]["stdout"], "printed");
    assert_eq!(
        app.get_submission(9999).await.unwrap_err().code,
        AppErrorCode::NotFound
    );
}

#[tokio::test]
async fn drafts_notes_hints_settings() {
    let app = app("misc").await;
    assert_eq!(
        app.load_draft("two-sum", Language::Rust).await.unwrap(),
        None
    );
    app.save_draft("two-sum", Language::Rust, "fn x() {}")
        .await
        .unwrap();
    assert_eq!(
        app.load_draft("two-sum", Language::Rust)
            .await
            .unwrap()
            .unwrap()
            .code,
        "fn x() {}"
    );
    app.set_note("two-sum", "use a map").await.unwrap();
    assert_eq!(
        app.get_note("two-sum").await.unwrap().as_deref(),
        Some("use a map")
    );
    assert_eq!(app.unlock_hint("two-sum", 2).await.unwrap(), [2]);
    assert_eq!(app.unlock_hint("two-sum", 1).await.unwrap(), [1, 2]);
    assert_eq!(
        app.unlock_hint("two-sum", 0).await.unwrap_err().code,
        AppErrorCode::InvalidInput
    );
    app.set_setting("ui_lang", &json!("es")).await.unwrap();
    assert_eq!(app.get_setting("ui_lang").await.unwrap(), Some(json!("es")));
    assert!(app.list_snapshots("two-sum").await.unwrap().is_empty());
    assert_eq!(
        app.restore_snapshot(1).await.unwrap_err().code,
        AppErrorCode::NotFound
    );
}

#[tokio::test]
async fn the_root_setting_is_honoured_on_next_open() {
    let p = paths("setting");
    let app = App::open(&p).await.unwrap();
    let custom = p.local_data_dir.join("custom-root");
    std::fs::create_dir_all(&custom).unwrap();
    std::fs::write(custom.join(kb_runner::ROOT_MARKER), "").unwrap();
    app.set_setting(SANDBOX_ROOT_SETTING, &json!(custom.display().to_string()))
        .await
        .unwrap();
    app.close().await;
    let app = App::open(&p).await.unwrap();
    assert_eq!(app.sandbox_root(), custom);
    assert_eq!(app.root_source, RootSource::Setting);
}

#[tokio::test]
async fn dev_data_is_reported() {
    let mut p = paths("devdata");
    p.dev_data = true;
    let app = App::open(&p).await.unwrap();
    assert!(app.app_info().await.unwrap().dev_data);
}

#[tokio::test]
async fn continue_is_the_latest_existing_draft() {
    let app = app("continue").await;
    assert_eq!(app.continue_item(UiLang::En).await.unwrap(), None);
    app.save_draft("two-sum", Language::Rust, "fn x() {}")
        .await
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5));
    // A newer draft for a problem that is not in content.db is skipped.
    app.save_draft("gone", Language::Python, "x").await.unwrap();
    let c = app.continue_item(UiLang::Es).await.unwrap().unwrap();
    assert_eq!(c.problem_id, "two-sum");
    assert_eq!(c.title, "Suma de dos");
    assert_eq!(c.language, Language::Rust);
}

#[tokio::test]
async fn editorial_unlocks_after_accepted_or_three_failures() {
    let first = app("editorial").await;
    // get_problem never carries the editorial.
    assert_eq!(
        first
            .get_problem("two-sum", UiLang::En)
            .await
            .unwrap()
            .editorial,
        None
    );
    let locked = first.editorial("two-sum", UiLang::En).await.unwrap();
    assert!(!locked.unlocked && locked.available);
    assert_eq!((locked.failed_submits, locked.required_failures), (0, 3));
    assert_eq!(locked.text, None);

    let wa = report_with(Verdict::WrongAnswer);
    for i in 0..3 {
        let st = first.editorial("two-sum", UiLang::En).await.unwrap();
        assert!(!st.unlocked, "after {i} failures");
        first
            .store
            .user
            .record_submission("two-sum", Language::Python, &format!("x{i}"), &wa)
            .await
            .unwrap();
    }
    let st = first.editorial("two-sum", UiLang::En).await.unwrap();
    assert_eq!(
        (st.unlocked, st.reason),
        (true, Some(EditorialReason::Attempts))
    );
    assert!(st.text.unwrap().len() > 10);

    let other = app("editorial-ac").await;
    other
        .store
        .user
        .record_submission(
            "two-sum",
            Language::Rust,
            "x",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    let st = other.editorial("two-sum", UiLang::Es).await.unwrap();
    assert_eq!(
        (st.unlocked, st.reason),
        (true, Some(EditorialReason::Accepted))
    );
    assert_eq!(st.failed_submits, 0);
}

#[tokio::test]
async fn last_language_follows_the_latest_draft() {
    let app = app("lastlang").await;
    assert_eq!(app.last_language("two-sum").await.unwrap(), None);
    app.save_draft("two-sum", Language::Rust, "a")
        .await
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5));
    app.save_draft("two-sum", Language::Php, "b").await.unwrap();
    assert_eq!(
        app.last_language("two-sum").await.unwrap(),
        Some(Language::Php)
    );
}

/// Submit options for tests: a fixed local date, not a review.
fn submit_opts() -> SubmitOptions {
    SubmitOptions {
        today: "2026-11-14".into(),
        review: false,
        lang: kb_store::UiLang::En,
    }
}

fn report_with(verdict: Verdict) -> Report {
    Report {
        verdict,
        tests: vec![],
        total_tests: 7,
        first_failure: None,
        compile_output: None,
        time_us: 1,
        cpu_us: 0,
        memory_kb: 1,
        compile: None,
    }
}

#[tokio::test]
async fn language_servers_are_reported_and_connect_errors_are_typed() {
    let app = app("lsp").await;
    let info = app.app_info().await.unwrap();
    let langs: Vec<Language> = info.language_servers.iter().map(|l| l.language).collect();
    assert_eq!(langs, kb_judge::languages::SUPPORTED);
    // A fresh sandbox root has no servers installed.
    assert!(
        info.language_servers
            .iter()
            .all(|l| !l.available && l.reason.is_some())
    );
    let err = app
        .lsp_connect("two-sum", Language::Python, "", |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::LanguageServerUnavailable);
    let err = app
        .lsp_connect("../x", Language::Python, "", |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidInput);
    let err = app
        .lsp_send(Language::Python, 1, "{}".into())
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::LanguageServerNotConnected);
}

#[tokio::test]
async fn a_fresh_root_has_no_languages_and_says_what_installing_costs() {
    let app = app("languages").await;
    let info = app.languages();
    let langs: Vec<Language> = info.languages.iter().map(|l| l.language).collect();
    assert_eq!(
        langs,
        [
            Language::Python,
            Language::TypeScript,
            Language::Php,
            Language::Rust
        ]
    );
    for l in &info.languages {
        assert!(!l.installed && !l.runtime_installed, "{l:?}");
        if l.available {
            assert!(l.download_bytes > 0 && l.installed_bytes > 0, "{l:?}");
        }
    }
    assert_eq!(
        info.platform_supported,
        kb_toolchain::Platform::current().is_some()
    );
    // Nothing to remove is not an error; the server status agrees.
    app.uninstall_language(Language::Python).await.unwrap();
    let ls = app.app_info().await.unwrap().language_servers;
    assert!(ls.iter().all(|s| !s.available));
}

#[test]
fn toolchain_errors_keep_their_codes() {
    let e: AppError = kb_toolchain::Error::Busy.into();
    assert_eq!(e.code, AppErrorCode::InstallBusy);
    let e: AppError = kb_toolchain::Error::Cancelled.into();
    assert_eq!(e.code, AppErrorCode::InstallCancelled);
    assert_eq!(
        serde_json::to_value(AppErrorCode::NotEnoughSpace).unwrap(),
        json!("not_enough_space")
    );
}

#[tokio::test]
async fn without_python_the_validator_is_skipped_and_the_reference_decides() {
    let app = app("no-python").await;
    // two-sum has validate.py; a fresh root has no Python, so the input that breaks a
    // constraint is not refused by the validator: the flow goes on to the reference,
    // which cannot run here either.
    let short = json!({"nums": [1], "target": 2})
        .as_object()
        .unwrap()
        .clone();
    let err = app
        .run("two-sum", Language::Rust, "x", vec![short], |_| {})
        .await
        .unwrap_err();
    assert_ne!(err.code, AppErrorCode::InvalidCustomInput, "{err}");
    assert!(
        app.get_problem("two-sum", UiLang::En)
            .await
            .unwrap()
            .has_validator
    );
}

#[tokio::test]
async fn problem_limits_follow_the_stored_calibration() {
    let p = paths("calibration");
    let app = App::open(&p).await.unwrap();
    // Not measured yet: the reference machine's limits, base × language factor.
    assert_eq!(app.calibration(), None);
    let view = app.get_problem("two-sum", UiLang::En).await.unwrap();
    assert_eq!(view.time_scale, 1.0);
    let limit = |v: &ProblemView, l: Language| {
        v.time_limits
            .iter()
            .find(|x| x.language == l)
            .unwrap()
            .time_limit_ms
    };
    assert_eq!(limit(&view, Language::Rust), view.time_limit_ms);
    assert_eq!(limit(&view, Language::Python), view.time_limit_ms * 10);
    assert_eq!(view.time_limits.len(), kb_judge::languages::SUPPORTED.len());

    // A slower machine, measured earlier: every limit doubles.
    let stored = json!({
        "factor": 2.0, "applied": 2.0, "runtime": "python",
        "measured_us": 432000, "reference_us": 216000, "measured_at": 1_700_000_000
    });
    app.set_setting(CALIBRATION_SETTING, &stored).await.unwrap();
    app.close().await;
    let app = App::open(&p).await.unwrap();
    let c = app.calibration().unwrap();
    assert_eq!((c.applied, c.runtime), (2.0, Language::Python));
    let view = app.get_problem("two-sum", UiLang::En).await.unwrap();
    assert_eq!(view.time_scale, 2.0);
    assert_eq!(limit(&view, Language::Rust), view.time_limit_ms * 2);
    assert_eq!(limit(&view, Language::Php), view.time_limit_ms * 10);
    assert_eq!(app.env().time_scale, 2.0);
    app.close().await;
}

#[tokio::test]
async fn recalibrating_needs_python_or_node() {
    let app = app("no-benchmark").await;
    let err = app.recalibrate().await.unwrap_err();
    assert_eq!(err.code, AppErrorCode::CalibrationUnavailable);
    // A Run without runtimes does not measure (nor fail because of it).
    app.ensure_calibrated().await;
    assert_eq!(app.calibration(), None);
    app.close().await;
}

/// The practice bookkeeping of a Submit (ADR 0019), with recorded submissions and no
/// toolchain: the first Accepted schedules a review, the daily problem and the streak.
#[tokio::test]
async fn submits_drive_reviews_daily_problem_and_streak() {
    use crate::practice::ReviewOutcome;
    let app = app("practice").await;
    let today = "2026-11-14";
    let day: kb_core::Day = today.parse().unwrap();

    // The daily problem is picked once and kept for the day.
    let daily = app.daily_problem(today, UiLang::En).await.unwrap().unwrap();
    assert_eq!(daily.day, today);
    assert!(!daily.solved);
    for _ in 0..3 {
        let again = app.daily_problem(today, UiLang::En).await.unwrap().unwrap();
        assert_eq!(again.problem.id, daily.problem.id);
    }
    let id = daily.problem.id.clone();

    // A wrong answer changes nothing.
    app.store
        .user
        .record_submission(
            &id,
            Language::Python,
            "x",
            &report_with(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
    let out = app
        .after_submit(&id, Verdict::WrongAnswer, false, day, false, UiLang::En)
        .await
        .unwrap();
    assert!(!out.first_solve && !out.daily_solved && out.next.is_none());

    // The first Accepted: a review in 3 days, the daily problem solved, streak 1.
    app.store
        .user
        .record_submission(&id, Language::Python, "x", &report_with(Verdict::Accepted))
        .await
        .unwrap();
    let out = app
        .after_submit(&id, Verdict::Accepted, false, day, false, UiLang::En)
        .await
        .unwrap();
    assert!(out.first_solve && out.daily_solved);
    assert_eq!(out.streak, 1);
    let next = out.next.unwrap();
    assert_ne!(next.id, id);
    assert_ne!(next.status, Some(kb_store::ProblemStatus::Solved));
    let state = app.review_state(&id, today).await.unwrap().unwrap();
    assert_eq!((state.due_on.as_str(), state.due), ("2026-11-17", false));
    assert!(app.reviews_due(today, UiLang::En).await.unwrap().is_empty());
    let daily = app.daily_problem(today, UiLang::En).await.unwrap().unwrap();
    assert!(daily.solved);
    assert_eq!(
        daily.problem.id, id,
        "the pick stays all day, even once solved"
    );

    // Three days later it is due; a review-mode Accepted moves it to 7 days.
    let later = "2026-11-17";
    let due = app.reviews_due(later, UiLang::En).await.unwrap();
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].problem.id, id);
    let out = app
        .after_submit(
            &id,
            Verdict::Accepted,
            true,
            later.parse().unwrap(),
            true,
            UiLang::En,
        )
        .await
        .unwrap();
    assert_eq!(
        out.review,
        Some(ReviewOutcome::Scheduled {
            due_on: "2026-11-24".into(),
            interval_days: 7
        })
    );
    assert!(
        !out.first_solve,
        "only the first Accepted schedules a first review"
    );
    // The last solved daily problem was on the 14th: no streak on the 17th.
    assert_eq!(out.streak, 0);

    // Snooze, remove, add back.
    app.review_snooze(&id, "2026-11-24").await.unwrap();
    assert_eq!(
        app.review_state(&id, "2026-11-24")
            .await
            .unwrap()
            .unwrap()
            .due_on,
        "2026-11-25"
    );
    app.review_remove(&id).await.unwrap();
    assert_eq!(app.review_state(&id, "2026-11-24").await.unwrap(), None);
    app.review_add(&id, "2026-11-24").await.unwrap();
    assert_eq!(
        app.review_state(&id, "2026-11-24")
            .await
            .unwrap()
            .unwrap()
            .due_on,
        "2026-11-27"
    );

    // A new day picks a new daily problem among the unsolved ones.
    let tomorrow = app
        .daily_problem("2026-11-15", UiLang::En)
        .await
        .unwrap()
        .unwrap();
    assert!(!tomorrow.solved);
    assert_ne!(
        tomorrow.problem.status,
        Some(kb_store::ProblemStatus::Solved)
    );
    assert!(app.daily_problem("15/11/2026", UiLang::En).await.is_err());
    app.close().await;
}

#[tokio::test]
async fn lists_with_unavailable_problems_and_the_pattern_map() {
    let app = app("lists").await;
    let id = app.create_list("  Warm-up ").await.unwrap();
    app.add_to_list(id, "two-sum").await.unwrap();
    app.add_to_list(id, "gone-problem").await.unwrap(); // not in content
    app.add_to_list(id, "nested-brackets").await.unwrap();
    let lists = app.lists(UiLang::Es).await.unwrap();
    assert_eq!(lists[0].name, "Warm-up");
    assert_eq!(lists[0].total, 2, "unavailable problems do not count");
    let ids: Vec<&str> = lists[0]
        .entries
        .iter()
        .map(|e| e.problem_id.as_str())
        .collect();
    assert_eq!(ids, ["two-sum", "gone-problem", "nested-brackets"]);
    assert!(lists[0].entries[1].problem.is_none());
    assert_eq!(
        lists[0].entries[0].problem.as_ref().unwrap().title,
        "Suma de dos"
    );
    app.reorder_list(
        id,
        &[
            "nested-brackets".into(),
            "two-sum".into(),
            "gone-problem".into(),
        ],
    )
    .await
    .unwrap();
    assert_eq!(app.lists_with("two-sum").await.unwrap(), [id]);
    assert_eq!(
        app.create_list("").await.unwrap_err().code,
        AppErrorCode::InvalidInput
    );
    assert_eq!(
        app.reorder_list(id, &["two-sum".into()])
            .await
            .unwrap_err()
            .code,
        AppErrorCode::InvalidInput
    );
    app.rename_list(id, "Basics").await.unwrap();
    app.delete_list(id).await.unwrap();
    assert!(app.lists(UiLang::En).await.unwrap().is_empty());

    let map = app.patterns(UiLang::En).await.unwrap();
    assert_eq!(map[0].pattern, "arrays-hashing");
    let total: u32 = map.iter().map(|m| m.total).sum();
    assert_eq!(
        total as usize,
        app.list_problems(&ProblemFilter::default())
            .await
            .unwrap()
            .len()
    );
    app.close().await;
}
