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
    assert_eq!(info.user_schema_version, 1);
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
        .submit("two-sum", Language::Python, "x", |_| {})
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
        .submit("nope", Language::Python, "x", |_| {})
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
        .submit("two-sum", Language::Python, "x", |_| {})
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
