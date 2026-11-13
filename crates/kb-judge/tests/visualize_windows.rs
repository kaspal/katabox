//! The visualizer's snapshots (ADR 0027) from the real debug harnesses, Python and
//! TypeScript, inside the Windows sandbox: shapes on real problems, identity and cycles,
//! caps and truncation, no user code, and the cost per stop. Needs the dev sandbox root:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --release --test visualize_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::Instant;

use kb_core::Language;
use kb_core::debug::{SnapObject, SnapValue, Snapshot, StopInfo, Trace};
use kb_judge::debug::{self, DebugRun, SNAPSHOT_MAX_BYTES, StopRequest};
use kb_judge::{JudgeEnv, Problem};
use kb_runner::ROOT_MARKER;
use serde_json::{Map, Value, json};

const LANGS: [Language; 2] = [Language::Python, Language::TypeScript];

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
            && root.join("toolchains/python-3.14/python.exe").is_file()
            && root.join("toolchains/node-26/node.exe").is_file(),
        "sandbox root, Python or Node missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

fn problem(id: &str) -> Problem {
    Problem::load(&repo().join("content/problems").join(id)).unwrap()
}

fn ext(lang: Language) -> &'static str {
    if lang == Language::Python { "py" } else { "ts" }
}

fn reference(id: &str, lang: Language) -> String {
    let path = repo()
        .join("content/problems")
        .join(id)
        .join(format!("solutions/ref.{}", ext(lang)));
    std::fs::read_to_string(path).unwrap()
}

fn trace(p: &Problem, lang: Language, src: &str, inp: &Map<String, Value>) -> Trace {
    match debug::trace(p, lang, src, inp, &env()).unwrap() {
        DebugRun::Trace(t) => t,
        other => panic!("{lang}: expected a trace, got {other:?}"),
    }
}

fn stop(
    p: &Problem,
    lang: Language,
    src: &str,
    inp: &Map<String, Value>,
    t: &Trace,
    step: usize,
    visualize: Option<&[&str]>,
) -> StopInfo {
    let req = StopRequest {
        step: step as u32,
        visualize: visualize.map(|v| v.iter().map(|s| s.to_string()).collect()),
        ..Default::default()
    };
    match debug::stop(p, lang, src, inp, t, &req, &env()).unwrap() {
        DebugRun::Stop(s) => s,
        other => panic!("{lang}: expected a stop, got {other:?}"),
    }
}

/// The snapshot, which must not be a failure.
fn snap(s: &StopInfo) -> &Snapshot {
    let snap = s.snapshot.as_ref().expect("a snapshot");
    assert_eq!(snap.error, None, "{snap:?}");
    snap
}

fn root<'a>(snap: &'a Snapshot, name: &str) -> &'a SnapValue {
    &snap
        .roots
        .iter()
        .find(|r| r.name == name)
        .unwrap_or_else(|| panic!("no root {name}: {:?}", snap.roots))
        .value
}

fn id_of(v: &SnapValue) -> u32 {
    match v {
        SnapValue::Ref { id } => *id,
        other => panic!("not a ref: {other:?}"),
    }
}

fn object<'a>(snap: &'a Snapshot, v: &SnapValue) -> &'a SnapObject {
    &snap.objects[id_of(v) as usize]
}

fn text(v: &SnapValue) -> &str {
    match v {
        SnapValue::Int { v }
        | SnapValue::Float { v }
        | SnapValue::Str { v }
        | SnapValue::Bool { v }
        | SnapValue::None { v }
        | SnapValue::Opaque { v } => v,
        SnapValue::Ref { id } => panic!("a ref to {id}, not a scalar"),
    }
}

/// A list's items as display text.
fn items(snap: &Snapshot, v: &SnapValue) -> (Vec<String>, u32) {
    match object(snap, v) {
        SnapObject::List { items, len, .. } => {
            (items.iter().map(|i| text(i).to_string()).collect(), *len)
        }
        other => panic!("not a list: {other:?}"),
    }
}

/// (input number, next) of a ListNode.
fn list_node(snap: &Snapshot, v: &SnapValue) -> (Option<u32>, String, SnapValue) {
    match object(snap, v) {
        SnapObject::ListNode { val, next, input } => (*input, text(val).to_string(), next.clone()),
        other => panic!("not a ListNode: {other:?}"),
    }
}

/// The 1-based line of the first line of `src` containing `needle`.
fn line_of(src: &str, needle: &str) -> u32 {
    src.lines().position(|l| l.contains(needle)).expect(needle) as u32 + 1
}

/// The first step on `line`.
fn step_on(t: &Trace, line: u32) -> usize {
    t.steps
        .iter()
        .position(|s| s.line == line)
        .unwrap_or_else(|| panic!("no step on line {line}"))
}

fn first_public(p: &Problem) -> Map<String, Value> {
    p.tests[0].input.clone()
}

// ---- Real problems ---------------------------------------------------------------------

fn real_problems(lang: Language) {
    // An array with pointers: budget-pair (prices, lo, hi).
    let p = problem("budget-pair");
    let src = reference("budget-pair", lang);
    let inp = first_public(&p);
    let t = trace(&p, lang, &src, &inp);
    let s = stop(
        &p,
        lang,
        &src,
        &inp,
        &t,
        t.steps.len() / 2,
        Some(&["prices", "lo", "hi", "missing"]),
    );
    let sn = snap(&s);
    assert_eq!(
        items(sn, root(sn, "prices")),
        (
            vec!["3", "5", "8", "12", "20"]
                .into_iter()
                .map(String::from)
                .collect(),
            5
        )
    );
    let (lo, hi): (i64, i64) = (
        text(root(sn, "lo")).parse().unwrap(),
        text(root(sn, "hi")).parse().unwrap(),
    );
    assert!(
        (0..5).contains(&lo) && (0..5).contains(&hi) && lo <= hi,
        "{lo} {hi}"
    );
    assert!(sn.roots.iter().all(|r| r.name != "missing"));
    assert!(!sn.truncated);

    // A dict / Map: two-sum's `seen` at the last step.
    let p = problem("two-sum");
    let src = reference("two-sum", lang);
    let inp = first_public(&p);
    let t = trace(&p, lang, &src, &inp);
    let s = stop(
        &p,
        lang,
        &src,
        &inp,
        &t,
        t.steps.len() - 1,
        Some(&["nums", "seen"]),
    );
    let sn = snap(&s);
    let SnapObject::Map {
        entries,
        len,
        type_name,
    } = object(sn, root(sn, "seen"))
    else {
        panic!("{sn:?}")
    };
    assert_eq!(
        type_name,
        if lang == Language::Python {
            "dict"
        } else {
            "Map"
        }
    );
    assert!(*len >= 1 && entries.len() == *len as usize, "{entries:?}");

    // A grid: orchard-plots' grid (3 × 3).
    let p = problem("orchard-plots");
    let src = reference("orchard-plots", lang);
    let inp = first_public(&p);
    let t = trace(&p, lang, &src, &inp);
    let s = stop(&p, lang, &src, &inp, &t, 2, Some(&["grid"]));
    let sn = snap(&s);
    let (rows, n) = match object(sn, root(sn, "grid")) {
        SnapObject::List { items, len, .. } => (items.clone(), *len),
        other => panic!("{other:?}"),
    };
    assert_eq!(n, 3);
    assert_eq!(items(sn, &rows[0]).0, ["3", "0", "2"]);
    assert_eq!(items(sn, &rows[2]).0, ["6", "2", "1"]);

    // ListNode: zigzag-boarding, with slow / fast labelled without being asked for.
    let p = problem("zigzag-boarding");
    let src = reference("zigzag-boarding", lang);
    let inp = first_public(&p);
    let t = trace(&p, lang, &src, &inp);
    let s = stop(&p, lang, &src, &inp, &t, 3, Some(&["queue"]));
    let sn = snap(&s);
    let mut at = root(sn, "queue").clone();
    let mut seen = vec![];
    while let SnapValue::Ref { .. } = at {
        let (input, val, next) = list_node(sn, &at);
        seen.push((input, val));
        at = next;
    }
    assert_eq!(
        seen,
        [
            (Some(0), "3"),
            (Some(1), "8"),
            (Some(2), "1"),
            (Some(3), "6"),
            (Some(4), "9")
        ]
        .map(|(i, v)| (i, v.to_string()))
    );
    assert!(sn.roots.iter().any(|r| r.name == "slow"), "{:?}", sn.roots);

    // TreeNode: window-view's root [7, 3, 9, 1, 5, null, null, null, 2].
    let p = problem("window-view");
    let src = reference("window-view", lang);
    let inp = first_public(&p);
    let t = trace(&p, lang, &src, &inp);
    let s = stop(&p, lang, &src, &inp, &t, 1, Some(&["root"]));
    let sn = snap(&s);
    let SnapObject::TreeNode {
        val,
        left,
        right,
        input,
    } = object(sn, root(sn, "root"))
    else {
        panic!("{sn:?}")
    };
    assert_eq!((text(val), *input), ("7", Some(0)));
    let SnapObject::TreeNode { val: l, .. } = object(sn, left) else {
        panic!()
    };
    let SnapObject::TreeNode {
        val: r, right: rr, ..
    } = object(sn, right)
    else {
        panic!()
    };
    assert_eq!((text(l), text(r)), ("3", "9"));
    assert!(matches!(rr, SnapValue::None { .. }));
}

#[test]
fn real_problems_python() {
    real_problems(Language::Python);
}

#[test]
fn real_problems_typescript() {
    real_problems(Language::TypeScript);
}

// ---- Identity and cycles ---------------------------------------------------------------

const CYCLE_PY: &str = "class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def zigzag_boarding(queue):
    slow = fast = queue
    tail = queue
    while tail.next is not None:
        tail = tail.next
    tail.next = queue.next
    done = True
    return None
";

const CYCLE_TS: &str = "export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function zigzagBoarding(queue: ListNode | null): ListNode | null {
  const slow = queue;
  const fast = queue;
  let tail = queue!;
  while (tail.next !== null) tail = tail.next;
  tail.next = queue!.next;
  const done = true;
  return null;
}
";

#[test]
fn identity_and_cycles() {
    for lang in LANGS {
        let src = if lang == Language::Python {
            CYCLE_PY
        } else {
            CYCLE_TS
        };
        let p = problem("zigzag-boarding");
        let inp = first_public(&p); // [3, 8, 1, 6, 9]
        let t = trace(&p, lang, src, &inp);

        // Before the cycle: the input's five nodes, numbered 0..4, ending in None.
        let early = step_on(&t, line_of(src, "tail = queue"));
        let s = stop(&p, lang, src, &inp, &t, early, Some(&["queue"]));
        let sn = snap(&s);
        let mut at = root(sn, "queue").clone();
        let mut inputs = vec![];
        while let SnapValue::Ref { .. } = at {
            let (input, _, next) = list_node(sn, &at);
            inputs.push(input.unwrap());
            at = next;
        }
        assert_eq!(inputs, [0, 1, 2, 3, 4], "{lang}");

        // After `tail.next = queue.next`: the same input numbers, and a cycle back to
        // node 1; queue, slow and fast are one object.
        let late = step_on(&t, line_of(src, "done = "));
        let s = stop(
            &p,
            lang,
            src,
            &inp,
            &t,
            late,
            Some(&["queue", "slow", "fast"]),
        );
        let sn = snap(&s);
        let q = id_of(root(sn, "queue"));
        assert_eq!(id_of(root(sn, "slow")), q, "{lang}");
        assert_eq!(id_of(root(sn, "fast")), q, "{lang}");
        assert!(
            sn.roots.iter().any(|r| r.name == "tail"),
            "node locals are added"
        );
        assert_eq!(sn.objects.len(), 5, "{lang}: each node once: {sn:?}");
        let mut at = root(sn, "queue").clone();
        let mut order = vec![];
        for _ in 0..6 {
            let (input, _, next) = list_node(sn, &at);
            order.push(input.unwrap());
            at = next;
        }
        assert_eq!(
            order,
            [0, 1, 2, 3, 4, 1],
            "{lang}: the cycle returns to node 1"
        );
        assert!(!sn.truncated);
    }
}

// ---- Caps, truncation and no user code -------------------------------------------------

const CAPS_PY: &str = r#"import builtins
import collections


class Weird:
    def __repr__(self):
        raise RuntimeError("repr ran")


class ListNode:
    def __init__(self):
        self.next = None

    @property
    def val(self):
        raise RuntimeError("property ran")


def two_sum(nums, target):
    big = list(range(100000))
    nested = []
    cur = nested
    for _ in range(100):
        nxt = []
        cur.append(nxt)
        cur = nxt
    text = "x" * 1000
    huge = 10 ** 3000
    giant = 2 ** 20000
    weird = Weird()
    fake = ListNode()
    shared = [big, big]
    counts = collections.Counter("abca")
    builtins.id = lambda o: 7
    done = True
    return [0, 1]
"#;

const CAPS_TS: &str = r#"class Weird {
  toString(): string {
    throw new Error("toString ran");
  }
  get boom(): number {
    throw new Error("getter ran");
  }
}

export class ListNode {
  next: ListNode | null = null;
  get val(): number {
    throw new Error("property ran");
  }
}

export function twoSum(nums: number[], target: number): number[] {
  const big = [...Array(100000).keys()];
  const nested: unknown[] = [];
  let cur: unknown[] = nested;
  for (let k = 0; k < 100; k++) {
    const nxt: unknown[] = [];
    cur.push(nxt);
    cur = nxt;
  }
  const text = "x".repeat(1000);
  const huge = 10n ** 3000n;
  const weird = new Weird();
  const fake = new ListNode();
  const shared = [big, big];
  const counts = new Map([["a", 2], ["b", 1], ["c", 1]]);
  const tricky: number[] = [1, 2, 3];
  Object.defineProperty(tricky, 1, { get() { throw new Error("index getter ran"); } });
  const proxy = new Proxy([1, 2, 3], {});
  Array.isArray = () => false;
  Object.getOwnPropertyDescriptor = () => undefined;
  (Map.prototype as any).entries = function () { throw new Error("patched"); };
  const done = true;
  return [0, 1];
}
"#;

#[test]
fn caps_truncation_and_no_user_code() {
    for lang in LANGS {
        let src = if lang == Language::Python {
            CAPS_PY
        } else {
            CAPS_TS
        };
        let p = problem("two-sum");
        let inp = first_public(&p);
        let t = trace(&p, lang, src, &inp);
        let at = step_on(&t, line_of(src, "done = "));
        let mut names = vec![
            "big", "nested", "text", "huge", "weird", "fake", "shared", "counts",
        ];
        if lang == Language::TypeScript {
            names.extend(["tricky", "proxy"]);
        } else {
            names.push("giant");
        }
        let s = stop(&p, lang, src, &inp, &t, at, Some(&names));
        let sn = snap(&s);
        assert!(sn.truncated, "{lang}");

        // A long list keeps its real length; the items stop at the cap.
        let (big, len) = items(sn, root(sn, "big"));
        assert_eq!((big.len(), len), (500, 100_000), "{lang}");
        assert_eq!(&big[..3], ["0", "1", "2"]);
        // Shared: both items are the same object as `big` (identity survives a patched
        // builtins.id / Array.isArray).
        let SnapObject::List { items: sh, .. } = object(sn, root(sn, "shared")) else {
            panic!()
        };
        assert_eq!(id_of(&sh[0]), id_of(root(sn, "big")), "{lang}");
        assert_eq!(id_of(&sh[1]), id_of(root(sn, "big")), "{lang}");
        assert_ne!(id_of(root(sn, "nested")), id_of(root(sn, "big")));

        // Depth: the nested lists stop at the cap with an opaque value.
        let mut v = root(sn, "nested").clone();
        let mut depth = 0;
        while let SnapValue::Ref { .. } = v {
            let SnapObject::List { items, .. } = object(sn, &v) else {
                panic!()
            };
            v = items[0].clone();
            depth += 1;
        }
        assert!(matches!(v, SnapValue::Opaque { .. }), "{lang}: {v:?}");
        assert!((30..=33).contains(&depth), "{lang}: depth {depth}");

        // Scalars are cut; a huge integer says how many digits it has.
        let t = text(root(sn, "text"));
        assert_eq!(t.chars().count(), 64, "{lang}");
        assert!(t.ends_with('…'));
        assert!(
            text(root(sn, "huge")).contains("(3001 digits)"),
            "{lang}: {}",
            text(root(sn, "huge"))
        );

        // User code never ran: these are opaque instead of raising.
        assert_eq!(
            root(sn, "weird"),
            &SnapValue::Opaque { v: "Weird".into() },
            "{lang}"
        );
        assert_eq!(
            root(sn, "fake"),
            &SnapValue::Opaque {
                v: "ListNode".into()
            },
            "{lang}"
        );
        let SnapObject::Map { len, entries, .. } = object(sn, root(sn, "counts")) else {
            panic!()
        };
        assert_eq!(
            (*len, entries.len()),
            (3, 3),
            "{lang}: the saved Map iterator ignores the patch"
        );
        if lang == Language::Python {
            // Beyond str()'s digit limit: the size only.
            assert_eq!(text(root(sn, "giant")), "≈2^20001");
        }
        if lang == Language::TypeScript {
            let SnapObject::List { items, .. } = object(sn, root(sn, "tricky")) else {
                panic!()
            };
            assert_eq!(text(&items[0]), "1");
            assert_eq!(
                items[1],
                SnapValue::Opaque {
                    v: "accessor".into()
                }
            );
            assert_eq!(root(sn, "proxy"), &SnapValue::Opaque { v: "Proxy".into() });
        }
        // Still well inside the byte cap.
        let size = serde_json::to_string(sn).unwrap().len();
        assert!(size < SNAPSHOT_MAX_BYTES, "{lang}: {size}");
    }
}

// ---- Cost per stop ---------------------------------------------------------------------

const COST_PY: &str = "def two_sum(nums, target):
    grid = [[r * 100 + c for c in range(100)] for r in range(100)]
    seen = {i: -i for i in range(5000)}
    done = True
    return [0, 1]
";

const COST_TS: &str = "export function twoSum(nums: number[], target: number): number[] {
  // Built without callbacks (each callback pause would be a step of the trace).
  const row = [...Array(100).keys()];
  const grid: number[][] = JSON.parse(JSON.stringify(Array(100).fill(row)));
  const seen = new Map([...Array(5000).keys()].entries());
  const done = true;
  return [0, 1];
}
";

/// The snapshot's cost at its caps: the same stop with and without `visualize`, three
/// times each (median). The target is 150 ms of overhead; the assertion allows twice that
/// so a busy machine does not fail it.
#[test]
fn cost_per_stop_at_the_caps() {
    for lang in LANGS {
        let src = if lang == Language::Python {
            COST_PY
        } else {
            COST_TS
        };
        let p = problem("two-sum");
        let nums: Vec<i64> = (0..100_000).collect();
        let inp = json!({"nums": nums, "target": 3})
            .as_object()
            .unwrap()
            .clone();
        let t = trace(&p, lang, src, &inp);
        let at = step_on(&t, line_of(src, "done = "));
        let timed = |viz: Option<&[&str]>| {
            let mut ms = vec![];
            let mut last = None;
            for _ in 0..3 {
                let start = Instant::now();
                last = Some(stop(&p, lang, src, &inp, &t, at, viz));
                ms.push(start.elapsed().as_secs_f64() * 1000.0);
            }
            ms.sort_by(f64::total_cmp);
            (ms[1], last.unwrap())
        };
        let (plain, _) = timed(None);
        let (with, s) = timed(Some(&["nums", "grid", "seen"]));
        let sn = snap(&s);
        let size = serde_json::to_string(sn).unwrap().len();
        let overhead = with - plain;
        println!(
            "{lang}: stop {plain:.0} ms, with snapshot {with:.0} ms (overhead {overhead:.0} ms, target < 150); snapshot {size} bytes, {} objects, truncated {}",
            sn.objects.len(),
            sn.truncated
        );
        assert!(sn.truncated, "{lang}: the caps were reached");
        assert!(size <= SNAPSHOT_MAX_BYTES, "{lang}: {size}");
        assert!(
            overhead < 300.0,
            "{lang}: snapshot overhead {overhead:.0} ms"
        );
    }
}
