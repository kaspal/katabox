//! The debugger's replay runs for JavaScript/TypeScript (ADR 0023): Node's in-process
//! inspector inside the Windows sandbox. Needs the dev sandbox root with Node:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --test debug_js_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::Instant;

use kb_core::Language;
use kb_core::debug::{OutcomeStatus, StopInfo, Trace, Truncation};
use kb_judge::debug::{self, DebugRun, StopRequest};
use kb_judge::{JudgeEnv, Problem};
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
        root.join(ROOT_MARKER).is_file() && root.join("toolchains/node-26/node.exe").is_file(),
        "sandbox root or Node missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

fn problem(id: &str) -> Problem {
    Problem::load(&repo().join("content/problems").join(id)).unwrap()
}

fn reference(id: &str) -> String {
    std::fs::read_to_string(
        repo()
            .join("content/problems")
            .join(id)
            .join("solutions/ref.ts"),
    )
    .unwrap()
}

fn input(v: Value) -> Map<String, Value> {
    v.as_object().unwrap().clone()
}

fn trace(p: &Problem, source: &str, inp: &Map<String, Value>) -> Trace {
    match debug::trace(p, Language::TypeScript, source, inp, &env()).unwrap() {
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
) -> StopInfo {
    match debug::stop(p, Language::TypeScript, source, inp, t, &req, &env()).unwrap() {
        DebugRun::Stop(s) => s,
        other => panic!("expected a stop, got {other:?}"),
    }
}

fn at(step: usize) -> StopRequest {
    StopRequest {
        step: step as u32,
        ..Default::default()
    }
}

fn local<'a>(s: &'a StopInfo, name: &str) -> &'a str {
    &s.locals
        .iter()
        .find(|v| v.name == name)
        .unwrap_or_else(|| panic!("no local {name}: {:?}", s.locals))
        .value
}

fn lines(t: &Trace) -> Vec<u32> {
    t.steps.iter().map(|s| s.line).collect()
}

/// Two-sum's TypeScript reference: lines, variables, an expansion, evaluations, the result.
#[test]
fn typescript_two_sum_variables_evaluations_and_result() {
    let p = problem("two-sum");
    let src = reference("two-sum");
    let inp = input(json!({"nums": [5, 8, 12, 1, 3], "target": 15}));
    let t = trace(&p, &src, &inp);
    // Line 2 once, then (3, 4, 5, 6) per pass, and the return on the fifth pass.
    let mut want = vec![2];
    for _ in 0..4 {
        want.extend([3, 4, 5, 6]);
    }
    want.extend([3, 4, 5]);
    assert_eq!(lines(&t), want);
    assert!(t.steps.iter().all(|s| s.depth == 1));
    let o = t.outcome.clone().unwrap();
    assert_eq!(o.value, json!([2, 4]));
    assert_eq!(o.step, t.steps.len() as u32 - 1);

    // The second time at `seen.set(...)`: seen holds 5 → 0 and nums[i] is 8.
    let hits: Vec<usize> = (0..t.steps.len())
        .filter(|&i| t.steps[i].line == 6)
        .collect();
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: hits[1] as u32,
            expand: vec![vec![json!("locals"), json!("seen")]],
            evals: vec![
                "nums[i]".into(),
                "boom".into(),
                "console.log('x'), seen.clear(), 7".into(),
            ],
            ..Default::default()
        },
    );
    assert_eq!(s.stack.len(), 1);
    assert_eq!(
        (s.stack[0].function.as_str(), s.stack[0].line),
        ("twoSum", 6)
    );
    assert_eq!(local(&s, "seen"), "Map(1) { 5 => 0 }");
    assert_eq!(local(&s, "nums"), "[ 5, 8, 12, 1, 3 ]");
    assert_eq!(s.expanded[0].children[0].name, "5");
    assert_eq!(s.expanded[0].children[0].value, "0");
    assert_eq!(s.evals[0].value.as_ref().unwrap().value, "8");
    assert_eq!(
        s.evals[1].error.as_deref(),
        Some("ReferenceError: boom is not defined")
    );
    assert_eq!(s.evals[2].stdout, "x\n");
    // That evaluation cleared `seen`, but every stop is a fresh replay.
    let later = stop(&p, &src, &inp, &t, at(hits[2]));
    assert_eq!(local(&later, "seen"), "Map(2) { 5 => 0, 8 => 1 }");
}

/// Node's type stripping keeps every line where it was, so steps and breakpoints map 1:1.
#[test]
fn typescript_lines_map_one_to_one() {
    let p = problem("two-sum");
    let src = r#"import type { Unused } from "./nowhere";

interface Pair {
  a: number;
  b: number;
}

type Index<
  T,
> = Map<T, number>;

export function twoSum(
  nums: number[],
  target: number,
): number[] {
  const seen: Index<number> = new Map<
    number,
    number
  >();
  for (let i = 0; i < nums.length; i++) {
    const pair: Pair | undefined = find<number>(
      seen,
      target - nums[i],
      i,
    );
    if (pair) return [pair.a, pair.b];
    seen.set(nums[i], i);
  }
  return [];
}

function find<T>(seen: Index<T>, want: T, i: number): Pair | undefined {
  const j = seen.get(want);
  return j === undefined ? undefined : { a: j, b: i };
}
"#;
    let inp = input(json!({"nums": [3, 4], "target": 7}));
    let t = trace(&p, src, &inp);
    let got: Vec<(u32, u32)> = t.steps.iter().map(|s| (s.line, s.depth)).collect();
    // The lines of the statements exactly as written: 16 `const seen`, 20 the loop, 21
    // the call to find (33-34 inside it), 26 the `if`, 27 `seen.set`.
    assert_eq!(
        got,
        [
            (16, 1),
            (20, 1),
            (21, 1),
            (33, 2),
            (34, 2),
            (26, 1),
            (27, 1),
            (20, 1),
            (21, 1),
            (33, 2),
            (34, 2),
            (26, 1)
        ]
    );
    assert_eq!(t.outcome.clone().unwrap().value, json!([0, 1]));
    let inside = got.iter().position(|s| *s == (33, 2)).unwrap();
    let s = stop(&p, src, &inp, &t, at(inside));
    let names: Vec<(&str, u32)> = s
        .stack
        .iter()
        .map(|f| (f.function.as_str(), f.line))
        .collect();
    assert_eq!(names, [("find", 33), ("twoSum", 21)]);
}

/// Plain JavaScript runs as TypeScript too; recursion shows in the depths and the stack.
#[test]
fn javascript_recursion_depths_and_stack() {
    let p = problem("outfit-picker");
    let src = "export function outfits(prices, budget) {
  const groups = prices.map((g) => [...g].sort((a, b) => a - b));
  const cheapest = new Array(groups.length + 1).fill(0);
  for (let c = groups.length - 1; c >= 0; c--) cheapest[c] = cheapest[c + 1] + groups[c][0];
  function choose(c, spent) {
    if (c === groups.length) return 1;
    let total = 0;
    for (const price of groups[c]) {
      if (spent + price + cheapest[c + 1] > budget) break;
      total += choose(c + 1, spent + price);
    }
    return total;
  }
  return choose(0, 0);
}
";
    let inp = input(json!({"prices": [[30, 50], [20, 40, 60], [10]], "budget": 100}));
    let t = trace(&p, src, &inp);
    assert_eq!(t.outcome.clone().unwrap().value, json!(5));
    let deepest = (0..t.steps.len())
        .max_by_key(|&i| (t.steps[i].depth, std::cmp::Reverse(i)))
        .unwrap();
    // outfits → choose(0) → choose(1) → choose(2) → choose(3).
    assert_eq!(t.steps[deepest].depth, 5);
    assert_eq!(t.steps[deepest].line, 6);
    let s = stop(&p, src, &inp, &t, at(deepest));
    let names: Vec<&str> = s.stack.iter().map(|f| f.function.as_str()).collect();
    assert_eq!(names, ["choose", "choose", "choose", "choose", "outfits"]);
    assert_eq!(local(&s, "c"), "3");
    assert_eq!(local(&s, "spent"), "60");
    // The outer frame shows its own variables.
    let outer = stop(
        &p,
        src,
        &inp,
        &t,
        StopRequest {
            step: deepest as u32,
            frame: 4,
            ..Default::default()
        },
    );
    assert_eq!(outer.frame, 4);
    assert_eq!(local(&outer, "budget"), "100");
}

#[test]
fn prints_are_tagged_with_their_steps() {
    let p = problem("two-sum");
    let src = "export function twoSum(nums, target) {
  console.log('start');
  for (let i = 0; i < 2; i++) {
    process.stdout.write('i=' + i + '\\n');
  }
  return [0, 1];
}
";
    let t = trace(&p, src, &input(json!({"nums": [1, 2], "target": 3})));
    assert_eq!(lines(&t), [2, 3, 4, 3, 4, 3, 6]);
    let chunks: Vec<(u32, &str)> = t.prints.iter().map(|c| (c.step, c.text.as_str())).collect();
    assert_eq!(chunks, [(0, "start\n"), (2, "i=0\n"), (4, "i=1\n")]);
}

#[test]
fn long_loops_are_truncated_at_the_step_or_pause_cap() {
    let p = problem("two-sum");
    let src = "export function twoSum(nums, target) {
  let s = 0;
  for (let i = 0; i < 1e7; i++) {
    s += i;
  }
  return [s, 0];
}
";
    let inp = input(json!({"nums": [1, 2], "target": 3}));
    let started = Instant::now();
    let t = trace(&p, src, &inp);
    let traced = started.elapsed();
    assert_eq!(t.truncated, Some(Truncation::Steps));
    assert!(t.outcome.is_none());
    assert!(t.steps.len() as u32 <= debug::JS_MAX_STEPS);
    let started = Instant::now();
    let s = stop(&p, src, &inp, &t, at(t.steps.len() - 1));
    let stopped = started.elapsed();
    assert_eq!(s.stack[0].line, t.steps.last().unwrap().line);
    println!(
        "loop: {} steps, trace {traced:?}, stop at the last step {stopped:?}",
        t.steps.len()
    );

    // A loop on one line is one step but many pauses: the pause cap ends it.
    let one_line = "export function twoSum(nums, target) {
  let s = 0;
  for (let i = 0; i < 1e7; i++) s += i;
  return [s, 0];
}
";
    let started = Instant::now();
    let t = trace(&p, one_line, &inp);
    assert_eq!(t.truncated, Some(Truncation::Steps));
    assert!(t.steps.len() < 5, "{:?}", lines(&t));
    println!(
        "one-line loop: {} steps, trace {:?}",
        t.steps.len(),
        started.elapsed()
    );
}

#[test]
fn seeded_randomness_replays_and_other_randomness_diverges() {
    let p = problem("two-sum");
    // Math.random is seeded: the same branches on every replay.
    let seeded = "export function twoSum(nums, target) {
  let n = 0;
  for (let i = 0; i < 12; i++) {
    if (Math.random() < 0.5) {
      n += 1;
    } else {
      n += 2;
    }
  }
  return [n, 0];
}
";
    let inp = input(json!({"nums": [1, 2], "target": 3}));
    let t = trace(&p, seeded, &inp);
    for step in [5, t.steps.len() - 1] {
        stop(&p, seeded, &inp, &t, at(step));
    }
    // crypto is not seeded: twelve coin flips replay the same way 1 time in 4096.
    let unseeded = seeded.replace(
        "Math.random()",
        "crypto.getRandomValues(new Uint8Array(1))[0] / 256",
    );
    let t = trace(&p, &unseeded, &inp);
    let r = debug::stop(
        &p,
        Language::TypeScript,
        &unseeded,
        &inp,
        &t,
        &at(t.steps.len() - 1),
        &env(),
    )
    .unwrap();
    assert_eq!(r, DebugRun::Diverged);
}

#[test]
fn exceptions_load_errors_and_unsupported_languages() {
    let p = problem("two-sum");
    let inp = input(json!({"nums": [1, 2], "target": 3}));
    let throws = "export function twoSum(nums, target) {
  const x = nums[0];
  return nums.missing.length;
}
";
    let t = trace(&p, throws, &inp);
    let o = t.outcome.unwrap();
    assert_eq!(o.status, OutcomeStatus::Error);
    assert_eq!(o.line, Some(3));
    let err = o.error.unwrap();
    assert!(
        err.contains("TypeError") && !err.contains("main.ts"),
        "{err}"
    );

    let broken = "export function twoSum(nums, target {\n  return [];\n}\n";
    let r = debug::trace(&p, Language::TypeScript, broken, &inp, &env()).unwrap();
    assert!(matches!(r, DebugRun::LoadError(_)), "{r:?}");
    let missing = "export function other() {}\n";
    let r = debug::trace(&p, Language::TypeScript, missing, &inp, &env()).unwrap();
    assert!(
        matches!(&r, DebugRun::LoadError(m) if m.contains("twoSum")),
        "{r:?}"
    );

    assert!(debug::supported(Language::TypeScript));
    assert!(!debug::supported(Language::Php));
    assert!(!debug::supported(Language::Rust));
}

/// Content batch 5: every new problem's ref.ts on its first public test, within the JS
/// step cap: the result passes the checker, and stops halfway and at the last step show
/// the entry function outermost.
#[test]
fn batch_5_problems_are_debuggable_in_typescript() {
    let problems = [
        ("shelf-codes", "countBelow"),
        ("lamp-hours", "litTime"),
        ("path-tiles", "tilings"),
        ("fewest-hops", "fewestHops"),
        ("club-merges", "clubsLeft"),
        ("balanced-streaks", "balancedStretches"),
        ("spice-market", "bestHaul"),
        ("first-match", "firstMatches"),
        ("shelf-order", "naturalOrder"),
        ("feature-flags", "usersWithAll"),
        ("page-wrap", "wrapLines"),
        ("undo-notebook", "notebook"),
        ("shelf-packing", "fewestShelves"),
        ("pocket-calculator", "evaluate"),
        ("thermostat-swings", "windowSwings"),
    ];
    assert_debuggable(&problems);
}

/// The node-type problems (ADR 0025): debuggable like the others.
#[test]
fn node_problems_are_debuggable_in_typescript() {
    assert_debuggable(&[
        ("playlist-rotation", "rotatePlaylist"),
        ("checkout-merge", "mergeLines"),
        ("fullest-level", "fullestLevel"),
        ("hedge-trim", "trimHedge"),
    ]);
}

/// Nodes read as a short line with their fields as children in a fixed order (ADR 0025).
#[test]
fn typescript_nodes_are_shown_readably() {
    let p = problem("playlist-rotation");
    let src = reference("playlist-rotation");
    let inp = p.tests[0].input.clone();
    let t = trace(&p, &src, &inp);
    assert_eq!(
        t.outcome.as_ref().unwrap().value,
        serde_json::json!([16, 23, 4, 8, 15])
    );
    let s = stop(
        &p,
        &src,
        &inp,
        &t,
        StopRequest {
            step: 0,
            expand: vec![vec![
                serde_json::json!("locals"),
                serde_json::json!("playlist"),
            ]],
            ..Default::default()
        },
    );
    assert_eq!(local(&s, "playlist"), "ListNode [4 → 8 → 15 → 16 → 23]");
    let names: Vec<(&str, &str)> = s.expanded[0]
        .children
        .iter()
        .map(|c| (c.name.as_str(), c.value.as_str()))
        .collect();
    assert_eq!(
        names,
        [("val", "4"), ("next", "ListNode [8 → 15 → 16 → 23]")]
    );

    let p = problem("hedge-trim");
    let src = reference("hedge-trim");
    let inp = p.tests[0].input.clone();
    let t = trace(&p, &src, &inp);
    assert_eq!(
        t.outcome.as_ref().unwrap().value,
        serde_json::json!([3, null, 2])
    );
    let s = stop(&p, &src, &inp, &t, at(0));
    assert_eq!(local(&s, "root"), "TreeNode(3, left=-5, right=2)");
}

/// Each problem's ref.ts on its first public test, within the JS step cap: the result
/// passes the checker, and stops halfway and at the last step show the entry function
/// outermost.
fn assert_debuggable(problems: &[(&str, &str)]) {
    for &(id, entry) in problems {
        let p = problem(id);
        let src = reference(id);
        let public = &p.tests[0];
        let started = Instant::now();
        let t = trace(&p, &src, &public.input);
        assert!(t.truncated.is_none(), "{id}: truncated");
        assert!(t.steps.len() as u32 <= debug::JS_MAX_STEPS, "{id}");
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
            let s = stop(&p, &src, &public.input, &t, at(step));
            assert!(!s.stack.is_empty(), "{id}@{step}: empty stack");
            assert_eq!(
                s.stack.last().unwrap().function,
                entry,
                "{id}@{step}: {:?}",
                s.stack
            );
        }
        println!(
            "{id:<20} {:>5} steps, max depth {}, {:?} for the trace and two stops",
            t.steps.len(),
            t.steps.iter().map(|s| s.depth).max().unwrap(),
            started.elapsed()
        );
    }
}

/// Content to 150, milestone 1: fundamentals, range queries, geometry, road repairs.
#[test]
fn content_150_m1_problems_are_debuggable_in_typescript() {
    assert_debuggable(&[
        ("temperature-swing", "temperatureSwing"),
        ("grade-bands", "gradeBands"),
        ("vowel-tally", "vowelTally"),
        ("leap-years", "leapYears"),
        ("halving-steps", "halvingSteps"),
        ("title-case", "titleCase"),
        ("mileage-log", "tripTotals"),
        ("stock-ledger", "stockLedger"),
        ("gauge-corrections", "gaugeReadings"),
        ("cold-snap", "coldSnaps"),
        ("road-repairs", "roadRepairs"),
        ("turn-signals", "turnSignals"),
        ("fenced-garden", "whereIsPoint"),
        ("crossing-paths", "pathsCross"),
        ("tree-fence", "fenceArea"),
    ]);
}

/// Content to 150, milestone 2: linked lists, trees, tries, heap, backtracking.
#[test]
fn content_150_m2_problems_are_debuggable_in_typescript() {
    assert_debuggable(&[
        ("two-queues-merge", "mergeQueues"),
        ("split-the-train", "splitTrain"),
        ("zigzag-boarding", "zigzagBoarding"),
        ("batch-reversal", "reverseBatches"),
        ("window-view", "windowView"),
        ("bracket-notation", "bracketNotation"),
        ("catalogue-check", "isSearchTree"),
        ("kth-smallest-code", "kthSmallest"),
        ("garden-walk", "longestWalk"),
        ("rebuild-tree", "rebuildTree"),
        ("richest-trail", "richestTrail"),
        ("best-code-pair", "bestXorPair"),
        ("word-grid-hunt", "findWords"),
        ("nearest-stalls", "nearestStalls"),
        ("coin-bag-combos", "coinCombos"),
    ]);
}
