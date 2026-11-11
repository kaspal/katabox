//! The sandbox checks of `kb-pack validate --run` (ADR 0016) on copies of two-sum, each
//! altered to break one rule. Needs the dev sandbox root:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-pack --test checks_windows
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::Language;
use kb_pack::{Check, Finding, check_problem, env_for};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> kb_judge::JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    assert!(
        root.join("toolchains/python-3.14/python.exe").is_file(),
        "dev sandbox root missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    env_for(&root).unwrap()
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap().flatten() {
        let t = to.join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            if e.file_name() != "__pycache__" {
                copy_dir(&e.path(), &t);
            }
        } else {
            std::fs::copy(e.path(), &t).unwrap();
        }
    }
}

/// A fresh copy of two-sum under a temp folder named `two-sum` (the id must match).
fn two_sum(case: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("kbp-checks-{case}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let dir = base.join("two-sum");
    copy_dir(&repo().join("content/problems/two-sum"), &dir);
    (base, dir)
}

fn finding(findings: &[Finding], check: Check) -> &Finding {
    findings
        .iter()
        .find(|f| f.check == check)
        .unwrap_or_else(|| panic!("no {check:?} finding in {findings:#?}"))
}

#[test]
fn a_complete_problem_passes_every_check() {
    let (_base, dir) = two_sum("ok");
    let f = check_problem(&dir, &env());
    assert!(f.iter().all(|x| x.ok), "{f:#?}");
    for check in [
        Check::Validator,
        Check::Reference(Language::Python),
        Check::Reference(Language::TypeScript),
        Check::Reference(Language::Php),
        Check::Reference(Language::Rust),
        Check::Stress,
        Check::Naive,
    ] {
        finding(&f, check);
    }
    assert!(finding(&f, Check::Stress).detail.starts_with("500 inputs"));
}

#[test]
fn a_reference_that_disagrees_with_brute_is_reported_with_the_input() {
    let (_base, dir) = two_sum("disagree");
    // Right on every committed test, wrong whenever -19 appears (random inputs have it).
    std::fs::write(
        dir.join("solutions/ref.py"),
        "def two_sum(nums: list[int], target: int) -> list[int]:\n    if -19 in nums:\n        return [0, 0]\n    seen = {}\n    for i, n in enumerate(nums):\n        if target - n in seen:\n            return [seen[target - n], i]\n        seen[n] = i\n    return []\n",
    )
    .unwrap();
    let f = check_problem(&dir, &env());
    assert!(finding(&f, Check::Reference(Language::Python)).ok, "{f:#?}");
    let s = finding(&f, Check::Stress);
    assert!(!s.ok);
    assert!(
        s.detail.contains("python disagrees with brute.py on seed"),
        "{}",
        s.detail
    );
    assert!(
        s.detail.contains("input:") && s.detail.contains("-19"),
        "{}",
        s.detail
    );
}

#[test]
fn naive_passing_means_the_tests_are_too_weak() {
    let (_base, dir) = two_sum("weak");
    // Without the max-size tests, the O(n²) solution is fast enough.
    for t in ["15", "16"] {
        std::fs::remove_file(dir.join(format!("tests/hidden/{t}.json"))).unwrap();
    }
    let f = check_problem(&dir, &env());
    let n = finding(&f, Check::Naive);
    assert!(!n.ok && n.detail.contains("do not force"), "{}", n.detail);
}

#[test]
fn a_wrong_naive_is_rejected() {
    let (_base, dir) = two_sum("wrong-naive");
    std::fs::write(
        dir.join("solutions/naive.py"),
        "def two_sum(nums: list[int], target: int) -> list[int]:\n    return [0, 1]\n",
    )
    .unwrap();
    let n = finding(&check_problem(&dir, &env()), Check::Naive).clone();
    assert!(
        !n.ok && n.detail.contains("must be correct"),
        "{}",
        n.detail
    );
}

#[test]
fn the_validator_rejecting_a_committed_test_fails() {
    let (_base, dir) = two_sum("invalid");
    std::fs::write(
        dir.join("tests/hidden/99.json"),
        r#"{"input": {"nums": [5], "target": 5}, "output": []}"#,
    )
    .unwrap();
    let v = finding(&check_problem(&dir, &env()), Check::Validator).clone();
    assert!(!v.ok, "{v:?}");
    assert!(
        v.detail.contains("hidden test") && v.detail.contains("2 ≤ nums.length ≤ 10⁴"),
        "{}",
        v.detail
    );
}

#[test]
fn a_reference_over_half_the_time_limit_fails() {
    let (_base, dir) = two_sum("slow");
    // About 0.7 s per max-size test: Accepted (limit 2 s) but over the 1 s budget.
    std::fs::write(
        dir.join("solutions/ref.py"),
        "import time\n\ndef two_sum(nums: list[int], target: int) -> list[int]:\n    if len(nums) == 10000:\n        t = time.perf_counter()\n        while time.perf_counter() - t < 0.7:\n            pass\n    seen = {}\n    for i, n in enumerate(nums):\n        if target - n in seen:\n            return [seen[target - n], i]\n        seen[n] = i\n    return []\n",
    )
    .unwrap();
    let r = finding(
        &check_problem(&dir, &env()),
        Check::Reference(Language::Python),
    )
    .clone();
    assert!(!r.ok && r.detail.starts_with("too slow"), "{}", r.detail);
}

/// The scripts run in the sandbox, like user code: a gen.py that reads a file outside it
/// fails with the sandbox's permission error instead of reading it.
#[test]
fn problem_scripts_run_in_the_sandbox() {
    let (_base, dir) = two_sum("escape");
    let secret = repo().join("Cargo.toml");
    std::fs::write(
        dir.join("gen.py"),
        format!(
            "def generate(seed, size):\n    open({:?}).read()\n    return {{'nums': [1, 2], 'target': 3}}\n",
            secret.display().to_string()
        ),
    )
    .unwrap();
    let s = finding(&check_problem(&dir, &env()), Check::Stress).clone();
    assert!(
        !s.ok && s.detail.contains("PermissionError"),
        "{}",
        s.detail
    );
}
