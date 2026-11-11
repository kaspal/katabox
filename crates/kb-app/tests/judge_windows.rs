//! Run and Submit on two-sum through the app service, in every language, inside the
//! Windows sandbox. Needs the dev sandbox root with the pinned toolchains:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-app --test judge_windows
//! ```
//!
//! `KATABOX_SANDBOX_ROOT` must not be set (it would override the dev root).
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use kb_app::{App, AppErrorCode, AppPaths, RootSource};
use kb_core::{JudgeEvent, Language, TestKind, Verdict};
use kb_store::{ProblemStatus, SnapshotSource};
use serde_json::json;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn dev_root() -> PathBuf {
    let root = std::path::absolute(repo().join("target/katabox")).unwrap();
    assert!(
        root.join(kb_runner::ROOT_MARKER).is_file()
            && root.join("toolchains/rust-1.99/bin/rustc.exe").is_file(),
        "dev sandbox root or toolchains missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    root
}

fn content_db() -> PathBuf {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("kb-app-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let out = dir.join("content.db");
        let o = out.clone();
        std::thread::spawn(move || {
            kb_pack::build_pack(&repo().join("content/problems"), &o, "test", "1").unwrap()
        })
        .join()
        .unwrap();
        out
    })
    .clone()
}

async fn app(name: &str) -> App {
    let dir = std::env::temp_dir().join(format!("kb-app-e2e-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let app = App::open(&AppPaths {
        content_db: content_db(),
        user_db: dir.join("user.db"),
        local_data_dir: dir.join("local"),
        dev_root: Some(dev_root()),
        dev_data: true,
    })
    .await
    .unwrap();
    let info = app.app_info().await.unwrap();
    assert_eq!(info.sandbox_root_source, RootSource::Dev);
    assert!(
        info.toolchains.iter().all(|t| t.present),
        "{:?}",
        info.toolchains
    );
    app
}

fn reference(lang: Language) -> String {
    std::fs::read_to_string(
        repo()
            .join("content/problems/two-sum/solutions")
            .join(format!("ref.{}", lang.extension())),
    )
    .unwrap()
}

/// Collects the progress events of one call.
fn recorder() -> (
    Arc<Mutex<Vec<JudgeEvent>>>,
    impl FnMut(JudgeEvent) + Send + 'static,
) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    (events, move |e| sink.lock().unwrap().push(e))
}

#[tokio::test]
async fn every_reference_is_accepted_with_progress() {
    let app = app("accept").await;
    for lang in kb_judge::languages::SUPPORTED {
        let (events, on) = recorder();
        let r = app
            .submit("two-sum", lang, &reference(lang), on)
            .await
            .unwrap();
        assert_eq!(
            r.report.verdict,
            Verdict::Accepted,
            "{lang}: {:?}",
            r.report
        );
        assert_eq!(r.report.tests.len(), 7);

        let events = events.lock().unwrap().clone();
        let mut i = 0;
        if lang == Language::Rust {
            assert_eq!(events[0], JudgeEvent::Compiling);
            i = 1;
        }
        assert_eq!(events[i], JudgeEvent::Running { total: 7 });
        let checked: Vec<u32> = events
            .iter()
            .filter_map(|e| match e {
                JudgeEvent::Checked { done, total: 7, .. } => Some(*done),
                _ => None,
            })
            .collect();
        assert_eq!(checked, (1..=7).collect::<Vec<_>>(), "{lang}");
        assert_eq!(
            events.last(),
            Some(&JudgeEvent::Finished {
                verdict: Verdict::Accepted
            })
        );
    }
    assert_eq!(app.list_submissions("two-sum").await.unwrap().len(), 4);
    let status = app.list_problems(&Default::default()).await.unwrap();
    assert_eq!(status[0].status, Some(ProblemStatus::Solved));
}

#[tokio::test]
async fn a_wrong_answer_shows_the_failing_input_and_hides_the_rest() {
    let app = app("wrong").await;
    // Right on the public tests (pairs at the start), wrong when the pair comes later.
    let code = "def two_sum(nums: list[int], target: int) -> list[int]:\n    return [0, 1]\n";
    let r = app
        .submit("two-sum", Language::Python, code, |_| {})
        .await
        .unwrap();
    assert_eq!(r.report.verdict, Verdict::WrongAnswer);
    let f = r.report.first_failure.as_ref().expect("first failure");
    assert!(f.input.contains_key("nums") && f.actual.is_some());
    for t in &r.report.tests {
        if t.result.kind == TestKind::Hidden {
            assert_eq!(t.actual, None);
        }
    }
    let view = app.get_submission(r.submission_id).await.unwrap();
    assert_eq!(view.report.verdict, Verdict::WrongAnswer);
    assert_eq!(view.code, code);
    let snaps = app.list_snapshots("two-sum").await.unwrap();
    assert_eq!(snaps[0].source, SnapshotSource::Submit);
    assert_eq!(snaps[0].verdict, Some(Verdict::WrongAnswer));
}

#[tokio::test]
async fn run_with_custom_inputs() {
    let app = app("custom").await;
    let custom = vec![
        json!({"nums": [5, 7, 1, 4], "target": 11})
            .as_object()
            .unwrap()
            .clone(),
        json!({"nums": [-3, 9, 3], "target": 0})
            .as_object()
            .unwrap()
            .clone(),
    ];
    let code = reference(Language::Python);
    let r = app
        .run("two-sum", Language::Python, &code, custom.clone(), |_| {})
        .await
        .unwrap();
    assert_eq!(r.verdict, Verdict::Accepted, "{r:?}");
    let kinds: Vec<TestKind> = r.tests.iter().map(|t| t.result.kind).collect();
    assert_eq!(
        kinds,
        [
            TestKind::Public,
            TestKind::Public,
            TestKind::Public,
            TestKind::Custom,
            TestKind::Custom
        ]
    );
    assert_eq!(r.tests[3].actual, Some(json!([1, 3])));

    // A solution that is right on the public tests but wrong on custom input 1 is judged
    // against the reference's answer.
    let wrong = code.replace(
        "    seen: dict[int, int] = {}\n",
        "    if target == 11:\n        return [0, 0]\n    seen: dict[int, int] = {}\n",
    );
    assert_ne!(wrong, code);
    let r = app
        .run("two-sum", Language::Python, &wrong, custom, |_| {})
        .await
        .unwrap();
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    let f = r.first_failure.unwrap();
    assert_eq!((f.kind, f.index), (TestKind::Custom, 1));
    assert_eq!(f.expected, json!([1, 3]));

    // An input the reference cannot handle.
    let bad = vec![
        json!({"nums": "oops", "target": 1})
            .as_object()
            .unwrap()
            .clone(),
    ];
    let err = app
        .run("two-sum", Language::Python, &code, bad, |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::InvalidCustomInput, "{err}");

    // Runs are snapshotted (deduplicated) with their verdict and never change the status.
    let snaps = app.list_snapshots("two-sum").await.unwrap();
    assert_eq!(snaps.len(), 2);
    assert_eq!(snaps[0].verdict, Some(Verdict::WrongAnswer));
    assert_eq!(snaps[1].verdict, Some(Verdict::Accepted));
    assert!(app.list_submissions("two-sum").await.unwrap().is_empty());
}
