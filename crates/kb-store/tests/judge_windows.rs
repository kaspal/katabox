//! Records a real judge Report: the two-sum Python reference (Accepted) and a wrong
//! answer, judged in the Windows sandbox. Needs the dev sandbox root:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-store --test judge_windows
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::{Language, Verdict};
use kb_judge::{JudgeEnv, Problem, judge};
use kb_runner::ROOT_MARKER;
use kb_store::{ProblemStatus, SnapshotSource, UserDb};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file()
            && root.join("toolchains/python-3.14/python.exe").is_file(),
        "sandbox root or Python missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

#[tokio::test]
async fn records_a_real_two_sum_report() {
    let problem = Problem::load(&repo().join("content/problems/two-sum")).unwrap();
    let env = env();
    let good =
        std::fs::read_to_string(repo().join("content/problems/two-sum/solutions/ref.py")).unwrap();
    let bad = "def two_sum(nums: list[int], target: int) -> list[int]:\n    return [0, 0]\n";
    let ac = judge(&problem, &problem.tests, Language::Python, &good, &env).unwrap();
    let wa = judge(&problem, &problem.tests, Language::Python, bad, &env).unwrap();
    assert_eq!(ac.verdict, Verdict::Accepted);
    assert_eq!(wa.verdict, Verdict::WrongAnswer);

    let dir = std::env::temp_dir().join(format!("kb-store-judge-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let db = UserDb::open(&dir.join("user.db")).await.unwrap();
    let wa_id = db
        .record_submission("two-sum", Language::Python, bad, &wa)
        .await
        .unwrap();
    let ac_id = db
        .record_submission("two-sum", Language::Python, &good, &ac)
        .await
        .unwrap();

    let stored = db.get_submission(ac_id).await.unwrap();
    assert_eq!(stored.summary.verdict, Verdict::Accepted);
    assert_eq!(stored.summary.time_us, ac.time_us);
    assert_eq!(stored.summary.memory_kb, ac.memory_kb);
    assert_eq!(stored.report, serde_json::to_value(&ac).unwrap());
    assert_eq!(stored.report["total_tests"], ac.total_tests);
    assert!(ac.total_tests >= 18, "two-sum has 3 public and 15+ hidden tests");

    let failed = db.get_submission(wa_id).await.unwrap();
    assert_eq!(failed.report["verdict"], "wrong_answer");
    assert!(failed.report["first_failure"].is_object());

    let status = db.problem_status("two-sum").await.unwrap().unwrap();
    assert_eq!(status.status, ProblemStatus::Solved);
    assert!(status.first_solved_at.is_some());
    let snaps = db.list_snapshots("two-sum").await.unwrap();
    assert_eq!(snaps.len(), 2);
    assert!(snaps.iter().all(|s| s.source == SnapshotSource::Submit));
    db.close().await;
    let _ = std::fs::remove_dir_all(&dir);
}
