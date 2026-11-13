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

/// Content batch 4: every new problem's Python reference can be debugged on its first
/// public test. The trace ends with the expected answer, a stop halfway and at the last
/// step shows the user's frames only (the entry function outermost), and the deepest
/// step shows the problem's helper: a class method, a recursive function or a
/// generator.
/// A frame name to find at the deepest step, and how many frames must match it.
type Helper = (&'static str, usize);

#[test]
fn new_problems_are_debuggable() {
    // (id, entry function, helper frame at the deepest step, frames of it there)
    let problems: [(&str, &str, Option<Helper>); 15] = [
        ("catalog-prefixes", "prefix_counts", Some(("Trie.", 1))),
        ("fun-run-results", "finish_order", None),
        ("family-generations", "generations", None),
        ("corner-shop", "shop", Some(("Shop.", 1))),
        ("outfit-picker", "outfits", Some(("<locals>.choose", 3))),
        ("short-names", "short_names", Some(("Trie.", 1))),
        (
            "postage-stamps",
            "exact_postage",
            Some(("<locals>.search", 3)),
        ),
        ("queue-swaps", "swaps_to_sort", Some(("<locals>.sort", 2))),
        ("sales-teams", "team_totals", None),
        ("library-holds", "holds", Some(("Library.", 1))),
        ("seed-rows", "plantings", None),
        ("ring-road", "ring_length", None),
        ("common-boss", "common_boss", Some(("<locals>.decide", 1))),
        (
            "delivery-round",
            "shortest_round",
            Some(("<locals>.best", 3)),
        ),
        ("toll-grid", "cheapest_crossing", Some(("neighbours", 1))),
    ];
    assert_debuggable(&problems);
}

/// Content batch 5: the same checks for the next 15 problems.
#[test]
fn batch_5_problems_are_debuggable() {
    let problems: [(&str, &str, Option<Helper>); 15] = [
        (
            "shelf-codes",
            "count_below",
            Some(("<locals>.first_at_least", 1)),
        ),
        ("lamp-hours", "lit_time", None),
        ("path-tiles", "tilings", None),
        ("fewest-hops", "fewest_hops", None),
        ("club-merges", "clubs_left", Some(("<locals>.find", 1))),
        ("balanced-streaks", "balanced_stretches", None),
        (
            "spice-market",
            "best_haul",
            Some(("<locals>.richer_first", 1)),
        ),
        ("first-match", "first_matches", Some(("Trie.", 1))),
        ("shelf-order", "natural_order", Some(("chunk_key", 1))),
        ("feature-flags", "users_with_all", None),
        ("page-wrap", "wrap_lines", None),
        ("undo-notebook", "notebook", Some(("Notebook.", 1))),
        (
            "shelf-packing",
            "fewest_shelves",
            Some(("<locals>.place", 1)),
        ),
        ("pocket-calculator", "evaluate", Some(("Parser.", 3))),
        ("thermostat-swings", "window_swings", None),
    ];
    assert_debuggable(&problems);
}

/// Each problem's ref.py on its first public test: the traced result passes the checker,
/// a stop halfway and at the last step shows only user frames (the entry function
/// outermost), and the deepest step shows the problem's helper frame.
fn assert_debuggable(problems: &[(&str, &str, Option<Helper>)]) {
    for &(id, entry, helper) in problems {
        let dir = repo().join("content/problems").join(id);
        let p = Problem::load(&dir).unwrap();
        let src = std::fs::read_to_string(dir.join("solutions/ref.py")).unwrap();
        let public = &p.tests[0];
        let started = Instant::now();
        let t = trace(&p, &src, &public.input);
        assert!(t.truncated.is_none(), "{id}: truncated");
        let o = t.outcome.clone().unwrap();
        assert_eq!(o.status, OutcomeStatus::Ok, "{id}: {:?}", o.error);
        assert!(
            matches!(
                p.checker.check(&public.expected, &o.value),
                kb_judge::CheckOutcome::Ok
            ),
            "{id}: {} is not {}",
            o.value,
            public.expected
        );
        let last = t.steps.len() - 1;
        for step in [last / 2, last] {
            let s = stop(
                &p,
                &src,
                &public.input,
                &t,
                StopRequest {
                    step: step as u32,
                    ..Default::default()
                },
            );
            assert!(!s.stack.is_empty(), "{id}@{step}: empty stack");
            assert_eq!(
                s.stack.last().unwrap().function,
                entry,
                "{id}@{step}: {:?}",
                s.stack
            );
            assert!(!s.locals.is_empty(), "{id}@{step}: no locals");
        }
        if let Some((frame, count)) = helper {
            let deepest = (0..t.steps.len())
                .max_by_key(|&i| (t.steps[i].depth, std::cmp::Reverse(i)))
                .unwrap();
            let s = stop(
                &p,
                &src,
                &public.input,
                &t,
                StopRequest {
                    step: deepest as u32,
                    ..Default::default()
                },
            );
            let names: Vec<&str> = s.stack.iter().map(|f| f.function.as_str()).collect();
            assert_eq!(
                s.stack.len() as u32,
                t.steps[deepest].depth,
                "{id}: {names:?}"
            );
            assert!(names[0].contains(frame), "{id}: {names:?}");
            assert!(
                names.iter().filter(|n| n.contains(frame)).count() >= count,
                "{id}: {names:?}"
            );
            assert_eq!(*names.last().unwrap(), entry, "{id}: {names:?}");
        }
        println!(
            "{id:<20} {:>5} steps, max depth {}, {:?} for the trace and its stops",
            t.steps.len(),
            t.steps.iter().map(|s| s.depth).max().unwrap(),
            started.elapsed()
        );
    }
}

/// The node-type problems (ADR 0025): debuggable like the others.
#[test]
fn node_problems_are_debuggable() {
    let problems: [(&str, &str, Option<Helper>); 4] = [
        ("playlist-rotation", "rotate_playlist", None),
        ("checkout-merge", "merge_lines", None),
        ("fullest-level", "fullest_level", None),
        ("hedge-trim", "trim_hedge", Some(("<locals>.trim", 2))),
    ];
    assert_debuggable(&problems);
}

/// Nodes read as a short line (never the whole chain, never an address), with their
/// fields as children in a fixed order, and expand lazily (ADR 0025).
#[test]
fn list_and_tree_nodes_are_shown_readably() {
    let load = |id: &str| {
        let dir = repo().join("content/problems").join(id);
        let p = Problem::load(&dir).unwrap();
        let src = std::fs::read_to_string(dir.join("solutions/ref.py")).unwrap();
        (p, src)
    };
    let (p, src) = load("playlist-rotation");
    let inp = p.tests[0].input.clone();
    let t = trace(&p, &src, &inp);
    assert_eq!(t.outcome.as_ref().unwrap().value, json!([16, 23, 4, 8, 15]));
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: 0,
            expand: vec![
                vec![json!("locals"), json!("playlist")],
                vec![json!("locals"), json!("playlist"), json!(1)],
            ],
            ..Default::default()
        },
    );
    let head = s.locals.iter().find(|v| v.name == "playlist").unwrap();
    assert_eq!(head.value, "ListNode [4 → 8 → 15 → 16 → 23]");
    assert_eq!((head.type_name.as_str(), head.children), ("ListNode", 2));
    let names: Vec<(&str, &str)> = s.expanded[0]
        .children
        .iter()
        .map(|c| (c.name.as_str(), c.value.as_str()))
        .collect();
    assert_eq!(
        names,
        [("val", "4"), ("next", "ListNode [8 → 15 → 16 → 23]")]
    );
    assert_eq!(s.expanded[1].children[0].value, "8");

    let (p, src) = load("hedge-trim");
    let inp = p.tests[0].input.clone();
    let t = trace(&p, &src, &inp);
    assert_eq!(t.outcome.as_ref().unwrap().value, json!([3, null, 2]));
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: 0,
            expand: vec![vec![json!("locals"), json!("root")]],
            ..Default::default()
        },
    );
    assert_eq!(local(&s, "root"), "TreeNode(3, left=-5, right=2)");
    let names: Vec<(&str, &str)> = s.expanded[0]
        .children
        .iter()
        .map(|c| (c.name.as_str(), c.value.as_str()))
        .collect();
    assert_eq!(
        names,
        [
            ("val", "3"),
            ("left", "TreeNode(-5, left=1, right=-4)"),
            ("right", "TreeNode(2, left=None, right=None)")
        ]
    );
}

/// Content to 150, milestone 1: fundamentals, range queries, geometry, road repairs.
#[test]
fn content_150_m1_problems_are_debuggable() {
    let problems: [(&str, &str, Option<Helper>); 15] = [
        ("temperature-swing", "temperature_swing", None),
        ("grade-bands", "grade_bands", None),
        ("vowel-tally", "vowel_tally", None),
        ("leap-years", "leap_years", None),
        ("halving-steps", "halving_steps", None),
        ("title-case", "title_case", None),
        ("mileage-log", "trip_totals", None),
        ("stock-ledger", "stock_ledger", Some(("<locals>.", 1))),
        (
            "gauge-corrections",
            "gauge_readings",
            Some(("<locals>.", 1)),
        ),
        ("cold-snap", "cold_snaps", Some(("<locals>.", 2))),
        ("road-repairs", "road_repairs", None),
        ("turn-signals", "turn_signals", None),
        ("fenced-garden", "where_is_point", None),
        ("crossing-paths", "paths_cross", Some(("<locals>.", 1))),
        ("tree-fence", "fence_area", Some(("<locals>.", 2))),
    ];
    assert_debuggable(&problems);
}

/// Content to 150, milestone 2: linked lists, trees, tries, heap, backtracking.
#[test]
fn content_150_m2_problems_are_debuggable() {
    let problems: [(&str, &str, Option<Helper>); 15] = [
        ("two-queues-merge", "merge_queues", None),
        ("split-the-train", "split_train", None),
        ("zigzag-boarding", "zigzag_boarding", None),
        ("batch-reversal", "reverse_batches", None),
        ("window-view", "window_view", None),
        (
            "bracket-notation",
            "bracket_notation",
            Some(("<locals>.write", 2)),
        ),
        (
            "catalogue-check",
            "is_search_tree",
            Some(("<locals>.ok", 2)),
        ),
        ("kth-smallest-code", "kth_smallest", None),
        ("garden-walk", "longest_walk", Some(("<locals>.depth", 2))),
        (
            "rebuild-tree",
            "rebuild_tree",
            Some(("TreeNode.__init__", 1)),
        ),
        ("richest-trail", "richest_trail", Some(("<locals>.gain", 2))),
        ("best-code-pair", "best_xor_pair", None),
        ("word-grid-hunt", "find_words", Some(("<locals>.dfs", 2))),
        ("nearest-stalls", "nearest_stalls", None),
        ("coin-bag-combos", "coin_combos", Some(("<locals>.pick", 2))),
    ];
    assert_debuggable(&problems);
}

/// Content to 150, milestone 3: graphs, dynamic programming, greedy, intervals.
#[test]
fn content_150_m3_problems_are_debuggable() {
    let problems: [(&str, &str, Option<Helper>); 15] = [
        ("two-teams", "two_teams", None),
        ("ferry-timetable", "ferry_times", None),
        ("toll-refunds", "toll_refunds", None),
        ("mutual-reach", "mutual_reach", None),
        ("critical-footbridges", "critical_bridges", None),
        ("backpack-weight", "best_pack", None),
        ("shared-melody", "shared_melody", None),
        ("log-cutting", "cut_cost", None),
        (
            "lantern-street",
            "brightest_street",
            Some(("<locals>.best", 3)),
        ),
        ("digit-budget", "digit_budget", Some(("<locals>.free", 2))),
        ("mirror-cuts", "mirror_cuts", None),
        ("most-talks", "most_talks", None),
        ("stepping-stones", "can_cross", None),
        ("fuel-stops", "fuel_stops", None),
        ("merge-bookings", "merge_bookings", None),
    ];
    assert_debuggable(&problems);
}
