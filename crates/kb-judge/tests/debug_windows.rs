//! The debugger's replay runs (ADR 0022) with real Python inside the Windows sandbox.
//! Needs the dev sandbox root with the pinned toolchains:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --test debug_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::Instant;

use kb_core::Language;
use kb_core::debug::{OutcomeStatus, Trace, Truncation};
use kb_judge::debug::{self, DebugLimit, DebugRun, MAX_STEPS, StopRequest};
use kb_judge::{JudgeEnv, JudgeError, Problem};
use kb_runner::ROOT_MARKER;
use serde_json::{Map, Value, json};

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

fn two_sum() -> Problem {
    Problem::load(&repo().join("content/problems/two-sum")).unwrap()
}

fn reference() -> String {
    std::fs::read_to_string(repo().join("content/problems/two-sum/solutions/ref.py")).unwrap()
}

fn input(v: Value) -> Map<String, Value> {
    v.as_object().unwrap().clone()
}

fn first_public() -> Map<String, Value> {
    input(json!({"nums": [5, 8, 12, 1, 3], "target": 15}))
}

fn trace(p: &Problem, source: &str, inp: &Map<String, Value>) -> Trace {
    match debug::trace(p, Language::Python, source, inp, &env()).unwrap() {
        DebugRun::Trace(t) => t,
        other => panic!("expected a trace, got {other:?}"),
    }
}

fn stop(
    p: &Problem,
    source: &str,
    inp: &Map<String, Value>,
    t: &Trace,
    req: StopRequest,
) -> kb_core::debug::StopInfo {
    match debug::stop(p, Language::Python, source, inp, t, &req, &env()).unwrap() {
        DebugRun::Stop(s) => s,
        other => panic!("expected a stop, got {other:?}"),
    }
}

fn local<'a>(s: &'a kb_core::debug::StopInfo, name: &str) -> &'a str {
    &s.locals
        .iter()
        .find(|v| v.name == name)
        .unwrap_or_else(|| panic!("no local {name}: {:?}", s.locals))
        .value
}

#[test]
fn two_sum_breakpoint_variables_step_and_result() {
    let p = two_sum();
    let src = reference();
    let inp = first_public();
    let t = trace(&p, &src, &inp);
    assert!(t.truncated.is_none());
    let o = t.outcome.as_ref().unwrap();
    assert_eq!(o.status, OutcomeStatus::Ok);
    assert_eq!(o.value, json!([2, 4]));
    // Every step is in the solution function (depth 1); line 7 is `seen[n] = i`.
    assert!(t.steps.iter().all(|s| s.depth == 1));
    let hits: Vec<usize> = (0..t.steps.len())
        .filter(|&i| t.steps[i].line == 7)
        .collect();
    // 5, 8, 12 and 1 are stored before 3 finds its partner 12.
    assert_eq!(hits.len(), 4, "{:?}", t.steps);

    // Paused at the breakpoint the first time: seen is still empty, n is 5.
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: hits[0] as u32,
            evals: vec!["seen".into(), "n * 2".into(), "undefined_name".into()],
            ..Default::default()
        },
    );
    assert_eq!(
        s.stack.len(),
        1,
        "harness frames must be hidden: {:?}",
        s.stack
    );
    assert_eq!(s.stack[0].function, "two_sum");
    assert_eq!(s.stack[0].line, 7);
    assert_eq!(local(&s, "seen"), "{}");
    assert_eq!(local(&s, "n"), "5");
    assert_eq!(s.evals[1].value.as_ref().unwrap().value, "10");
    assert!(!s.evals[2].ok);
    assert!(s.evals[2].error.as_deref().unwrap().contains("NameError"));

    // The next step (step over) is back at the loop header; the second hit has seen = {5: 0}.
    assert_eq!(t.steps[hits[0] + 1].line, 3);
    let s2 = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: hits[1] as u32,
            expand: vec![vec![json!("locals"), json!("seen")]],
            ..Default::default()
        },
    );
    assert_eq!(local(&s2, "seen"), "{5: 0}");
    assert_eq!(local(&s2, "n"), "8");
    let seen = &s2.expanded[0];
    assert_eq!(seen.children.len(), 1);
    assert_eq!(seen.children[0].name, "5");
    assert_eq!(seen.children[0].value, "0");
}

#[test]
fn prints_are_tagged_with_their_steps() {
    let p = two_sum();
    let src = "def two_sum(nums, target):\n    for i in range(len(nums)):\n        print('at', i)\n    return [0, 1]\n";
    let t = trace(&p, src, &first_public());
    let lines: Vec<u32> = t.steps.iter().map(|s| s.line).collect();
    assert_eq!(t.prints.len(), 5, "{:?}", t.prints);
    for (k, chunk) in t.prints.iter().enumerate() {
        assert_eq!(chunk.text, format!("at {k}\n"));
        assert_eq!(
            lines[chunk.step as usize], 3,
            "a print belongs to its print() line"
        );
    }
}

#[test]
fn a_long_loop_reaches_the_step_cap_with_a_usable_trace() {
    let p = two_sum();
    let src = "def two_sum(nums, target):\n    i = 0\n    while True:\n        i += 1\n";
    let start = Instant::now();
    let t = trace(&p, src, &first_public());
    eprintln!("trace of {} steps: {:?}", t.steps.len(), start.elapsed());
    assert_eq!(t.truncated, Some(Truncation::Steps));
    assert_eq!(t.steps.len(), MAX_STEPS as usize);
    assert!(t.outcome.is_none());

    // The replay cost near the end of a long trace (recorded in ADR 0022).
    for step in [1_000u32, 150_000, MAX_STEPS - 1] {
        let start = Instant::now();
        let s = stop(
            &p,
            src,
            &first_public(),
            &t,
            StopRequest {
                step,
                ..Default::default()
            },
        );
        eprintln!("stop at step {step}: {:?}", start.elapsed());
        assert!(local(&s, "i").parse::<u32>().unwrap() > 0);
    }
}

#[test]
fn deep_recursion_is_traced_with_depths() {
    let p = two_sum();
    let src = "def down(k):\n    if k == 0:\n        return 0\n    return 1 + down(k - 1)\n\ndef two_sum(nums, target):\n    down(300)\n    return [0, 1]\n";
    let t = trace(&p, src, &first_public());
    let max_depth = t.steps.iter().map(|s| s.depth).max().unwrap();
    assert_eq!(max_depth, 302, "two_sum + 301 calls of down");
    let deepest = t.steps.iter().position(|s| s.depth == max_depth).unwrap();
    let s = stop(
        &p,
        src,
        &first_public(),
        &t,
        StopRequest {
            step: deepest as u32,
            ..Default::default()
        },
    );
    assert_eq!(s.stack.len(), 302);
    assert_eq!(s.stack[0].function, "down");
    assert_eq!(s.stack.last().unwrap().function, "two_sum");
    assert_eq!(local(&s, "k"), "0");
}

#[test]
fn heavy_steps_hit_the_cpu_cap_and_sleeps_the_wall_limit() {
    let mut p = two_sum();
    // A small base limit keeps the test short: (30 × 10 + 1000) ms × 5 = 6.5 s of CPU.
    p.time_limit_ms = 30;
    let busy = "def two_sum(nums, target):\n    while True:\n        sum(range(10**7))\n";
    let r = debug::trace(&p, Language::Python, busy, &first_public(), &env()).unwrap();
    assert_eq!(r, DebugRun::Limit(DebugLimit::Time));
    let sleepy =
        "import time\ndef two_sum(nums, target):\n    while True:\n        time.sleep(1)\n";
    let start = Instant::now();
    let r = debug::trace(&p, Language::Python, sleepy, &first_public(), &env()).unwrap();
    assert_eq!(r, DebugRun::Limit(DebugLimit::Time));
    eprintln!("sleep loop stopped after {:?}", start.elapsed());
}

#[test]
fn a_non_deterministic_program_is_reported_as_diverged() {
    let p = two_sum();
    let src = "import os\ndef two_sum(nums, target):\n    x = 0\n    for _ in range(40):\n        if os.urandom(1)[0] % 2:\n            x += 1\n        else:\n            x -= 1\n    return [0, 1]\n";
    let inp = first_public();
    let t = trace(&p, src, &inp);
    let last = (t.steps.len() - 1) as u32;
    let r = debug::stop(
        &p,
        Language::Python,
        src,
        &inp,
        &t,
        &StopRequest {
            step: last,
            ..Default::default()
        },
        &env(),
    )
    .unwrap();
    assert_eq!(r, DebugRun::Diverged);
}

#[test]
fn sets_of_strings_replay_the_same_way() {
    // String hashing is fixed for debug runs, so set order and the path through it repeat.
    let p = two_sum();
    let src = "def two_sum(nums, target):\n    words = {'kiwi', 'fig', 'plum', 'pear', 'lime', 'date'}\n    out = []\n    for w in words:\n        if len(out) % 2:\n            out.append(w)\n        else:\n            out.insert(0, w)\n    return [0, 1]\n";
    let inp = first_public();
    let t = trace(&p, src, &inp);
    let last = (t.steps.len() - 1) as u32;
    let s = stop(
        &p,
        src,
        &inp,
        &t,
        StopRequest {
            step: last,
            evals: vec!["out".into()],
            ..Default::default()
        },
    );
    assert!(s.evals[0].ok);
}

#[test]
fn evaluations_never_change_later_steps() {
    let p = two_sum();
    let src = reference();
    let inp = first_public();
    let t = trace(&p, &src, &inp);
    let hits: Vec<usize> = (0..t.steps.len())
        .filter(|&i| t.steps[i].line == 7)
        .collect();
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: hits[0] as u32,
            evals: vec!["nums.append(99)".into(), "print('side effect')".into()],
            ..Default::default()
        },
    );
    assert!(s.evals[0].ok);
    assert_eq!(s.evals[1].stdout, "side effect\n");
    let later = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: hits[1] as u32,
            ..Default::default()
        },
    );
    assert_eq!(local(&later, "nums"), "[5, 8, 12, 1, 3]");
}

#[test]
fn exceptions_load_errors_and_other_languages() {
    let p = two_sum();
    let src = "def two_sum(nums, target):\n    x = nums[0]\n    return nums[99]\n";
    let t = trace(&p, src, &first_public());
    let o = t.outcome.unwrap();
    assert_eq!(o.status, OutcomeStatus::Error);
    assert_eq!(o.line, Some(3));
    let err = o.error.unwrap();
    assert!(
        err.contains("IndexError") && !err.contains("main.py"),
        "{err}"
    );
    assert!(!err.contains(":\\"), "absolute paths are stripped: {err}");

    let broken = "def two_sum(nums, target)\n    return []\n";
    let r = debug::trace(&p, Language::Python, broken, &first_public(), &env()).unwrap();
    assert!(matches!(r, DebugRun::LoadError(m) if m.contains("SyntaxError")));

    assert!(!debug::supported(Language::Rust));
    let r = debug::trace(&p, Language::Rust, "", &first_public(), &env());
    assert!(matches!(r, Err(JudgeError::Unsupported(Language::Rust))));
}
