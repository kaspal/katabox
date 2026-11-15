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
    assert_eq!(info.user_schema_version, 4); // migrations 0001-0004
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
    // And one with a value of the wrong type, naming the param (ADR 0025).
    let bad = json!({"nums": [1, "x"], "target": 3})
        .as_object()
        .unwrap()
        .clone();
    let err = app
        .run("two-sum", Language::Python, "x", vec![bad], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidCustomInput);
    assert!(
        err.message
            .contains("custom input 1: nums: [1]: expected an int"),
        "{}",
        err.message
    );
    // A malformed level-order tree is refused with the param and the reason.
    let bad = json!({"root": [1, null, null, 2]})
        .as_object()
        .unwrap()
        .clone();
    let err = app
        .run("hedge-trim", Language::Python, "x", vec![bad], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidCustomInput);
    assert!(
        err.message.contains("custom input 1: root: TreeNode") && err.message.contains("no parent"),
        "{}",
        err.message
    );
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
        stderr: None,
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

/// A brand-new user starts at the first problem of the first track pattern (from
/// `PATTERN_TRACK` and the ranks); after one solve the usual daily and next rules apply.
#[tokio::test]
async fn a_brand_new_user_starts_at_fundamentals() {
    use crate::practice::{PATTERN_TRACK, daily_pick, next_problem};
    let app = app("brand-new").await;
    let today = "2026-11-14";
    let all = app.all_problems(UiLang::En).await.unwrap();
    let start = all
        .iter()
        .filter(|p| p.pattern == PATTERN_TRACK[0])
        .min_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.id.cmp(&b.id)))
        .expect("the pack has fundamentals problems");

    // No solves: the daily problem and "next problem" both point to it.
    let daily = app.daily_problem(today, UiLang::En).await.unwrap().unwrap();
    assert_eq!(daily.problem.id, start.id);
    for from in ["two-sum", "nested-brackets"] {
        let next = app.next_problem(from, UiLang::En).await.unwrap().unwrap();
        assert_eq!(next.id, start.id, "next from {from}");
    }

    // One solve (two-sum, not the start problem): the usual rules.
    app.store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "x",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    let all = app.all_problems(UiLang::En).await.unwrap();
    let next = app
        .next_problem("two-sum", UiLang::En)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(Some(&next), next_problem(&all, "two-sum"));
    let two_sum = all.iter().find(|p| p.id == "two-sum").unwrap();
    assert_eq!(next.pattern, two_sum.pattern, "stays in two-sum's pattern");
    // Today's pick is kept all day; a new day takes the weighted pick.
    let tomorrow = "2026-11-15";
    let daily = app
        .daily_problem(tomorrow, UiLang::En)
        .await
        .unwrap()
        .unwrap();
    let install = app.store.user.install_id().await.unwrap();
    let expected = daily_pick(&all, tomorrow.parse().unwrap(), &install).unwrap();
    assert_eq!(daily.problem.id, expected.id);
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
    // The track starts with fundamentals (content to 150).
    assert_eq!(map[0].pattern, "fundamentals");
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

// --- Practice modes (ADR 0020) -------------------------------------------------------

fn real_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// A mock interview: fresh problems, wall-clock timer (survives reopening), every locked
/// command refused here for the session's problems, a scorecard at the end.
#[tokio::test]
async fn mock_interview_locks_resumes_and_scores() {
    use crate::interview::InterviewSetup;
    let p = paths("interview");
    let app = App::open(&p).await.unwrap();
    // Only two easy problems stay fresh (never opened).
    let all = app.list_problems(&ProblemFilter::default()).await.unwrap();
    for pr in &all {
        if pr.id != "lonely-sock" && pr.id != "same-letters" {
            app.mark_opened(&pr.id).await.unwrap();
        }
    }
    // Submissions are stamped with the real time, so the session starts now.
    let t0 = real_now_ms() - 1_000;
    app.set_clock(Some(t0));
    let setup = InterviewSetup {
        duration_min: 30,
        mix: "easy+easy".into(),
        languages: vec![Language::Python, Language::Rust],
    };
    for bad in [
        InterviewSetup {
            duration_min: 25,
            ..setup.clone()
        },
        InterviewSetup {
            mix: "easy+impossible".into(),
            ..setup.clone()
        },
        InterviewSetup {
            languages: vec![],
            ..setup.clone()
        },
    ] {
        assert_eq!(
            app.start_interview(&bad, UiLang::En)
                .await
                .unwrap_err()
                .code,
            AppErrorCode::InvalidInput
        );
    }
    let v = app.start_interview(&setup, UiLang::En).await.unwrap();
    let mut ids: Vec<String> = v.problems.iter().map(|p| p.id.clone()).collect();
    ids.sort();
    assert_eq!(ids, ["lonely-sock", "same-letters"]);
    assert_eq!((v.remaining_s, v.ends_at), (1800, t0 + 1_800_000));
    assert_eq!(
        app.start_interview(&setup, UiLang::En)
            .await
            .unwrap_err()
            .code,
        AppErrorCode::InterviewActive
    );

    // Every locked command is refused for the session's problems.
    let pid = "lonely-sock";
    let locked = |e: AppError| assert_eq!(e.code, AppErrorCode::LockedInInterview, "{e}");
    locked(app.editorial(pid, UiLang::En).await.unwrap_err());
    locked(app.unlock_hint(pid, 1).await.unwrap_err());
    locked(app.unlocked_hints(pid).await.unwrap_err());
    locked(app.list_submissions(pid).await.unwrap_err());
    locked(app.list_snapshots(pid).await.unwrap_err());
    locked(
        app.accepted_solution(pid, Language::Python)
            .await
            .unwrap_err(),
    );
    assert!(
        app.get_problem(pid, UiLang::En)
            .await
            .unwrap()
            .hints
            .is_empty()
    );
    // By id too: a submission or snapshot of a session problem.
    let wa = app
        .store
        .user
        .record_submission(
            pid,
            Language::Python,
            "x",
            &report_with(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
    locked(app.get_submission(wa).await.unwrap_err());
    let snap = app.store.user.list_snapshots(pid).await.unwrap()[0].id;
    locked(app.restore_snapshot(snap).await.unwrap_err());
    app.store
        .user
        .record_submission(pid, Language::Python, "y", &report_with(Verdict::Accepted))
        .await
        .unwrap();
    // Other problems and notes stay available.
    app.editorial("two-sum", UiLang::En).await.unwrap();
    app.set_note(pid, "scratch").await.unwrap();
    assert!(
        !app.get_problem("two-sum", UiLang::En)
            .await
            .unwrap()
            .hints
            .is_empty()
    );

    // Closing the app mid-session: the timer resumes from the wall clock.
    app.close().await;
    let app = App::open(&p).await.unwrap();
    app.set_clock(Some(t0 + 600_000));
    let v = app.interview_active(UiLang::En).await.unwrap().unwrap();
    assert_eq!(v.remaining_s, 1200);
    locked(app.editorial(pid, UiLang::En).await.unwrap_err());

    // Time runs out: the next query ends it, and everything unlocks.
    app.set_clock(Some(t0 + 1_800_000 + 1));
    assert_eq!(app.interview_active(UiLang::En).await.unwrap(), None);
    app.editorial(pid, UiLang::En).await.unwrap();
    assert_eq!(app.list_submissions(pid).await.unwrap().len(), 2);
    let history = app.interviews(UiLang::En).await.unwrap();
    assert_eq!(history.len(), 1);
    let r = &history[0];
    assert_eq!(
        (r.ended_at, r.used_s, r.solved, r.first_try),
        (t0 + 1_800_000, 1800, 1, 0)
    );
    let lonely = r.problems.iter().find(|p| p.problem_id == pid).unwrap();
    assert_eq!(
        (lonely.solved, lonely.submits, lonely.first_try),
        (true, 2, false)
    );
    assert_eq!(r.titles.len(), 2);

    // lonely-sock now has submissions: only one fresh problem is left.
    assert_eq!(
        app.start_interview(&setup, UiLang::En)
            .await
            .unwrap_err()
            .code,
        AppErrorCode::NotEnoughProblems
    );
    app.close().await;
}

#[tokio::test]
async fn ending_an_interview_early_scores_it_now() {
    use crate::interview::InterviewSetup;
    let app = app("interview-end").await;
    let t0 = real_now_ms() - 1_000;
    app.set_clock(Some(t0));
    let setup = InterviewSetup {
        duration_min: 45,
        mix: "medium+hard".into(),
        languages: vec![Language::TypeScript],
    };
    let v = app.start_interview(&setup, UiLang::Es).await.unwrap();
    assert_eq!(v.problems.len(), 2);
    // Only the session's languages may be judged on its problems.
    let other = v.problems[0].id.clone();
    let err = app
        .run(&other, Language::Python, "x", vec![], |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidInput);
    app.set_clock(Some(t0 + 300_000));
    let r = app.end_interview(UiLang::Es).await.unwrap();
    assert_eq!((r.used_s, r.solved), (300, 0));
    assert_eq!(app.interview_active(UiLang::Es).await.unwrap(), None);
    assert_eq!(
        app.end_interview(UiLang::Es).await.unwrap_err().code,
        AppErrorCode::NotFound
    );
    app.close().await;
}

/// Cross-language practice: accepted solution per language, solved counts, and a new
/// language never touches another language's draft, for all 12 ordered pairs.
#[tokio::test]
async fn cross_language_solutions_and_drafts() {
    let app = app("cross").await;
    let langs = kb_judge::languages::SUPPORTED;
    for from in langs {
        for to in langs {
            if from == to {
                continue;
            }
            let pid = "two-sum";
            app.store
                .user
                .record_submission(
                    pid,
                    from,
                    &format!("solution in {from}"),
                    &report_with(Verdict::Accepted),
                )
                .await
                .unwrap();
            let draft = format!("my {to} draft");
            app.save_draft(pid, to, &draft).await.unwrap();
            let sol = app.accepted_solution(pid, from).await.unwrap().unwrap();
            assert_eq!(sol.code, format!("solution in {from}"));
            assert_eq!(sol.language, from);
            // Opening the split view reads only: the target draft is untouched.
            assert_eq!(app.load_draft(pid, to).await.unwrap().unwrap().code, draft);
        }
    }
    let by = app.solved_by_language().await.unwrap();
    assert_eq!(by.len(), 4);
    assert!(by.iter().all(|c| c.solved == 1), "{by:?}");
    assert_eq!(app.solved_languages("two-sum").await.unwrap().len(), 4);
    assert_eq!(
        app.accepted_solution("nested-brackets", Language::Rust)
            .await
            .unwrap(),
        None
    );
    app.close().await;
}

/// Export → fresh data folder → import reproduces Progress and Home exactly.
#[tokio::test]
async fn export_import_round_trip_reproduces_progress_and_home() {
    let today = "2026-11-14";
    let a = app("export-a").await;
    // Progress through the API and the store, as the app does.
    for (pid, lang, v) in [
        ("two-sum", Language::Python, Verdict::WrongAnswer),
        ("two-sum", Language::Python, Verdict::Accepted),
        ("two-sum", Language::Rust, Verdict::Accepted),
        ("nested-brackets", Language::Php, Verdict::TimeLimitExceeded),
        ("budget-pair", Language::TypeScript, Verdict::Accepted),
    ] {
        a.store
            .user
            .record_submission(pid, lang, &format!("{pid} {lang}"), &report_with(v))
            .await
            .unwrap();
        a.after_submit(pid, v, false, today.parse().unwrap(), false, UiLang::En)
            .await
            .unwrap();
    }
    a.save_draft("nested-brackets", Language::Php, "<?php // wip")
        .await
        .unwrap();
    a.set_note("two-sum", "use a map").await.unwrap();
    a.unlock_hint("nested-brackets", 1).await.unwrap();
    a.mark_opened("rising-towers").await.unwrap();
    let daily = a.daily_problem(today, UiLang::En).await.unwrap().unwrap();
    a.review_add("nested-brackets", today).await.unwrap();
    let list = a.create_list("Warm-up").await.unwrap();
    a.add_to_list(list, "two-sum").await.unwrap();
    a.add_to_list(list, "retired-one").await.unwrap();
    a.set_setting("ui_lang", &json!("es")).await.unwrap();
    a.set_setting("editor.keymap", &json!("vim")).await.unwrap();
    // Learn progress travels (ADR 0029); the Learn layout stays home and must not break
    // the import.
    let learn_last = json!({"subject": "sorting", "topic": "quick-sort", "step": 14});
    a.set_setting("learn.viewed", &json!(["bubble-sort", "quick-sort"]))
        .await
        .unwrap();
    a.set_setting("learn.last", &learn_last).await.unwrap();
    a.set_setting("learn.layout", &json!([20, 50, 30]))
        .await
        .unwrap();
    a.set_setting("app.install_id", &json!("machine-a"))
        .await
        .unwrap();
    {
        use crate::interview::InterviewSetup;
        a.set_clock(Some(real_now_ms() - 1_000));
        a.start_interview(
            &InterviewSetup {
                duration_min: 30,
                mix: "easy+medium".into(),
                languages: vec![Language::Python],
            },
            UiLang::En,
        )
        .await
        .unwrap();
        a.end_interview(UiLang::En).await.unwrap();
        a.set_clock(None);
    }

    let dir = crate::testutil::temp_dir("export-file");
    let file = dir.join("progress.kbx.json");
    let counts = a.export_to(&file).await.unwrap();
    assert_eq!(counts.submissions, 5);
    assert_eq!(counts.interviews, 1);
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("\"format\": \"katabox-export/v1\""));
    assert!(
        !text.contains("machine-a"),
        "machine-specific settings stay home"
    );

    let b = app("export-b").await;
    let preview = b.import_preview_from(&file).await.unwrap();
    assert_eq!(preview.counts, counts);
    assert_eq!(preview.conflicts, 0);
    let r = b.import_apply(preview.id).await.unwrap();
    assert!(r.added > 0);
    assert_eq!(
        b.import_apply(preview.id).await.unwrap_err().code,
        AppErrorCode::NotFound,
        "a preview applies once"
    );

    // Progress and Home, compared through the same queries the screens use.
    for lang in [UiLang::En, UiLang::Es] {
        let f = ProblemFilter {
            lang: Some(lang),
            ..Default::default()
        };
        assert_eq!(
            a.list_problems(&f).await.unwrap(),
            b.list_problems(&f).await.unwrap()
        );
        assert_eq!(
            a.patterns(lang).await.unwrap(),
            b.patterns(lang).await.unwrap()
        );
        assert_eq!(a.lists(lang).await.unwrap(), b.lists(lang).await.unwrap());
        assert_eq!(
            a.continue_item(lang).await.unwrap(),
            b.continue_item(lang).await.unwrap()
        );
        assert_eq!(
            a.reviews_due("2026-11-20", lang).await.unwrap(),
            b.reviews_due("2026-11-20", lang).await.unwrap()
        );
        assert_eq!(
            a.daily_problem(today, lang).await.unwrap(),
            b.daily_problem(today, lang).await.unwrap()
        );
        let (ia, ib) = (
            a.interviews(lang).await.unwrap(),
            b.interviews(lang).await.unwrap(),
        );
        assert_eq!(ia.len(), ib.len());
        assert_eq!(
            ia.iter()
                .map(|r| {
                    let mut r = r.clone();
                    r.id = 0;
                    r
                })
                .collect::<Vec<_>>(),
            ib.iter()
                .map(|r| {
                    let mut r = r.clone();
                    r.id = 0;
                    r
                })
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(
        a.streak(today).await.unwrap(),
        b.streak(today).await.unwrap()
    );
    assert_eq!(
        a.solved_by_language().await.unwrap(),
        b.solved_by_language().await.unwrap()
    );
    assert_eq!(b.get_setting("ui_lang").await.unwrap(), Some(json!("es")));
    assert_eq!(
        b.get_setting("learn.viewed").await.unwrap(),
        Some(json!(["bubble-sort", "quick-sort"]))
    );
    assert_eq!(b.get_setting("learn.last").await.unwrap(), Some(learn_last));
    assert_eq!(b.get_setting("learn.layout").await.unwrap(), None);
    assert!(
        !b.get_setting("app.install_id")
            .await
            .unwrap()
            .is_some_and(|v| v == json!("machine-a"))
    );
    assert_eq!(
        b.daily_problem(today, UiLang::En)
            .await
            .unwrap()
            .unwrap()
            .problem
            .id,
        daily.problem.id
    );
    assert_eq!(b.unlocked_hints("nested-brackets").await.unwrap(), [1]);
    assert_eq!(
        b.get_note("two-sum").await.unwrap().as_deref(),
        Some("use a map")
    );

    // Importing again changes nothing (idempotent).
    let again = b.import_preview_from(&file).await.unwrap();
    assert!(again.conflicts > 0);
    let r2 = b.import_apply(again.id).await.unwrap();
    assert_eq!((r2.added, r2.updated), (0, 0), "{r2:?}");
    a.close().await;
    b.close().await;
}

#[tokio::test]
async fn import_keeps_the_newer_record_and_never_downgrades() {
    let a = app("merge-a").await;
    a.store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "old",
            &report_with(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
    a.set_note("two-sum", "old note").await.unwrap();
    let dir = crate::testutil::temp_dir("merge-file");
    let file = dir.join("a.kbx.json");
    a.export_to(&file).await.unwrap();

    // B solved two-sum and wrote a newer note.
    let b = app("merge-b").await;
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    b.store
        .user
        .record_submission(
            "two-sum",
            Language::Rust,
            "new",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    b.set_note("two-sum", "newer note").await.unwrap();
    let preview = b.import_preview_from(&file).await.unwrap();
    assert_eq!(preview.conflicts, 2); // status and note
    b.import_apply(preview.id).await.unwrap();
    let st = b
        .store
        .user
        .problem_status("two-sum")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        st.status,
        kb_store::ProblemStatus::Solved,
        "solved never downgrades"
    );
    assert_eq!(
        b.get_note("two-sum").await.unwrap().as_deref(),
        Some("newer note")
    );
    assert_eq!(b.list_submissions("two-sum").await.unwrap().len(), 2);
    a.close().await;
    b.close().await;
}

#[tokio::test]
async fn import_rejects_bad_files_and_imports_nothing() {
    let a = app("reject-a").await;
    a.store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "code",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    a.review_add("two-sum", "2026-11-14").await.unwrap();
    a.set_setting("theme", &json!("dark")).await.unwrap();
    let dir = crate::testutil::temp_dir("reject");
    let good = dir.join("good.kbx.json");
    a.export_to(&good).await.unwrap();
    let base: Value = serde_json::from_str(&std::fs::read_to_string(&good).unwrap()).unwrap();

    let b = app("reject-b").await;
    type Mutation = Box<dyn Fn(&mut Value)>;
    let cases: Vec<(&str, Mutation)> = vec![
        (
            "format",
            Box::new(|v| v["format"] = json!("katabox-export/v2")),
        ),
        (
            "unknown field",
            Box::new(|v| v["data"]["extra"] = json!([])),
        ),
        (
            "unknown row field",
            Box::new(|v| v["data"]["submissions"][0]["shell"] = json!("rm -rf")),
        ),
        (
            "wrong type",
            Box::new(|v| v["data"]["submissions"][0]["time_us"] = json!("fast")),
        ),
        (
            "bad id",
            Box::new(|v| v["data"]["submissions"][0]["problem_id"] = json!("../../etc")),
        ),
        (
            "bad language",
            Box::new(|v| v["data"]["submissions"][0]["language"] = json!("cobol")),
        ),
        (
            "bad verdict",
            Box::new(|v| v["data"]["submissions"][0]["verdict"] = json!("passed")),
        ),
        (
            "bad date",
            Box::new(|v| v["data"]["reviews"][0]["due_on"] = json!("2026-02-30")),
        ),
        (
            "setting key",
            Box::new(
                |v| v["data"]["settings"] = json!([{"key": "sandbox_root", "value": "C:/", "updated_at": 1_800_000_000_000_i64}]),
            ),
        ),
        (
            "long code",
            Box::new(|v| v["data"]["submissions"][0]["code"] = json!("x".repeat(1024 * 1024 + 1))),
        ),
        (
            "time range",
            Box::new(|v| v["data"]["submissions"][0]["created_at"] = json!(5)),
        ),
    ];
    for (name, mutate) in cases {
        let mut v = base.clone();
        mutate(&mut v);
        let f = dir.join(format!("{}.kbx.json", name.replace(' ', "-")));
        std::fs::write(&f, serde_json::to_vec(&v).unwrap()).unwrap();
        let err = b.import_preview_from(&f).await.unwrap_err();
        assert_eq!(err.code, AppErrorCode::ImportInvalid, "{name}: {err}");
    }
    // Not JSON at all, and over the size cap.
    let junk = dir.join("junk.kbx.json");
    std::fs::write(&junk, b"#!/bin/sh\necho hi").unwrap();
    assert_eq!(
        b.import_preview_from(&junk).await.unwrap_err().code,
        AppErrorCode::ImportInvalid
    );
    let big = dir.join("big.kbx.json");
    let f = std::fs::File::create(&big).unwrap();
    f.set_len(crate::transfer::IMPORT_MAX_BYTES + 1).unwrap();
    let err = b.import_preview_from(&big).await.unwrap_err();
    assert!(err.message.contains("larger than"), "{err}");
    // Nothing was imported.
    assert!(b.list_submissions("two-sum").await.unwrap().is_empty());
    a.close().await;
    b.close().await;
}

#[tokio::test]
async fn learn_commands_read_the_bundled_topics() {
    let a = app("learn").await;
    let catalog = a.learn_catalog(UiLang::En).await.unwrap();
    assert_eq!(catalog.len(), 14, "every subject, with or without topics");
    let sorting = catalog.iter().find(|s| s.id == "sorting").unwrap();
    let ids: Vec<_> = sorting.topics.iter().map(|t| t.id.as_str()).collect();
    // In topic.toml `order`: the Phase 0 sorts first, then the Phase 1 ones.
    assert_eq!(
        ids,
        [
            "bubble-sort",
            "insertion-sort",
            "quick-sort",
            "selection-sort",
            "shell-sort",
            "merge-sort",
            "quick-sort-3way",
            "heap-sort",
            "counting-sort",
            "radix-sort",
            "bucket-sort",
            "tim-sort",
            "quickselect",
        ]
    );
    // Phases 2-6 filled every subject (no "Coming soon" left).
    assert!(
        catalog.iter().all(|s| !s.topics.is_empty()),
        "every subject has topics"
    );

    let view = a.learn_topic("quick-sort", UiLang::Es).await.unwrap();
    assert_eq!(view.topic.subject_name, "Ordenación");
    let linked: Vec<_> = view.problems.iter().map(|p| p.id.as_str()).collect();
    assert_eq!(
        linked,
        view.topic
            .problems
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(view.topic.samples.len(), 2);

    let err = a
        .learn_topic("no-such-topic", UiLang::En)
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::NotFound);
    let found = a.search_learn("pivote", UiLang::En).await.unwrap();
    assert_eq!(found[0].id, "quick-sort");
}

#[tokio::test]
async fn diagnostics_name_versions_but_no_paths_or_user() {
    let p = paths("diagnostics");
    let app = App::open(&p).await.unwrap();
    let d = app.diagnostics().await.unwrap();
    assert_eq!(d.app_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(d.os, std::env::consts::OS);
    assert!(d.content_pack.starts_with("test "), "{}", d.content_pack);
    assert_eq!(d.runtimes.len(), kb_judge::languages::SUPPORTED.len());
    if kb_toolchain::Platform::current().is_some() {
        let py = d
            .runtimes
            .iter()
            .find(|r| r.language == kb_core::Language::Python)
            .unwrap();
        assert!(
            py.version
                .as_deref()
                .is_some_and(|v| v.starts_with("python-")),
            "{py:?}"
        );
    }
    let text = serde_json::to_string(&d).unwrap();
    let mut private: Vec<String> = vec![
        p.user_db.parent().unwrap().display().to_string(),
        app.sandbox_root().display().to_string(),
    ];
    for var in ["USERNAME", "USER", "USERPROFILE", "HOME"] {
        if let Ok(v) = std::env::var(var)
            && v.len() > 2
        {
            private.push(v);
        }
    }
    for needle in private {
        assert!(
            !text.contains(&needle),
            "diagnostics contain {needle:?}: {text}"
        );
        let escaped = serde_json::to_string(&needle).unwrap();
        assert!(
            !text.contains(escaped.trim_matches('"')),
            "diagnostics contain {needle:?}"
        );
    }
    assert!(!text.contains(":\\") && !text.contains("/home/"), "{text}");
}

#[tokio::test]
async fn finishing_the_welcome_sets_the_start_floor_and_picks_a_problem() {
    let p = paths("onboarding");
    let app = App::open(&p).await.unwrap();
    let placement = app.placement_problems(kb_store::UiLang::En).await.unwrap();
    assert_eq!(placement.len(), 3);
    assert!(
        placement
            .iter()
            .all(|x| x.mode == kb_core::ProblemMode::Function)
    );
    // Nothing solved: the placement starts the user at fundamentals.
    let ids = placement.iter().map(|x| x.id.clone()).collect();
    let start = app
        .finish_onboarding(Some(ids), kb_store::UiLang::En)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(start.pattern, "fundamentals");
    let floor = app.start_floor().await.unwrap().unwrap();
    assert_eq!(floor.pattern, "fundamentals");
    assert_eq!(
        app.get_setting(crate::practice::ONBOARDING_SETTING)
            .await
            .unwrap(),
        Some(serde_json::Value::Bool(true))
    );
}
