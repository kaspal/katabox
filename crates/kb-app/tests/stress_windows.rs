//! The stress lab (expansion M8, ADR 0036) end to end in the Windows sandbox: a seeded
//! wrong solution against a correct brute force disagrees within the budget, in Python and
//! TypeScript and on a program problem; a broken brute force is reported as such.
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-app --test stress_windows -- --nocapture
//! ```
#![cfg(windows)]
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use kb_app::stress::{StressSource, StressStop};
use kb_app::{App, AppPaths};
use kb_core::{Language, Verdict};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn dev_root() -> PathBuf {
    let root = std::path::absolute(repo().join("target/katabox")).unwrap();
    assert!(
        root.join(kb_runner::ROOT_MARKER).is_file(),
        "dev sandbox root missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    root
}

fn content_db() -> PathBuf {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("kb-app-stress-{}", std::process::id()));
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
    let dir = std::env::temp_dir().join(format!("kb-app-stress-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    App::open(&AppPaths {
        content_db: content_db(),
        user_db: dir.join("user.db"),
        local_data_dir: dir.join("local"),
        dev_root: Some(dev_root()),
        dev_data: true,
    })
    .await
    .unwrap()
}

/// Wrong whenever a number is negative.
const WRONG_PY: &str = "def two_sum(nums: list[int], target: int) -> list[int]:\n    if any(x < 0 for x in nums):\n        return [0, 0]\n    seen: dict[int, int] = {}\n    for i, n in enumerate(nums):\n        if target - n in seen:\n            return [seen[target - n], i]\n        seen[n] = i\n    return []\n";
const BRUTE_PY: &str = "def two_sum(nums: list[int], target: int) -> list[int]:\n    for i in range(len(nums)):\n        for j in range(i + 1, len(nums)):\n            if nums[i] + nums[j] == target:\n                return [i, j]\n    return []\n";
const WRONG_TS: &str = "export function twoSum(nums: number[], target: number): number[] {\n  if (nums.some((x) => x < 0)) return [0, 0];\n  const seen = new Map<number, number>();\n  for (let i = 0; i < nums.length; i++) {\n    const j = seen.get(target - nums[i]);\n    if (j !== undefined) return [j, i];\n    seen.set(nums[i], i);\n  }\n  return [];\n}\n";
const BRUTE_TS: &str = "export function twoSum(nums: number[], target: number): number[] {\n  for (let i = 0; i < nums.length; i++)\n    for (let j = i + 1; j < nums.length; j++)\n      if (nums[i] + nums[j] === target) return [i, j];\n  return [];\n}\n";

#[tokio::test]
async fn finds_a_disagreement_in_python_and_typescript() {
    let a = app("two-sum").await;
    for (lang, wrong, brute) in [
        (Language::Python, WRONG_PY, BRUTE_PY),
        (Language::TypeScript, WRONG_TS, BRUTE_TS),
    ] {
        let started = std::time::Instant::now();
        let mut events = 0;
        let r = a
            .stress("two-sum", lang, wrong, brute, |_| events += 1)
            .await
            .unwrap();
        println!(
            "{lang:?}: {:?} after {} rounds, {} inputs, {:?}",
            r.stop,
            r.rounds,
            r.tested,
            started.elapsed()
        );
        assert_eq!(r.stop, StressStop::Found, "{r:?}");
        assert_eq!(r.source, StressSource::Generator);
        assert_eq!(r.verdict, Some(Verdict::WrongAnswer));
        assert_eq!(r.actual, Some(serde_json::json!([0, 0])));
        let nums = r.input.as_ref().unwrap()["nums"]
            .as_array()
            .unwrap()
            .clone();
        assert!(nums.iter().any(|v| v.as_i64().unwrap() < 0), "{r:?}");
        assert!(events as u32 + 1 >= r.rounds);
    }
}

#[tokio::test]
async fn a_program_problem_and_a_broken_brute_force() {
    let a = app("program").await;
    let reference =
        std::fs::read_to_string(repo().join("content/problems/lap-splits/solutions/ref.py"))
            .unwrap();
    // The fastest lap printed as the slowest one.
    let wrong = reference.replace("clock(min(laps))", "clock(max(laps))");
    assert_ne!(wrong, reference);
    let r = a
        .stress("lap-splits", Language::Python, &wrong, &reference, |_| {})
        .await
        .unwrap();
    assert_eq!(r.stop, StressStop::Found, "{r:?}");
    assert!(r.input.as_ref().unwrap().contains_key("stdin"));

    // A brute force that crashes is the brute force's failure, not a disagreement.
    let broken = "def two_sum(nums, target):\n    raise ValueError('not yet')\n";
    let r = a
        .stress("two-sum", Language::Python, BRUTE_PY, broken, |_| {})
        .await
        .unwrap();
    assert_eq!(r.stop, StressStop::BruteFailed, "{r:?}");
    assert_eq!(r.verdict, Some(Verdict::RuntimeError));

    // SQL has no stress lab.
    let sql = a
        .stress(
            "late-returns",
            Language::Sql,
            "SELECT 1",
            "SELECT 1",
            |_| {},
        )
        .await;
    assert!(sql.is_err());
}

/// "Check" (expansion M9): types first, then validate.py in the sandbox.
#[tokio::test]
async fn check_custom_reports_types_and_constraints() {
    let a = app("check").await;
    let input = |v: serde_json::Value| v.as_object().unwrap().clone();
    let r = a
        .check_custom(
            "two-sum",
            vec![
                input(serde_json::json!({"nums": [1, 2], "target": 3})),
                input(serde_json::json!({"nums": "x", "target": 3})),
                input(serde_json::json!({"nums": [1], "target": 3})),
            ],
        )
        .await
        .unwrap();
    assert!(r.constraints_checked);
    assert_eq!(r.problems[0], None);
    assert!(r.problems[1].is_some(), "{r:?}");
    assert!(
        r.problems[2]
            .as_deref()
            .unwrap_or("")
            .contains("nums.length"),
        "{r:?}"
    );
}
