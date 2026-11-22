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
    assert_eq!(info.user_schema_version, 13); // migrations 0001-0013
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
        kata: None,
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
    use crate::practice::{PATTERN_TRACK, daily_pick_rated, hide_unopened_sql, next_problem_rated};
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
    app.forget_ratings(); // the store call above bypasses Submit
    // No SQL problem opened yet: SQL is never suggested (ADR 0031); fix-the-bug problems
    // never are (expansion M6).
    let all = hide_unopened_sql(
        crate::practice::without_fix(app.all_problems(UiLang::En).await.unwrap()),
        "",
    );
    assert!(all.iter().all(|p| p.mode != kb_core::ProblemMode::Sql));
    let next = app
        .next_problem("two-sum", UiLang::En)
        .await
        .unwrap()
        .unwrap();
    // The rating (expansion M2) is a soft weight on both picks.
    let ratings = app.current_ratings().await.unwrap().by_pattern();
    assert_eq!(ratings.len(), 1, "one solve rates one pattern");
    assert_eq!(Some(&next), next_problem_rated(&all, "two-sum", &ratings));
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
    let expected = daily_pick_rated(&all, tomorrow.parse().unwrap(), &install, &ratings).unwrap();
    assert_eq!(daily.problem.id, expected.id);
    assert_ne!(daily.problem.mode, kb_core::ProblemMode::Sql);
    assert!(!daily.problem.fix);
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
    assert!(text.contains("\"format\": \"katabox-export/v2\""));
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

/// A file written by 1.0.0-beta.1 (`katabox-export/v1`) still imports after v2.
#[tokio::test]
async fn a_v1_export_still_imports() {
    let dir = crate::testutil::temp_dir("export-v1");
    let file = dir.join("old.kbx.json");
    std::fs::write(&file, include_str!("../tests/fixtures/export-v1.kbx.json")).unwrap();
    let b = app("export-v1").await;
    let preview = b.import_preview_from(&file).await.unwrap();
    assert_eq!(preview.counts.submissions, 2);
    assert_eq!(preview.counts.lists, 1);
    let r = b.import_apply(preview.id).await.unwrap();
    assert!(r.added > 0);
    assert_eq!(
        b.store.user.get_note("two-sum").await.unwrap().as_deref(),
        Some("use a map")
    );
    assert_eq!(
        b.get_setting("editor.keymap").await.unwrap(),
        Some(json!("vim"))
    );
    // Re-exported, it is a v2 file that imports again.
    let again = dir.join("again.kbx.json");
    b.export_to(&again).await.unwrap();
    assert!(
        std::fs::read_to_string(&again)
            .unwrap()
            .contains("katabox-export/v2")
    );
    let c = app("export-v1-again").await;
    let p = c.import_preview_from(&again).await.unwrap();
    assert_eq!(p.counts.submissions, 2);
    b.close().await;
    c.close().await;
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
            Box::new(|v| v["format"] = json!("katabox-export/v3")),
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

/// Kata drills (expansion M1): start from solved problems only, lock the problem's
/// history and help while running, keep drafts out, and travel with an export.
#[tokio::test]
async fn kata_sessions_lock_and_export() {
    use crate::kata::KataStart;
    use kb_store::kata::KataKind;
    let today = "2026-11-21";
    let a = app("kata-a").await;
    let start = |kind: KataKind, source: Option<&str>| KataStart {
        kind,
        source: source.map(str::to_string),
        problems: vec![],
        today: today.into(),
    };
    // Nothing solved yet: no kata.
    let err = a
        .start_kata(&start(KataKind::Daily, None), UiLang::En)
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidInput);
    for pid in ["two-sum", "budget-pair"] {
        a.store
            .user
            .record_submission(
                pid,
                Language::Python,
                "solved",
                &report_with(Verdict::Accepted),
            )
            .await
            .unwrap();
    }
    a.set_note("two-sum", "use a map").await.unwrap();
    let o = a.kata_options(today, UiLang::En).await.unwrap();
    assert_eq!(o.daily.len(), 2, "only two problems are solved");
    assert!(o.running.is_none());
    // Unsolved problems never join a custom kata.
    let mut custom = start(KataKind::Custom, None);
    custom.problems = vec!["nested-brackets".into()];
    assert_eq!(
        a.start_kata(&custom, UiLang::En).await.unwrap_err().code,
        AppErrorCode::InvalidInput
    );

    let view = a
        .start_kata(&start(KataKind::Daily, None), UiLang::En)
        .await
        .unwrap();
    assert_eq!(view.problems.len(), 2);
    assert_eq!(view.duration_s, crate::kata::DAILY_KATA_S);
    // One kata at a time, and no mock interview meanwhile.
    assert_eq!(
        a.start_kata(
            &start(KataKind::Pattern, Some("arrays-hashing")),
            UiLang::En
        )
        .await
        .unwrap_err()
        .code,
        AppErrorCode::KataActive
    );
    // Locked while running: submissions, notes, hints; drafts are not saved.
    assert_eq!(
        a.list_submissions("two-sum").await.unwrap_err().code,
        AppErrorCode::LockedInKata
    );
    assert_eq!(
        a.get_note("two-sum").await.unwrap_err().code,
        AppErrorCode::LockedInKata
    );
    assert_eq!(
        a.unlocked_hints("two-sum").await.unwrap_err().code,
        AppErrorCode::LockedInKata
    );
    assert!(
        a.get_problem("two-sum", UiLang::En)
            .await
            .unwrap()
            .hints
            .is_empty()
    );
    a.save_draft("two-sum", Language::Python, "from memory")
        .await
        .unwrap();
    assert_eq!(
        a.store
            .user
            .load_draft("two-sum", Language::Python)
            .await
            .unwrap(),
        None
    );
    // Problems outside the kata stay open.
    a.list_submissions("nested-brackets").await.unwrap();

    let attempt = a.kata_open("two-sum").await.unwrap();
    assert_eq!(attempt.submits, 0);
    let outcome = a
        .record_kata(
            view.started_at,
            "two-sum",
            Language::Python,
            "def twoSum(): ...",
            Verdict::Accepted,
        )
        .await
        .unwrap();
    assert!(outcome.new_best && outcome.previous_best_ms.is_none());
    assert_eq!(outcome.attempt.submits, 1);
    // A kata solve is not a submission.
    assert_eq!(
        a.store
            .user
            .list_submissions("two-sum")
            .await
            .unwrap()
            .len(),
        1
    );

    let ended = a.end_kata(UiLang::En).await.unwrap().unwrap();
    assert_eq!(ended.solved, 1);
    assert_eq!(
        a.get_note("two-sum").await.unwrap().as_deref(),
        Some("use a map")
    );
    // Today's daily kata runs once.
    assert_eq!(
        a.start_kata(&start(KataKind::Daily, None), UiLang::En)
            .await
            .unwrap_err()
            .code,
        AppErrorCode::InvalidInput
    );
    let overview = a.kata_overview(today, UiLang::En).await.unwrap();
    assert_eq!(overview.streak, 1);
    assert_eq!(overview.bests.len(), 1);
    assert_eq!(overview.sessions.len(), 1);

    // Export → import carries the sessions and attempts.
    let dir = crate::testutil::temp_dir("kata-export");
    let file = dir.join("kata.kbx.json");
    let counts = a.export_to(&file).await.unwrap();
    assert_eq!((counts.kata_sessions, counts.kata_attempts), (1, 1));
    let b = app("kata-b").await;
    let p = b.import_preview_from(&file).await.unwrap();
    b.import_apply(p.id).await.unwrap();
    let ob = b.kata_overview(today, UiLang::En).await.unwrap();
    assert_eq!(ob.streak, 1);
    assert_eq!(ob.bests, overview.bests);
    // A second import changes nothing.
    let p = b.import_preview_from(&file).await.unwrap();
    let r = b.import_apply(p.id).await.unwrap();
    assert_eq!(r.added, 0);
    a.close().await;
    b.close().await;
}

/// Study plans and ratings (expansion M2): a plan from a goal, done and rescheduled
/// items, the calendar file, and both travelling with an export; ratings replayed from
/// submissions.
#[tokio::test]
async fn study_plans_and_ratings() {
    use crate::plan_app::{PlanIcsLabels, PlanSetup};
    let a = app("plan-a").await;
    assert_eq!(a.plan_view(UiLang::En).await.unwrap(), None);
    let r0 = a.ratings().await.unwrap();
    assert_eq!((r0.overall.rating, r0.overall.events), (1200, 0));

    let setup = PlanSetup {
        patterns: vec!["arrays-hashing".into()],
        weeks: 1,
        minutes_per_day: 40,
        today: "2026-11-23".into(),
    };
    let bad = PlanSetup {
        weeks: 0,
        ..setup.clone()
    };
    assert_eq!(
        a.create_plan(&bad, UiLang::En).await.unwrap_err().code,
        AppErrorCode::InvalidInput
    );
    let plan = a.create_plan(&setup, UiLang::En).await.unwrap();
    assert_eq!(plan.patterns, ["arrays-hashing"]);
    assert_eq!(
        (plan.start_day.as_str(), plan.end_day.as_str()),
        ("2026-11-23", "2026-11-29")
    );
    assert!(plan.total > 0);
    let first = plan
        .items
        .iter()
        .find(|i| i.kind == crate::plan::PlanItemKind::Problem)
        .unwrap()
        .clone();
    assert!(first.title.is_some());

    // Solving a planned problem marks its item done; ratings move.
    a.store
        .user
        .record_submission(
            first.reference.as_deref().unwrap(),
            Language::Python,
            "x",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    a.forget_ratings();
    let v = a.plan_view(UiLang::En).await.unwrap().unwrap();
    let it = v
        .items
        .iter()
        .find(|i| i.position == first.position)
        .unwrap();
    assert_eq!(it.status, "done");
    let r = a.ratings().await.unwrap();
    assert_eq!(r.overall.events, 1);
    assert!(r.overall.rating > 1200);
    assert_eq!(r.patterns[0].pattern, "arrays-hashing");

    // Skip one, reschedule another (it and the pending items after it move a day).
    let pending: Vec<_> = v
        .items
        .iter()
        .filter(|i| i.status == "pending")
        .cloned()
        .collect();
    a.set_plan_item(v.created_at, pending[0].position, "skipped")
        .await
        .unwrap();
    assert!(
        a.set_plan_item(v.created_at, pending[0].position, "never")
            .await
            .is_err()
    );
    if pending.len() > 1 {
        a.reschedule_plan_item(v.created_at, pending[1].position)
            .await
            .unwrap();
        let after = a.plan_view(UiLang::En).await.unwrap().unwrap();
        let moved = after
            .items
            .iter()
            .find(|i| i.position == pending[1].position)
            .unwrap();
        let d: kb_core::Day = pending[1].day.parse().unwrap();
        assert_eq!(moved.day, d.add_days(1).to_string());
    }

    // The calendar file.
    let labels = PlanIcsLabels {
        title: "KataBox plan".into(),
        review: "Reviews".into(),
        kata: "Kata".into(),
        topic: "Learn: ".into(),
        minutes: "min".into(),
    };
    let dir = crate::testutil::temp_dir("plan-ics");
    let ics = dir.join("plan.ics");
    a.export_plan_ics(&ics, &labels, UiLang::En).await.unwrap();
    let text = std::fs::read_to_string(&ics).unwrap();
    assert!(text.starts_with("BEGIN:VCALENDAR\r\n") && text.contains("BEGIN:VEVENT"));

    // Export → import: the plan (still the active one) and its items.
    let file = dir.join("plan.kbx.json");
    let counts = a.export_to(&file).await.unwrap();
    assert_eq!(counts.study_plans, 1);
    assert_eq!(counts.plan_items, plan.total);
    let b = app("plan-b").await;
    let p = b.import_preview_from(&file).await.unwrap();
    b.import_apply(p.id).await.unwrap();
    let vb = b.plan_view(UiLang::En).await.unwrap().unwrap();
    assert_eq!(vb.created_at, v.created_at);
    assert_eq!(vb.total, plan.total);
    assert_eq!(
        b.ratings().await.unwrap().overall.events,
        1,
        "replayed from the imported history"
    );
    a.close().await;
    b.close().await;
}

/// The rating replay (expansion M2) stays fast: about 500 submissions over the whole
/// catalogue, replayed from user.db (the queries, the content list and the replay).
#[tokio::test]
async fn rating_replay_on_500_submissions() {
    let a = app("rating-500").await;
    let all = a.all_problems(UiLang::En).await.unwrap();
    let mut n = 0;
    'outer: for round in 0..10 {
        for p in &all {
            let v = if round % 3 == 2 {
                Verdict::Accepted
            } else {
                Verdict::WrongAnswer
            };
            a.store
                .user
                .record_submission(&p.id, Language::Python, "x", &report_with(v))
                .await
                .unwrap();
            n += 1;
            if n == 500 {
                break 'outer;
            }
        }
    }
    a.forget_ratings();
    let t = std::time::Instant::now();
    let r = a.ratings().await.unwrap();
    let cold = t.elapsed();
    let t = std::time::Instant::now();
    a.ratings().await.unwrap();
    let warm = t.elapsed();
    eprintln!("rating replay, {n} submissions: cold {cold:?}, cached {warm:?}");
    assert!(r.overall.events > 0);
    assert!(cold < std::time::Duration::from_secs(2), "{cold:?}");
    a.close().await;
}

/// The mistake journal and reflections (expansion M3): old and new failures are tagged,
/// the user edits tags and notes, the shrinker adds its tag, reflections are checked,
/// and all of it travels with an export.
#[tokio::test]
async fn mistake_journal_and_reflections() {
    let a = app("journal-a").await;
    // A failure made "before the journal": tagged on first look.
    let old = a
        .store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "x",
            &report_with(Verdict::WrongAnswer),
        )
        .await
        .unwrap();
    let tle = a
        .store
        .user
        .record_submission(
            "nested-brackets",
            Language::Rust,
            "y",
            &report_with(Verdict::TimeLimitExceeded),
        )
        .await
        .unwrap();
    let ok = a
        .store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "z",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    let j = a.journal_overview(UiLang::En).await.unwrap();
    assert_eq!(j.failures, 2);
    let tags: Vec<(&str, u32)> = j.tags.iter().map(|t| (t.tag.as_str(), t.count)).collect();
    assert_eq!(tags, [("wa", 1), ("tle", 1)]);
    assert_eq!(j.tags[0].examples[0].title, "Two Sum");

    assert_eq!(a.submission_mistake(ok).await.unwrap(), None);
    let m = a.submission_mistake(old).await.unwrap().unwrap();
    assert_eq!(m.auto_tags, ["wa"]);
    a.set_submission_mistake(old, &["wa".into(), "edge:empty".into()], "empty input")
        .await
        .unwrap();
    assert_eq!(
        a.set_submission_mistake(old, &["nonsense".into()], "")
            .await
            .unwrap_err()
            .code,
        AppErrorCode::InvalidInput
    );
    assert!(
        a.set_submission_mistake(ok, &["wa".into()], "")
            .await
            .is_err()
    );
    a.tag_shrunk(tle).await.unwrap();
    let m = a.submission_mistake(tle).await.unwrap().unwrap();
    assert_eq!(m.tags, ["tle", "shrunk"]);

    assert_eq!(a.reflection("two-sum").await.unwrap(), None);
    a.set_reflection(
        "two-sum",
        " one pass with a map ",
        Some("O(n)"),
        Some("O(n)"),
    )
    .await
    .unwrap();
    assert!(
        a.set_reflection("two-sum", "x", Some("O(fast)"), None)
            .await
            .is_err()
    );
    assert!(
        a.set_reflection("two-sum", &"x".repeat(501), None, None)
            .await
            .is_err()
    );
    assert_eq!(
        a.reflection("two-sum").await.unwrap().unwrap().why,
        "one pass with a map"
    );

    let dir = crate::testutil::temp_dir("journal-export");
    let file = dir.join("j.kbx.json");
    let counts = a.export_to(&file).await.unwrap();
    assert_eq!((counts.mistakes, counts.reflections), (2, 1));
    let b = app("journal-b").await;
    let p = b.import_preview_from(&file).await.unwrap();
    b.import_apply(p.id).await.unwrap();
    let jb = b.journal_overview(UiLang::En).await.unwrap();
    assert_eq!(jb.failures, 2);
    assert!(
        jb.tags
            .iter()
            .any(|t| t.tag == "edge:empty" && t.examples[0].note == "empty input")
    );
    assert_eq!(
        b.reflection("two-sum")
            .await
            .unwrap()
            .unwrap()
            .time_class
            .as_deref(),
        Some("O(n)")
    );
    a.close().await;
    b.close().await;
}

/// Snippets and "Search everything" (expansion M4): save, edit, delete, search across
/// problems, topics and snippets, and export the library.
#[tokio::test]
async fn snippets_and_search_everything() {
    use crate::comforts::SnippetInput;
    let a = app("snip-a").await;
    let s = a
        .save_snippet(&SnippetInput {
            uid: None,
            language: Language::Python,
            name: "Binary search".into(),
            tags: vec!["Search".into()],
            body: "lo, hi = 0, len(a) - 1".into(),
        })
        .await
        .unwrap();
    assert_eq!(s.uid.len(), 16);
    assert_eq!(s.tags, ["search"]);
    let edited = a
        .save_snippet(&SnippetInput {
            uid: Some(s.uid.clone()),
            language: Language::Python,
            name: "Binary search (lower bound)".into(),
            tags: vec![],
            body: "lo, hi = 0, len(a)".into(),
        })
        .await
        .unwrap();
    assert_eq!(
        (edited.uid.as_str(), edited.created_at),
        (s.uid.as_str(), s.created_at)
    );
    assert!(
        a.save_snippet(&SnippetInput {
            uid: None,
            language: Language::Sql,
            name: "x".into(),
            tags: vec![],
            body: "SELECT 1".into(),
        })
        .await
        .is_err()
    );
    assert_eq!(a.snippets().await.unwrap().len(), 1);

    let r = a.search_all("two sum", UiLang::En).await.unwrap();
    assert!(r.problems.iter().any(|p| p.id == "two-sum"));
    let r = a.search_all("lower bound", UiLang::En).await.unwrap();
    assert_eq!(r.snippets.len(), 1);
    assert!(
        a.search_all("  ", UiLang::En)
            .await
            .unwrap()
            .problems
            .is_empty()
    );

    let dir = crate::testutil::temp_dir("snip-export");
    let file = dir.join("s.kbx.json");
    assert_eq!(a.export_to(&file).await.unwrap().snippets, 1);
    let b = app("snip-b").await;
    let p = b.import_preview_from(&file).await.unwrap();
    b.import_apply(p.id).await.unwrap();
    assert_eq!(b.snippets().await.unwrap(), a.snippets().await.unwrap());
    a.delete_snippet(&s.uid).await.unwrap();
    assert!(a.snippets().await.unwrap().is_empty());
    a.close().await;
    b.close().await;
}

/// Reading exercises (expansion M7): the bundled sets, the answer check, progress and the
/// export round trip.
#[tokio::test]
async fn reading_answers_are_checked_counted_and_exported() {
    use crate::reading::ReadingAnswer;
    let a = app("reading").await;
    let sets = a.reading_sets(UiLang::En).await.unwrap();
    assert_eq!(sets.len(), 8);
    assert_eq!(sets.iter().map(|s| s.items).sum::<u32>(), 70);
    let set = a.reading_set("print-basics", UiLang::En).await.unwrap();
    assert_eq!(set.items[0].choices, ["10", "15", "4"]);
    let print = |output: &str| ReadingAnswer {
        language: Language::Python,
        output: Some(output.into()),
        time: None,
        space: None,
    };
    let wrong = a
        .answer_reading("print-basics/1", print("15"), UiLang::Es)
        .await
        .unwrap();
    assert!(!wrong.correct);
    assert_eq!(wrong.output.as_deref(), Some("10"));
    assert!(wrong.explanation.contains("rango"));
    let right = a
        .answer_reading("print-basics/1", print("10\n"), UiLang::En)
        .await
        .unwrap();
    assert!(right.correct && right.attempts == 2);
    let complexity = a
        .answer_reading(
            "complexity-loops/2",
            ReadingAnswer {
                language: Language::TypeScript,
                output: None,
                time: Some("O(log n)".into()),
                space: Some("O(1)".into()),
            },
            UiLang::En,
        )
        .await
        .unwrap();
    assert!(complexity.correct);
    assert_eq!(complexity.dominant_line, Some(3));
    // Only Python and TypeScript versions exist; unknown items are not found.
    let mut rust = print("10");
    rust.language = Language::Rust;
    assert!(
        a.answer_reading("print-basics/1", rust, UiLang::En)
            .await
            .is_err()
    );
    assert!(
        a.answer_reading("print-basics/99", print("10"), UiLang::En)
            .await
            .is_err()
    );
    // Topic links come from topic.toml.
    let links = a.reading_links("big-o-growth", UiLang::En).await.unwrap();
    assert_eq!(
        links.first().map(|l| l.id.as_str()),
        Some("complexity-loops/1")
    );

    // Export, then import into a fresh app: the answers come back.
    let file = crate::testutil::temp_dir("reading-export").join("x.kbx.json");
    a.export_to(&file).await.unwrap();
    let b = app("reading-import").await;
    let preview = b.import_preview_from(&file).await.unwrap();
    assert_eq!(preview.counts.reading_answers, 2);
    b.import_apply(preview.id).await.unwrap();
    let rows = b.reading_answers().await.unwrap();
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .any(|r| r.item_id == "print-basics/1" && r.attempts == 2 && r.correct_at.is_some())
    );
}

/// Backups (expansion M10, review A4): the README on the first backup, the newest `keep`
/// backups kept, and files that are not ours never touched.
#[tokio::test]
async fn backups_prune_only_their_own_files() {
    let a = app("backup").await;
    let folder = crate::testutil::temp_dir("backup-folder");
    std::fs::write(folder.join("holiday.jpg"), b"not ours").unwrap();
    std::fs::write(folder.join("katabox-backup-notes.txt"), b"not ours either").unwrap();
    assert!(a.backup_now().await.is_err(), "no folder chosen yet");
    a.set_backup_folder(Some(&folder)).await.unwrap();
    a.set_backup_options(7, 2, true).await.unwrap();
    let day = 86_400_000;
    let t0 = 1_795_334_709_000;
    for k in 0..3 {
        a.set_clock(Some(t0 + k * day));
        a.backup_now().await.unwrap();
    }
    let mut names: Vec<String> = std::fs::read_dir(&folder)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "README-katabox-backups.txt",
            "holiday.jpg",
            "katabox-backup-20261123-080509.kbx.json",
            "katabox-backup-20261124-080509.kbx.json",
            "katabox-backup-notes.txt",
        ]
    );
    // A backup imports like an export.
    let b = app("backup-restore").await;
    b.import_preview_from(&folder.join("katabox-backup-20261124-080509.kbx.json"))
        .await
        .unwrap();
    // Due rules: not on start within the week; on exit when that option is on.
    let s = a.backup_status().await.unwrap();
    assert_eq!(s.last, Some(t0 + 2 * day));
    assert_eq!(s.next_due, Some(t0 + 9 * day));
    a.set_clock(Some(t0 + 3 * day));
    assert!(!a.backup_if_due(false).await);
    assert!(a.backup_if_due(true).await);
    // A missing folder is reported, not fatal.
    std::fs::remove_dir_all(&folder).unwrap();
    a.set_clock(Some(t0 + 30 * day));
    assert!(!a.backup_if_due(false).await);
    assert!(a.backup_status().await.unwrap().error.is_some());
}

/// Focus sessions (expansion M10): the running one is a setting; Stop keeps a summary.
#[tokio::test]
async fn focus_sessions_survive_and_summarise() {
    let a = app("focus").await;
    let t0 = 1_795_334_709_000;
    a.set_clock(Some(t0));
    assert!(a.focus_start(4, 5).await.is_err());
    a.focus_start(25, 5).await.unwrap();
    assert_eq!(a.focus_current().await.unwrap().unwrap().started_at, t0);
    a.set_clock(Some(t0 + 40 * 60_000));
    let row = a.focus_stop().await.unwrap().unwrap();
    // One work period done, the second under way.
    assert_eq!(
        (row.cycles, row.ended_at - row.started_at),
        (1, 40 * 60_000)
    );
    assert_eq!(a.focus_current().await.unwrap(), None);
    assert_eq!(a.focus_stop().await.unwrap(), None);
    assert_eq!(a.focus_sessions().await.unwrap(), vec![row]);
}

/// Packs (expansion M11, ADR 0038, review A3): preview, install, the merged content after a
/// restart, practice leaves pack problems out by default, the update rule, uninstall
/// keeps history.
#[tokio::test]
async fn packs_install_update_and_uninstall() {
    use crate::packs::PackAction;
    let p = crate::testutil::paths("packs");
    let work = crate::testutil::temp_dir("packs-src");
    let make = |version: &str, file: &str| {
        let m = work.join(format!("{file}.toml"));
        std::fs::write(
            &m,
            format!("id = \"drills\"\nname = \"Drills\"\nauthor = \"Ana\"\nversion = \"{version}\"\nlicence = \"CC-BY-SA-4.0\"\nmin_app = \"1.0.0-beta.1\"\n"),
        )
        .unwrap();
        let out = work.join(format!("{file}.kbpack"));
        kb_pack::kbpack::write_pack(
            &[crate::testutil::repo().join("content/problems/budget-pair")],
            &m,
            &out,
        )
        .unwrap();
        out
    };
    let v1 = make("1.0.0", "v1");
    let a = App::open(&p).await.unwrap();
    let preview = a.import_pack_preview(&v1).await.unwrap();
    assert_eq!(
        (preview.action, preview.problems, preview.installed.clone()),
        (PackAction::Install, 1, None)
    );
    assert_eq!(preview.sha256.len(), 64);
    a.import_pack_apply(preview.token).await.unwrap();
    a.close().await;

    // After a restart the pack's problem is in the library, marked, and judgeable data.
    let a = App::open(&p).await.unwrap();
    let all = a.list_problems(&ProblemFilter::default()).await.unwrap();
    let pack_p = all
        .iter()
        .find(|x| x.id == "drills--budget-pair")
        .expect("pack problem listed");
    assert_eq!(pack_p.pack.as_deref(), Some("drills"));
    assert!(
        a.get_problem("drills--budget-pair", UiLang::En)
            .await
            .unwrap()
            .pack
            .is_some()
    );
    // A3: not suggested unless asked for.
    let pool = a.practice_problems(UiLang::En).await.unwrap();
    assert!(pool.iter().all(|x| x.pack.is_none()));
    a.set_setting(crate::practice::INCLUDE_PACKS, &json!(true))
        .await
        .unwrap();
    assert!(
        a.practice_problems(UiLang::En)
            .await
            .unwrap()
            .iter()
            .any(|x| x.pack.is_some())
    );
    // History on a pack problem.
    a.store
        .user
        .record_submission(
            "drills--budget-pair",
            Language::Python,
            "x",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();

    // The same file: nothing to do; an older or equal version: refused; newer: update.
    let same = a.import_pack_preview(&v1).await.unwrap();
    assert_eq!(same.action, PackAction::Same);
    assert_eq!(
        a.import_pack_apply(same.token).await.unwrap_err().code,
        AppErrorCode::PackRefused
    );
    let v0 = make("0.9.0", "v0");
    let older = a.import_pack_preview(&v0).await.unwrap();
    assert_eq!(older.action, PackAction::NotNewer);
    assert_eq!(
        a.import_pack_apply(older.token).await.unwrap_err().code,
        AppErrorCode::PackRefused
    );
    let v2 = make("1.1.0", "v2");
    let newer = a.import_pack_preview(&v2).await.unwrap();
    assert_eq!(
        (newer.action, newer.installed.as_deref()),
        (PackAction::Update, Some("1.0.0"))
    );
    a.import_pack_apply(newer.token).await.unwrap();
    assert_eq!(a.packs().await.unwrap()[0].version, "1.1.0");

    // Uninstall: only its folder goes; after a restart the problem is gone, history stays.
    a.uninstall_pack("drills").await.unwrap();
    assert!(!p.local_data_dir.join("packs/drills").exists());
    a.close().await;
    let a = App::open(&p).await.unwrap();
    assert!(
        a.list_problems(&ProblemFilter::default())
            .await
            .unwrap()
            .iter()
            .all(|x| x.pack.is_none())
    );
    assert!(
        !a.store
            .user
            .list_submissions("drills--budget-pair")
            .await
            .unwrap()
            .is_empty()
    );
    assert!(a.packs().await.unwrap().is_empty());
    // A refused pack leaves nothing behind.
    std::fs::write(work.join("bad.kbpack"), b"not a zip").unwrap();
    let err = a
        .import_pack_preview(&work.join("bad.kbpack"))
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::PackRefused);
    let leftovers: Vec<_> = std::fs::read_dir(p.local_data_dir.join("packs"))
        .unwrap()
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with(".staging"))
        .collect();
    assert!(leftovers.is_empty(), "{leftovers:?}");
}

/// Classroom mode (expansion M12, ADR 0039): an assignment pack, its locks while it runs
/// (kb-app, not only the UI), the results file and its chain, the teacher's import and CSV.
#[tokio::test]
async fn classroom_assignment_locks_results_and_teacher_import() {
    use crate::classroom::AssignmentState;
    let p = crate::testutil::paths("classroom");
    let work = crate::testutil::temp_dir("classroom-src");
    let m = work.join("pack.toml");
    std::fs::write(
        &m,
        "id = \"week\"\nname = \"Week 3\"\nauthor = \"Ms. Ana\"\nversion = \"1.0.0\"\nlicence = \"CC-BY-SA-4.0\"\nmin_app = \"1.0.0-beta.1\"\n",
    )
    .unwrap();
    let at = work.join("assignment.toml");
    std::fs::write(
        &at,
        "title = \"Week 3\"\ndue = \"2026-12-01\"\nproblems = [\"budget-pair\", \"two-sum\"]\nlanguages = [\"python\"]\ntime_limit_min = 60\nhints = false\n",
    )
    .unwrap();
    let file = work.join("week.kbpack");
    kb_pack::kbpack::write_pack_with(
        &[crate::testutil::repo().join("content/problems/budget-pair")],
        &m,
        Some(&at),
        &file,
    )
    .unwrap();
    let a = App::open(&p).await.unwrap();
    let preview = a.import_pack_preview(&file).await.unwrap();
    assert_eq!(preview.assignment.as_deref(), Some("Week 3"));
    a.import_pack_apply(preview.token).await.unwrap();
    a.close().await;
    let a = App::open(&p).await.unwrap();

    let list = a.assignments(UiLang::En).await.unwrap();
    assert_eq!(list.len(), 1);
    let v = &list[0];
    assert_eq!(v.state, AssignmentState::NotStarted);
    assert_eq!(
        v.problems.iter().map(|x| x.id.as_str()).collect::<Vec<_>>(),
        ["week--budget-pair", "two-sum"]
    );
    assert!(v.problems.iter().all(|x| x.summary.is_some()));
    // Not started: nothing is locked yet, and there are no results.
    a.unlocked_hints("two-sum").await.unwrap();
    assert!(a.assignment_results("week", "Ana", false).await.is_err());

    // Running: help is locked (hints = false) and only Python is judged, in kb-app.
    a.store
        .user
        .record_submission(
            "two-sum",
            Language::Python,
            "before",
            &report_with(Verdict::Accepted),
        )
        .await
        .unwrap();
    // Distinct milliseconds for "before", the start and the submissions after it.
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let v = a.start_assignment("week", UiLang::En).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    assert_eq!(v.state, AssignmentState::Running);
    assert!(v.ends_at.is_some());
    let locked = |e: AppError| assert_eq!(e.code, AppErrorCode::LockedInAssignment, "{e}");
    locked(a.unlocked_hints("two-sum").await.unwrap_err());
    locked(a.unlock_hint("two-sum", 1).await.unwrap_err());
    locked(a.editorial("two-sum", UiLang::En).await.unwrap_err());
    locked(a.list_submissions("week--budget-pair").await.unwrap_err());
    locked(
        a.accepted_solution("two-sum", Language::Python)
            .await
            .unwrap_err(),
    );
    assert!(
        a.get_problem("two-sum", UiLang::En)
            .await
            .unwrap()
            .hints
            .is_empty()
    );
    // Other problems stay open.
    a.unlocked_hints("budget-pair").await.unwrap();
    let wrong_lang = a
        .run("two-sum", Language::Rust, "fn main() {}", vec![], |_| {})
        .await
        .unwrap_err();
    assert_eq!(wrong_lang.code, AppErrorCode::InvalidInput, "{wrong_lang}");
    assert!(wrong_lang.message.contains("assignment"), "{wrong_lang}");
    // Only one assignment at a time; starting again is a no-op.
    assert_eq!(
        a.start_assignment("week", UiLang::En)
            .await
            .unwrap()
            .started_at,
        v.started_at
    );

    // Submissions while it runs count; the one before does not.
    for (code, verdict) in [("wa", Verdict::WrongAnswer), ("ok", Verdict::Accepted)] {
        a.store
            .user
            .record_submission("two-sum", Language::Python, code, &report_with(verdict))
            .await
            .unwrap();
    }
    let results = a.assignment_results("week", " Ana ", true).await.unwrap();
    assert_eq!(results.student, "Ana");
    assert_eq!(results.entries.len(), 2);
    let two = &results.entries[1];
    assert_eq!(
        (two.submits, two.best_verdict, two.code.as_deref()),
        (2, Some(Verdict::Accepted), Some("ok"))
    );
    assert_eq!(results.entries[0].submits, 0);
    assert!(crate::classroom::chain_ok(&results));
    assert!(
        a.assignment_results("week", "", false)
            .await
            .unwrap_err()
            .message
            .contains("name")
    );
    let no_code = a.assignment_results("week", "Ana", false).await.unwrap();
    assert!(no_code.entries.iter().all(|e| e.code.is_none()));

    // The time limit ends it: the locks lift.
    let started = v.started_at.unwrap();
    a.set_clock(Some(started + 61 * 60_000));
    let v = a.assignment("week", UiLang::En).await.unwrap();
    assert_eq!(v.state, AssignmentState::Finished);
    assert_eq!(v.submitted_at, Some(started + 60 * 60_000));
    a.unlocked_hints("two-sum").await.unwrap();
    a.set_clock(None);

    // Teacher side: save, import (twice: a duplicate), an edited copy, a hostile file.
    let out = work.join("ana.results.json");
    a.save_assignment_results("week", "Ana", true, &out)
        .await
        .unwrap();
    let mut edited: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
    edited["student"] = json!("Bo");
    edited["entries"][0]["submits"] = json!(1);
    edited["entries"][0]["best_verdict"] = json!("accepted");
    edited["entries"][0]["language"] = json!("python");
    let bo = work.join("bo.results.json");
    std::fs::write(&bo, serde_json::to_vec(&edited).unwrap()).unwrap();
    let evil = work.join("evil.results.json");
    std::fs::write(&evil, b"{\"format\":\"katabox-export/v2\"}").unwrap();
    let imported = a
        .import_class_results(&[out.clone(), out.clone(), bo, evil])
        .await
        .unwrap();
    assert_eq!(
        (imported.added, imported.duplicates, imported.edited),
        (2, 1, 1)
    );
    assert_eq!(imported.refused.len(), 1);
    let class = a.class_results().await.unwrap();
    assert_eq!(class.len(), 1);
    assert_eq!(class[0].title, "Week 3");
    let flags: Vec<(String, bool)> = class[0]
        .students
        .iter()
        .map(|s| (s.student.clone(), s.chain_ok))
        .collect();
    assert_eq!(
        flags,
        [("Ana".to_string(), true), ("Bo".to_string(), false)]
    );
    let csv = work.join("week.csv");
    a.save_class_csv("week", &csv).await.unwrap();
    let text = std::fs::read_to_string(&csv).unwrap();
    assert!(
        text.contains("Ana,yes,two-sum,python,accepted,2,"),
        "{text}"
    );
    assert!(
        text.contains("Bo,no,week--budget-pair,python,accepted,1,"),
        "{text}"
    );
}
