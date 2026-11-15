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

/// Submit options for tests: a fixed local date, not a review.
fn submit_opts() -> kb_app::SubmitOptions {
    kb_app::SubmitOptions {
        today: "2026-11-14".into(),
        review: false,
        lang: kb_store::UiLang::En,
    }
}

/// Test helper, off in normal runs: with `KB_TEST_BURN_THREADS=<n>` it keeps n threads
/// busy until dropped, to reproduce timing behaviour on a loaded machine.
struct CpuBurner(
    Vec<std::thread::JoinHandle<()>>,
    std::sync::Arc<std::sync::atomic::AtomicBool>,
);

impl CpuBurner {
    fn from_env() -> Option<CpuBurner> {
        let n: usize = std::env::var("KB_TEST_BURN_THREADS").ok()?.parse().ok()?;
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let threads = (0..n)
            .map(|_| {
                let stop = stop.clone();
                std::thread::spawn(move || {
                    let mut x = 1u64;
                    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                        x = std::hint::black_box(
                            x.wrapping_mul(6364136223846793005).wrapping_add(1),
                        );
                    }
                })
            })
            .collect();
        Some(CpuBurner(threads, stop))
    }
}

impl Drop for CpuBurner {
    fn drop(&mut self) {
        self.1.store(true, std::sync::atomic::Ordering::Relaxed);
        for t in self.0.drain(..) {
            let _ = t.join();
        }
    }
}

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

/// Public plus hidden tests of two-sum, from the package.
fn two_sum_tests() -> u32 {
    kb_judge::Problem::load(&repo().join("content/problems/two-sum"))
        .unwrap()
        .tests
        .len() as u32
}

fn content_db() -> PathBuf {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("kb-app-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let out = dir.join("content.db");
        let o = out.clone();
        std::thread::spawn(move || {
            // Profile inputs are not committed (ADR 0024): generate the one the profile
            // test uses, so the pack carries it.
            let env = kb_pack::env_for(&dev_root()).unwrap();
            kb_pack::profile::fill_caches(&repo().join("content/problems/balanced-streaks"), &env)
                .unwrap();
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
            .submit("two-sum", lang, &reference(lang), &submit_opts(), on)
            .await
            .unwrap();
        assert_eq!(
            r.report.verdict,
            Verdict::Accepted,
            "{lang}: {:?}",
            r.report
        );
        let n = two_sum_tests();
        assert_eq!(r.report.tests.len(), n as usize);

        let events = events.lock().unwrap().clone();
        let mut i = 0;
        if lang == Language::Rust {
            assert_eq!(events[0], JudgeEvent::Compiling);
            i = 1;
        }
        assert_eq!(events[i], JudgeEvent::Running { total: n });
        let checked: Vec<u32> = events
            .iter()
            .filter_map(|e| match e {
                JudgeEvent::Checked { done, total, .. } if *total == n => Some(*done),
                _ => None,
            })
            .collect();
        assert_eq!(checked, (1..=n).collect::<Vec<_>>(), "{lang}");
        assert_eq!(
            events.last(),
            Some(&JudgeEvent::Finished {
                verdict: Verdict::Accepted
            })
        );
    }
    assert_eq!(app.list_submissions("two-sum").await.unwrap().len(), 4);
    let status = app.list_problems(&Default::default()).await.unwrap();
    let two_sum = status.iter().find(|p| p.id == "two-sum").unwrap();
    assert_eq!(two_sum.status, Some(ProblemStatus::Solved));
}

#[tokio::test]
async fn a_wrong_answer_shows_the_failing_input_and_hides_the_rest() {
    let app = app("wrong").await;
    // A constant answer: wrong on the first public test already.
    let code = "def two_sum(nums: list[int], target: int) -> list[int]:\n    return [0, 1]\n";
    let r = app
        .submit("two-sum", Language::Python, code, &submit_opts(), |_| {})
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

/// The judge's dev layout and the installer's receipts name the same programs.
#[tokio::test]
async fn installed_runtimes_are_where_the_judge_looks() {
    let app = app("layout").await;
    let info = app.app_info().await.unwrap();
    let dev = kb_judge::JudgeEnv::dev(dev_root());
    for t in &info.toolchains {
        let want = dev.toolchain_program(t.language).unwrap();
        assert_eq!(
            t.program.as_deref(),
            Some(want.display().to_string().as_str()),
            "{}",
            t.language
        );
    }
    let langs = app.languages();
    assert!(langs.platform_supported);
    assert!(
        langs
            .languages
            .iter()
            .all(|l| l.installed && l.download_bytes == 0),
        "{langs:?}"
    );
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap().flatten() {
        let t = to.join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            copy_dir(&e.path(), &t);
        } else {
            std::fs::copy(e.path(), &t).unwrap();
        }
    }
}

/// kb-runner grants the sandbox access to a toolchain folder lazily, once. The installer
/// replaces a component as a whole folder renamed in from staging (ADR 0015): after a
/// replacement the sandbox must still read it.
#[tokio::test]
async fn a_runtime_replaced_by_rename_still_runs_in_the_sandbox() {
    let root = std::env::temp_dir().join(format!("kb-app-replace-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("receipts")).unwrap();
    std::fs::write(root.join(kb_runner::ROOT_MARKER), b"").unwrap();
    copy_dir(
        &dev_root().join("toolchains/python-3.14"),
        &root.join("toolchains/python-3.14"),
    );
    std::fs::copy(
        dev_root().join("receipts/python.json"),
        root.join("receipts/python.json"),
    )
    .unwrap();
    let dir = std::env::temp_dir().join(format!("kb-app-replace-data-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let app = App::open(&AppPaths {
        content_db: content_db(),
        user_db: dir.join("user.db"),
        local_data_dir: dir.join("local"),
        dev_root: Some(root.clone()),
        dev_data: true,
    })
    .await
    .unwrap();
    let accepted =
        |r: kb_app::SubmitResult| assert_eq!(r.report.verdict, Verdict::Accepted, "{:?}", r.report);
    let code = reference(Language::Python);
    accepted(
        app.submit("two-sum", Language::Python, &code, &submit_opts(), |_| {})
            .await
            .unwrap(),
    );

    // What promotion does: a new copy staged elsewhere in the root, the old folder gone,
    // the new one renamed into place.
    let staged = root.join("staging/new");
    copy_dir(&root.join("toolchains/python-3.14"), &staged);
    std::fs::remove_dir_all(root.join("toolchains/python-3.14")).unwrap();
    std::fs::rename(&staged, root.join("toolchains/python-3.14")).unwrap();
    accepted(
        app.submit("two-sum", Language::Python, &code, &submit_opts(), |_| {})
            .await
            .unwrap(),
    );
    app.close().await;
}

/// With Python installed, custom inputs go through the problem's validate.py (in the
/// sandbox) before the reference runs: the broken constraint is the message.
#[tokio::test]
async fn custom_inputs_breaking_a_constraint_get_the_validators_message() {
    let app = app("validator").await;
    let ok = json!({"nums": [4, 6], "target": 10})
        .as_object()
        .unwrap()
        .clone();
    let short = json!({"nums": [5], "target": 5})
        .as_object()
        .unwrap()
        .clone();
    for lang in [Language::Python, Language::Rust] {
        let err = app
            .run(
                "two-sum",
                lang,
                &reference(lang),
                vec![ok.clone(), short.clone()],
                |_| {},
            )
            .await
            .unwrap_err();
        assert_eq!(err.code, AppErrorCode::InvalidCustomInput, "{err}");
        assert_eq!(err.message, "custom input 2: 2 ≤ nums.length ≤ 10⁵");
    }
    // Valid custom inputs still run.
    let r = app
        .run(
            "two-sum",
            Language::Python,
            &reference(Language::Python),
            vec![ok],
            |_| {},
        )
        .await
        .unwrap();
    assert!(r.verdict.is_accepted(), "{r:?}");
}

/// The first Run measures this machine once (ADR 0017); the limits follow it.
#[tokio::test]
async fn the_first_run_calibrates_this_machine() {
    let app = app("calibrate").await;
    assert_eq!(app.calibration(), None);
    let ok = app
        .run(
            "two-sum",
            Language::Rust,
            &reference(Language::Rust),
            vec![],
            |_| {},
        )
        .await
        .unwrap();
    assert!(ok.verdict.is_accepted(), "{ok:?}");
    let c = app.calibration().expect("measured on the first run");
    assert_eq!(c.runtime, Language::Python);
    assert!(c.factor > 0.0 && (0.5..=3.0).contains(&c.applied), "{c:?}");
    let stored = app
        .get_setting(kb_app::CALIBRATION_SETTING)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored["runtime"], json!("python"));

    // Recalibrate measures again and the problem page shows limits at the new scale.
    let again = app.recalibrate().await.unwrap();
    assert!(again.measured_at >= c.measured_at);
    let view = app
        .get_problem("two-sum", kb_store::UiLang::En)
        .await
        .unwrap();
    assert_eq!(view.time_scale, again.applied);
    let rust = view
        .time_limits
        .iter()
        .find(|l| l.language == Language::Rust)
        .unwrap();
    assert_eq!(
        rust.time_limit_ms,
        (300.0 * again.applied).ceil() as u32,
        "{view:?}"
    );
}

/// A real Accepted submit (ADR 0019): the first solve schedules a review in 3 days and
/// suggests a next problem; a review-mode submit three days later moves it to 7 days.
#[tokio::test]
async fn a_first_accepted_submit_schedules_a_review() {
    let app = app("practice").await;
    let r = app
        .submit(
            "two-sum",
            Language::Python,
            &reference(Language::Python),
            &submit_opts(),
            |_| {},
        )
        .await
        .unwrap();
    assert!(r.report.verdict.is_accepted(), "{:?}", r.report);
    assert!(r.practice.first_solve);
    let next = r.practice.next.expect("a next problem");
    assert_ne!(next.id, "two-sum");
    let state = app
        .review_state("two-sum", "2026-11-14")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.due_on, "2026-11-17");

    let review = kb_app::SubmitOptions {
        today: "2026-11-17".into(),
        review: true,
        lang: kb_store::UiLang::En,
    };
    let r = app
        .submit(
            "two-sum",
            Language::Rust,
            &reference(Language::Rust),
            &review,
            |_| {},
        )
        .await
        .unwrap();
    assert_eq!(
        r.practice.review,
        Some(kb_app::practice::ReviewOutcome::Scheduled {
            due_on: "2026-11-24".into(),
            interval_days: 7
        })
    );
    assert!(!r.practice.first_solve);
}

/// A debug session through the App (ADR 0022): breakpoint, step, watch, console, result.
#[tokio::test]
async fn a_debug_session_on_two_sum() {
    use kb_app::debug::{DebugAction, DebugInspect, DebugStart, DebugState};
    let app = app("debug").await;
    let input = json!({"nums": [5, 8, 12, 1, 3], "target": 15});
    let start = DebugStart {
        problem: "two-sum".into(),
        language: Language::Python,
        code: reference(Language::Python),
        input: input.as_object().unwrap().clone(),
        breakpoints: vec![7],
        watches: vec!["target - n".into()],
        visualize: None,
    };
    let v = app.debug_start(start.clone()).await.unwrap();
    let paused = |v: &kb_app::debug::DebugView| match &v.state {
        DebugState::Paused { line, stop } => (*line, stop.clone()),
        other => panic!("not paused: {other:?}"),
    };
    let (line, stop) = paused(&v);
    assert_eq!(line, 7);
    assert_eq!(stop.evals[0].value.as_ref().unwrap().value, "10");
    let id = v.session;

    // Step over: back to the loop header; continue: the breakpoint again with seen = {5: 0}.
    let v = app.debug_action(id, DebugAction::StepOver).await.unwrap();
    assert_eq!(paused(&v).0, 3);
    let v = app.debug_action(id, DebugAction::Continue).await.unwrap();
    let (line, stop) = paused(&v);
    assert_eq!(line, 7);
    assert!(
        stop.locals
            .iter()
            .any(|l| l.name == "seen" && l.value == "{5: 0}")
    );
    // Step back is free with replay.
    let v = app.debug_action(id, DebugAction::StepBack).await.unwrap();
    assert_eq!(paused(&v).0, 5);

    // Expand `seen` and add a watch; then the console.
    let v = app
        .debug_inspect(
            id,
            DebugInspect {
                frame: 0,
                expand: vec![vec![json!("locals"), json!("seen")]],
                watches: vec!["len(seen)".into()],
                visualize: Some(vec!["seen".into()]),
            },
        )
        .await
        .unwrap();
    let (_, stop) = paused(&v);
    assert_eq!(stop.expanded[0].children[0].value, "0");
    assert_eq!(stop.evals[0].value.as_ref().unwrap().value, "1");
    // The visualizer's snapshot of `seen` (ADR 0027): a dict with one entry.
    let snap = stop.snapshot.as_ref().expect("a snapshot");
    assert_eq!(snap.roots[0].name, "seen");
    assert!(
        matches!(
            &snap.objects[0],
            kb_core::debug::SnapObject::Map { len: 1, .. }
        ),
        "{snap:?}"
    );
    // Closing the view (no `visualize`) stops it.
    let v = app
        .debug_inspect(
            id,
            DebugInspect {
                frame: 0,
                expand: vec![vec![json!("locals"), json!("seen")]],
                watches: vec!["len(seen)".into()],
                visualize: None,
            },
        )
        .await
        .unwrap();
    assert!(paused(&v).1.snapshot.is_none());
    let e = app
        .debug_evaluate(id, 0, "sorted(seen)".into())
        .await
        .unwrap();
    assert_eq!(e.value.unwrap().value, "[5]");

    // Without breakpoints, continue runs to the end and shows the result.
    app.debug_set_breakpoints(id, vec![]).unwrap();
    let v = app.debug_action(id, DebugAction::Continue).await.unwrap();
    let DebugState::Finished { outcome, truncated } = &v.state else {
        panic!("{v:?}")
    };
    assert!(truncated.is_none());
    assert_eq!(outcome.as_ref().unwrap().value, json!([2, 4]));

    // Restart goes back to the first breakpoint (none now: straight to the end).
    app.debug_end(id);
    let err = app
        .debug_action(id, DebugAction::Restart)
        .await
        .unwrap_err();
    assert_eq!(err.code, AppErrorCode::NotFound);

    // Other languages and bad inputs are refused.
    let mut rust = start.clone();
    rust.language = Language::Rust;
    assert_eq!(
        app.debug_start(rust).await.unwrap_err().code,
        AppErrorCode::DebugUnsupported
    );
    let mut bad = start;
    bad.input = json!({"nums": [1]}).as_object().unwrap().clone();
    assert_eq!(
        app.debug_start(bad).await.unwrap_err().code,
        AppErrorCode::InvalidInput
    );
}

/// The same session commands for TypeScript (ADR 0023): two-sum's reference, with a
/// breakpoint on `seen.set(...)`, every step action and the result.
#[tokio::test]
async fn a_typescript_debug_session_on_two_sum() {
    use kb_app::debug::{DebugAction, DebugInspect, DebugStart, DebugState};
    let app = app("debug-ts").await;
    let start = DebugStart {
        problem: "two-sum".into(),
        language: Language::TypeScript,
        code: reference(Language::TypeScript),
        input: json!({"nums": [5, 8, 12, 1, 3], "target": 15})
            .as_object()
            .unwrap()
            .clone(),
        breakpoints: vec![6],
        watches: vec!["target - nums[i]".into()],
        visualize: None,
    };
    assert!(kb_app::debug::languages().contains(&Language::TypeScript));
    let v = app.debug_start(start).await.unwrap();
    let paused = |v: &kb_app::debug::DebugView| match &v.state {
        DebugState::Paused { line, stop } => (*line, stop.clone()),
        other => panic!("not paused: {other:?}"),
    };
    let (line, stop) = paused(&v);
    assert_eq!(line, 6);
    assert_eq!(stop.evals[0].value.as_ref().unwrap().value, "10");
    let id = v.session;

    // Over: back to the loop header; into: the next line; back: the header again.
    let v = app.debug_action(id, DebugAction::StepOver).await.unwrap();
    assert_eq!(paused(&v).0, 3);
    let v = app.debug_action(id, DebugAction::StepInto).await.unwrap();
    assert_eq!(paused(&v).0, 4);
    let v = app.debug_action(id, DebugAction::StepBack).await.unwrap();
    assert_eq!(paused(&v).0, 3);
    // Continue: the breakpoint again, with 5 → 0 in seen.
    let v = app.debug_action(id, DebugAction::Continue).await.unwrap();
    let (line, stop) = paused(&v);
    assert_eq!(line, 6);
    assert!(
        stop.locals
            .iter()
            .any(|l| l.name == "seen" && l.value == "Map(1) { 5 => 0 }"),
        "{:?}",
        stop.locals
    );
    let v = app
        .debug_inspect(
            id,
            DebugInspect {
                frame: 0,
                expand: vec![vec![json!("locals"), json!("nums")]],
                watches: vec!["seen.size".into()],
                visualize: None,
            },
        )
        .await
        .unwrap();
    let (_, stop) = paused(&v);
    assert_eq!(stop.expanded[0].children.len(), 5);
    assert_eq!(stop.evals[0].value.as_ref().unwrap().value, "1");
    let e = app
        .debug_evaluate(id, 0, "[...seen.keys()]".into())
        .await
        .unwrap();
    assert_eq!(e.value.unwrap().value, "[ 5 ]");

    // Out of twoSum: the program ends with the result.
    let v = app.debug_action(id, DebugAction::StepOut).await.unwrap();
    let DebugState::Finished { outcome, .. } = &v.state else {
        panic!("{v:?}")
    };
    assert_eq!(outcome.as_ref().unwrap().value, json!([2, 4]));
    app.debug_end(id);
}

/// The complexity profiler (ADR 0024) through the app: the Python reference of
/// balanced-streaks is estimated O(n) with the reference line at every size; the
/// nested-loop solution is slower than linear and stops early. Problems without a
/// profile say so.
#[tokio::test]
async fn profiles_a_solution_and_estimates_the_class() {
    use kb_core::profile::ComplexityClass;
    let _load = CpuBurner::from_env();
    let app = app("profile").await;
    // A fresh app measures the machine once first (ADR 0017); that is not the profile.
    app.recalibrate().await.unwrap();
    let dir = repo().join("content/problems/balanced-streaks/solutions");
    let read = |f: &str| std::fs::read_to_string(dir.join(f)).unwrap();
    // The rest of the suite runs sandboxes alongside, and under that load the 12 s
    // budget can stop a profile after a size or two (as designed). Try again then.
    let mut attempt = 0;
    let fast = loop {
        attempt += 1;
        let started = std::time::Instant::now();
        let view = app
            .profile_solution("balanced-streaks", Language::Python, &read("ref.py"))
            .await
            .unwrap();
        // The profile's own time: waiting for other sandbox users (this file's tests
        // run alongside) is not part of its budget.
        let own = started
            .elapsed()
            .saturating_sub(std::time::Duration::from_millis(u64::from(view.queued_ms)));
        assert!(own < std::time::Duration::from_secs(15), "{own:?}");
        if view.points.len() >= 4 || attempt == 3 {
            break view;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    };
    let sizes: Vec<u32> = fast.reference.iter().map(|r| r.n).collect();
    assert_eq!(sizes, [300, 1000, 3000, 10000, 30000, 100000]);
    // This file's other tests share the machine, so the 12 s budget may stop the largest
    // size (that is the designed behaviour); the strict hit rate is the calibration
    // test's job (kb-judge complexity_windows).
    assert!(fast.points.len() >= 4, "{fast:?}");
    assert!(
        matches!(
            fast.stopped,
            None | Some(kb_core::profile::ProfileStop::Budget { .. })
        ),
        "{fast:?}"
    );
    let estimate = fast.estimate.as_ref().unwrap();
    assert!(
        estimate.class == ComplexityClass::Linear
            || estimate.close_alternative == Some(ComplexityClass::Linear),
        "{fast:?}"
    );
    assert!(fast.reference.iter().all(|r| r.us > 0));

    let slow = app
        .profile_solution("balanced-streaks", Language::Python, &read("naive.py"))
        .await
        .unwrap();
    assert!(slow.stopped.is_some(), "{slow:?}");
    let class = slow.estimate.as_ref().map(|e| e.class);
    assert!(
        matches!(
            class,
            Some(ComplexityClass::Quadratic | ComplexityClass::Exponential)
        ),
        "{slow:?}"
    );

    let none = app
        .profile_solution("corner-shop", Language::Python, "")
        .await;
    assert_eq!(
        none.err().map(|e| e.code),
        Some(AppErrorCode::ProfileUnavailable)
    );
}

/// The failing-test shrinker (ADR 0030): a two-sum solution that fails whenever a number
/// is negative shrinks from eight numbers to a valid input of at most three that still has
/// a negative number, within the run budget.
#[tokio::test]
async fn shrinks_a_failing_input_to_a_small_one_that_still_fails() {
    let app = app("shrink").await;
    let wrong = "def two_sum(nums, target):\n    if any(x < 0 for x in nums):\n        return [0, 0]\n    seen = {}\n    for i, n in enumerate(nums):\n        if target - n in seen:\n            return [seen[target - n], i]\n        seen[n] = i\n    return []\n";
    let input = json!({"nums": [10, 20, -3, 40, 50, 60, 1, 2], "target": 3});
    let r = app
        .shrink(
            "two-sum",
            Language::Python,
            wrong,
            input.as_object().unwrap().clone(),
            |_| {},
        )
        .await
        .unwrap();
    assert_ne!(r.stop, kb_app::ShrinkStop::NotFailing, "{r:?}");
    let nums: Vec<i64> = r.input["nums"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_i64().unwrap())
        .collect();
    assert!(nums.len() <= 3 && nums.iter().any(|x| *x < 0), "{r:?}");
    assert!(r.size < r.original_size, "{r:?}");
    assert!(r.runs <= kb_app::shrink::MAX_RUNS, "{r:?}");
    assert_eq!(r.verdict, Verdict::WrongAnswer, "{r:?}");
    assert_eq!(r.actual, Some(json!([0, 0])));

    // A correct solution does not fail: nothing to shrink.
    let right =
        std::fs::read_to_string(repo().join("content/problems/two-sum/solutions/ref.py")).unwrap();
    let ok = app
        .shrink(
            "two-sum",
            Language::Python,
            &right,
            input.as_object().unwrap().clone(),
            |_| {},
        )
        .await
        .unwrap();
    assert_eq!(ok.stop, kb_app::ShrinkStop::NotFailing);
}

/// SQL problems (ADR 0031) through the app: SQL only, the starter, the tables for the
/// schema panel, Run and Submit with the reference and a wrong query, and the features an
/// SQL problem does not have.
#[tokio::test]
async fn an_sql_problem_end_to_end() {
    let app = app("sql").await;
    let view = app
        .get_problem("late-returns", kb_store::UiLang::En)
        .await
        .unwrap();
    assert_eq!(view.mode, kb_core::ProblemMode::Sql);
    let tables = view.sql_tables.as_ref().expect("the schema panel's tables");
    assert_eq!(tables[0]["name"], "members");
    assert_eq!(
        tables[1]["columns"][1]["references"],
        json!(["members", "id"])
    );

    // SQL only, and SQL only for SQL problems.
    let stub = app.get_stub("late-returns", Language::Sql).await.unwrap();
    assert!(
        stub.contains("SELECT") && stub.contains("FROM members"),
        "{stub}"
    );
    let e = app
        .get_stub("late-returns", Language::Python)
        .await
        .unwrap_err();
    assert_eq!(e.code, AppErrorCode::UnsupportedLanguage);
    let e = app.get_stub("two-sum", Language::Sql).await.unwrap_err();
    assert_eq!(e.code, AppErrorCode::UnsupportedLanguage);
    let e = app
        .run("late-returns", Language::Python, "x", vec![], |_| {})
        .await
        .unwrap_err();
    assert_eq!(e.code, AppErrorCode::UnsupportedLanguage);

    // The reference: Accepted on Run (public tests) and Submit (all of them).
    let query =
        std::fs::read_to_string(repo().join("content/problems/late-returns/solutions/ref.sql"))
            .unwrap();
    let (events, sink) = recorder();
    let run = app
        .run("late-returns", Language::Sql, &query, vec![], sink)
        .await
        .unwrap();
    assert_eq!(run.verdict, Verdict::Accepted, "{:?}", run.first_failure);
    assert!(
        events
            .lock()
            .unwrap()
            .iter()
            .any(|e| matches!(e, JudgeEvent::Finished { .. }))
    );
    let ok = app
        .submit(
            "late-returns",
            Language::Sql,
            &query,
            &submit_opts(),
            |_| {},
        )
        .await
        .unwrap();
    assert_eq!(ok.report.verdict, Verdict::Accepted);
    assert_eq!(
        app.solved_languages("late-returns").await.unwrap(),
        [Language::Sql]
    );

    // wrong.sql: Wrong Answer with both row sets for the diff.
    let wrong =
        std::fs::read_to_string(repo().join("content/problems/late-returns/solutions/wrong.sql"))
            .unwrap();
    let r = app
        .submit(
            "late-returns",
            Language::Sql,
            &wrong,
            &submit_opts(),
            |_| {},
        )
        .await
        .unwrap();
    assert_eq!(r.report.verdict, Verdict::WrongAnswer);
    let f = r.report.first_failure.as_ref().unwrap();
    assert!(f.input.contains_key("tables"));
    assert!(f.expected["rows"].is_array() && f.actual.as_ref().unwrap()["rows"].is_array());

    // A write is refused before it runs (the harness's authorizer).
    let r = app
        .run(
            "late-returns",
            Language::Sql,
            "DELETE FROM loans",
            vec![],
            |_| {},
        )
        .await
        .unwrap();
    assert_eq!(r.verdict, Verdict::CompilationError);

    // Off for SQL, each with its code.
    let custom = json!({"tables": {"members": [], "loans": []}})
        .as_object()
        .unwrap()
        .clone();
    let e = app
        .run(
            "late-returns",
            Language::Sql,
            &query,
            vec![custom.clone()],
            |_| {},
        )
        .await
        .unwrap_err();
    assert_eq!(e.code, AppErrorCode::InvalidCustomInput);
    let e = app
        .debug_start(kb_app::debug::DebugStart {
            problem: "late-returns".into(),
            language: Language::Sql,
            code: query.clone(),
            input: custom,
            breakpoints: vec![],
            watches: vec![],
            visualize: None,
        })
        .await
        .unwrap_err();
    assert_eq!(e.code, AppErrorCode::DebugUnsupported);
    let e = app
        .profile_solution("late-returns", Language::Sql, &query)
        .await
        .unwrap_err();
    assert_eq!(e.code, AppErrorCode::ProfileUnavailable);
}
