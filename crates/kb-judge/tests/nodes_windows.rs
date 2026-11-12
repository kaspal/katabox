//! ListNode / TreeNode round trips (ADR 0025): every language's harness builds the input
//! nodes from JSON and writes the returned ones back, including empty, single-node, gappy,
//! very long and very deep structures, and refuses cycles and shared nodes with a clear
//! Runtime Error. Needs the dev sandbox root with the toolchains:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --test nodes_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::{Language, MessageCode, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, judge, starter_code};
use kb_runner::ROOT_MARKER;
use serde_json::{Value, json};

fn env() -> JudgeEnv {
    let root =
        std::path::absolute(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/katabox"))
            .unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file(),
        "no sandbox root at {}",
        root.display()
    );
    JudgeEnv::dev(root)
}

const ALL: [Language; 4] = [
    Language::Python,
    Language::TypeScript,
    Language::Php,
    Language::Rust,
];

/// A problem `name(param: ty) -> ret` whose tests expect `outputs[i]` for `inputs[i]`.
fn problem(name: &str, param: &str, ty: &str, ret: &str, cases: &[(Value, Value)]) -> Problem {
    let tests = cases
        .iter()
        .enumerate()
        .map(|(i, (input, output))| TestCase {
            kind: TestKind::Public,
            index: i as u32 + 1,
            input: json!({ param: input }).as_object().unwrap().clone(),
            expected: output.clone(),
        })
        .collect();
    let sig = json!({"name": name, "params": [{"name": param, "type": ty}], "returns": ret});
    // A generous limit: the deep and long cases are about correctness, not speed.
    Problem::from_parts(name, &sig, "exact", 2000, 512, tests).unwrap()
}

/// The starter code with its body replaced by `body` (the node definitions stay).
fn solution(lang: Language, p: &Problem, body: &str) -> String {
    let stub = starter_code(lang, &p.signature).unwrap();
    let stub = match lang {
        Language::Python => stub.replace("    pass\n", body),
        Language::TypeScript | Language::Php => stub.replacen("{\n\n}", &format!("{{\n{body}}}"), 1),
        Language::Rust => stub.replace(
            "    // Do not write `fn main`: KataBox provides it and calls this function.\n    todo!()\n",
            body,
        ),
        _ => unreachable!(),
    };
    stub
}

fn skewed(depth: usize) -> Value {
    // A right-leaning chain in level order: 1, null, 2, null, 3, ...
    let mut v = vec![json!(1)];
    for i in 2..=depth {
        v.push(Value::Null);
        v.push(json!(i));
    }
    Value::Array(v)
}

fn assert_accepted(lang: Language, p: &Problem, source: &str) {
    let r = judge(p, &p.tests, lang, source, &env()).unwrap();
    assert_eq!(
        r.verdict,
        Verdict::Accepted,
        "{lang}: {}\n{source}",
        serde_json::to_string_pretty(&r.first_failure).unwrap()
    );
}

fn assert_cycle(lang: Language, p: &Problem, source: &str) {
    let started = std::time::Instant::now();
    let r = judge(p, &p.tests[..1], lang, source, &env()).unwrap();
    assert_eq!(r.verdict, Verdict::RuntimeError, "{lang}: {r:?}");
    let m = r
        .first_failure
        .as_ref()
        .and_then(|f| f.message.clone())
        .unwrap();
    assert_eq!(m.code, MessageCode::NodeCycle, "{lang}: {m:?}");
    assert!(started.elapsed().as_secs() < 30, "{lang}: slow");
}

#[test]
fn lists_round_trip_and_reverse_with_user_nodes() {
    let long: Vec<i64> = (0..100_000).collect();
    let mut rev = long.clone();
    rev.reverse();
    let cases = [
        (json!([]), json!([])),
        (json!([7]), json!([7])),
        (json!([1, 2, 3]), json!([3, 2, 1])),
        (json!(long), json!(rev)),
    ];
    let p = problem("reverseList", "head", "ListNode", "ListNode", &cases);
    // Builds a new list with the class's own constructor (nodes the harness never made).
    let bodies = [
        (
            Language::Python,
            "    out = None\n    while head is not None:\n        out = ListNode(head.val, out)\n        head = head.next\n    return out\n",
        ),
        (
            Language::TypeScript,
            "  let out: ListNode | null = null;\n  for (let n = head; n !== null; n = n.next) out = new ListNode(n.val, out);\n  return out;\n",
        ),
        (
            Language::Php,
            "    $out = null;\n    for ($n = $head; $n !== null; $n = $n->next) { $out = new ListNode($n->val, $out); }\n    return $out;\n",
        ),
        (
            Language::Rust,
            "    let mut out: Option<Box<ListNode>> = None;\n    let mut cur = head;\n    while let Some(mut n) = cur {\n        cur = n.next.take();\n        n.next = out;\n        out = Some(n);\n    }\n    out\n",
        ),
    ];
    for (lang, body) in bodies {
        assert_accepted(lang, &p, &solution(lang, &p, body));
    }
    // A list that loops back is refused (Rust's Box list cannot loop).
    let cycles = [
        (
            Language::Python,
            "    a = ListNode(1)\n    a.next = ListNode(2, a)\n    return a\n",
        ),
        (
            Language::TypeScript,
            "  const a = new ListNode(1);\n  a.next = new ListNode(2, a);\n  return a;\n",
        ),
        (
            Language::Php,
            "    $a = new ListNode(1);\n    $a->next = new ListNode(2, $a);\n    return $a;\n",
        ),
    ];
    for (lang, body) in cycles {
        assert_cycle(lang, &p, &solution(lang, &p, body));
    }
}

#[test]
fn trees_round_trip_including_a_deep_one() {
    let cases = [
        (json!([]), json!([])),
        (json!([1]), json!([1])),
        (json!([1, null, 2, 3]), json!([1, null, 2, 3])),
        (
            json!([5, 3, 8, 1, null, 7, 9, null, 2]),
            json!([5, 3, 8, 1, null, 7, 9, null, 2]),
        ),
        (skewed(10_000), skewed(10_000)),
    ];
    let p = problem("sameTree", "root", "TreeNode", "TreeNode", &cases);
    let bodies = [
        (Language::Python, "    return root\n"),
        (Language::TypeScript, "  return root;\n"),
        (Language::Php, "    return $root;\n"),
        (Language::Rust, "    root\n"),
    ];
    for (lang, body) in bodies {
        assert_accepted(lang, &p, &solution(lang, &p, body));
    }
    // A node shared by two parents is refused.
    let shared = [
        (
            Language::Python,
            "    leaf = TreeNode(2)\n    return TreeNode(1, leaf, leaf)\n",
        ),
        (
            Language::TypeScript,
            "  const leaf = new TreeNode(2);\n  return new TreeNode(1, leaf, leaf);\n",
        ),
        (
            Language::Php,
            "    $leaf = new TreeNode(2);\n    return new TreeNode(1, $leaf, $leaf);\n",
        ),
        (
            Language::Rust,
            "    let leaf = Some(Rc::new(RefCell::new(TreeNode::new(2))));\n    let mut top = TreeNode::new(1);\n    top.left = leaf.clone();\n    top.right = leaf;\n    Some(Rc::new(RefCell::new(top)))\n",
        ),
    ];
    for (lang, body) in shared {
        assert_cycle(lang, &p, &solution(lang, &p, body));
    }
}

#[test]
fn arrays_of_lists_and_a_solution_without_the_class() {
    let cases = [
        (json!([]), json!([])),
        (json!([[], [1, 2], []]), json!([[], [1, 2], []])),
    ];
    let p = problem("sameLists", "lists", "ListNode[]", "ListNode[]", &cases);
    for lang in ALL {
        let body = match lang {
            Language::Python => "    return lists\n",
            Language::TypeScript => "  return lists;\n",
            Language::Php => "    return $lists;\n",
            _ => "    lists\n",
        };
        assert_accepted(lang, &p, &solution(lang, &p, body));
    }
    // Python, TypeScript and PHP fall back to the judge's own class when a solution
    // dropped the starter definition.
    let bare = [
        (
            Language::Python,
            "def same_lists(lists):\n    return lists\n",
        ),
        (
            Language::TypeScript,
            "export function sameLists(lists: unknown[]) {\n  return lists;\n}\n",
        ),
        (
            Language::Php,
            "<?php\nfunction sameLists(array $lists): array {\n    return $lists;\n}\n",
        ),
    ];
    for (lang, source) in bare {
        assert_accepted(lang, &p, source);
    }
}
