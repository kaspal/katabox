//! End-to-end judge tests on two-sum: real Python, Node, PHP and rustc inside the Windows
//! sandbox. Needs the dev sandbox root with the pinned toolchains:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1      # Python, Node, PHP and Rust in target/katabox
//! cargo test -p kb-judge --test e2e_windows
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::{Language, Message, MessageCode, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, Report, judge, starter_code};
use kb_runner::ROOT_MARKER;
use serde_json::json;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The dev sandbox root (`target/katabox`, or `KB_TEST_ROOT`).
fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    let tc = root.join("toolchains");
    assert!(
        root.join(ROOT_MARKER).is_file()
            && tc.join("python-3.14/python.exe").is_file()
            && tc.join("node-26/node.exe").is_file()
            && tc.join("php-8.5/php.exe").is_file()
            && tc.join("rust-1.99/bin/rustc.exe").is_file(),
        "sandbox root or toolchains missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

fn two_sum() -> Problem {
    Problem::load(&repo().join("content/problems/two-sum")).unwrap()
}

/// Judge `source` against every two-sum test.
fn run(lang: Language, source: &str) -> Report {
    let p = two_sum();
    let report = judge(&p, &p.tests, lang, source, &env()).unwrap();
    eprintln!("{lang}: {}", serde_json::to_string_pretty(&report).unwrap());
    report
}

/// The two-sum tests small enough to print every input (not the 10⁴-number max tests).
fn small_tests() -> Vec<kb_core::TestCase> {
    two_sum()
        .tests
        .into_iter()
        .filter(|t| serde_json::to_string(&t.input).unwrap().len() < 1000)
        .collect()
}

/// Judge `source` against the small tests only (tests that print every input).
fn run_small(lang: Language, source: &str) -> Report {
    let report = judge(&two_sum(), &small_tests(), lang, source, &env()).unwrap();
    eprintln!("{lang}: {}", serde_json::to_string_pretty(&report).unwrap());
    report
}

fn reference(lang: Language) -> String {
    let path = repo()
        .join("content/problems/two-sum/solutions")
        .join(format!("ref.{}", lang.extension()));
    std::fs::read_to_string(path).unwrap()
}

/// The first failure's message, asserting its code.
fn failure_message(r: &Report, code: MessageCode) -> String {
    let m: Message = r
        .first_failure
        .as_ref()
        .and_then(|f| f.message.clone())
        .expect("a failure with a message");
    assert_eq!(m.code, code, "{m:?}");
    m.text
}

/// The compiler output, asserting its code.
fn compile_output(r: &Report, code: MessageCode) -> String {
    let m = r.compile_output.clone().expect("compile output");
    assert_eq!(m.code, code, "{m:?}");
    m.text
}

// ---- Accepted ---------------------------------------------------------------------

#[test]
fn accepted_python_reference() {
    let r = run(Language::Python, &reference(Language::Python));
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.tests.len(), two_sum().tests.len());
    assert_eq!(r.total_tests as usize, two_sum().tests.len());
    assert!(r.first_failure.is_none());
    assert!(
        r.tests
            .iter()
            .all(|t| t.result.verdict == Verdict::Accepted)
    );
    assert_eq!(r.tests[3].result.kind, TestKind::Hidden);
    let sum: u64 = r.tests.iter().map(|t| t.result.time_us).sum();
    assert_eq!(r.time_us, sum);
}

#[test]
fn accepted_typescript_reference() {
    let r = run(Language::TypeScript, &reference(Language::TypeScript));
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.tests.len(), two_sum().tests.len());
}

#[test]
fn unordered_checker_accepts_reversed_pair() {
    let src = "export function twoSum(nums: number[], target: number): number[] {\n\
               for (let i = 0; i < nums.length; i++)\n\
                 for (let j = i + 1; j < nums.length; j++)\n\
                   if (nums[i] + nums[j] === target) return [j, i];\n\
               return [];\n}\n";
    assert_eq!(run(Language::TypeScript, src).verdict, Verdict::Accepted);
}

// ---- Wrong Answer -----------------------------------------------------------------

#[test]
fn wrong_answer_python_reports_first_failing_input() {
    let r = run(
        Language::Python,
        "def two_sum(nums, target):\n    return [2, 4]\n",
    );
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    // Public 1 expects [2, 4]; public 2 is the first that does not.
    assert_eq!(r.tests[0].result.verdict, Verdict::Accepted);
    let f = r.first_failure.unwrap();
    assert_eq!((f.kind, f.index), (TestKind::Public, 2));
    assert_eq!(f.input.get("target"), Some(&json!(2)));
    assert_eq!(f.expected, json!([1, 3]));
    assert_eq!(f.actual, Some(json!([2, 4])));
    let m = f.message.unwrap();
    assert_eq!(m.code, MessageCode::OutputMismatch);
    assert_eq!(m.text, "expected [1,3], got [2,4]");
}

#[test]
fn wrong_answer_typescript() {
    let r = run(
        Language::TypeScript,
        "export function twoSum(nums: number[], target: number): number[] {\n  return [-1, -1];\n}\n",
    );
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    assert_eq!(
        r.tests.len(),
        two_sum().tests.len(),
        "every test still runs"
    );
    assert_eq!(r.first_failure.unwrap().index, 1);
}

#[test]
fn starter_stubs_parse_and_get_wrong_answer() {
    let sig = two_sum().signature;
    for lang in [Language::Python, Language::TypeScript] {
        let r = run(lang, &starter_code(lang, &sig).unwrap());
        assert_eq!(r.verdict, Verdict::WrongAnswer, "{lang}");
        assert_eq!(r.first_failure.unwrap().actual, Some(json!(null)), "{lang}");
    }
    // Typed returns: PHP's empty body fails its `: array` return type, and Rust's stub is
    // `todo!()`. Both compile and run.
    let r = run(Language::Php, &starter_code(Language::Php, &sig).unwrap());
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(failure_message(&r, MessageCode::Exception).contains("TypeError"));
    let r = run(Language::Rust, &starter_code(Language::Rust, &sig).unwrap());
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(failure_message(&r, MessageCode::Exception).contains("not yet implemented"));
}

// ---- Time Limit Exceeded ----------------------------------------------------------

#[test]
fn time_limit_exceeded_python() {
    let r = run(
        Language::Python,
        "def two_sum(nums, target):\n    if nums[0] == 7:\n        while True:\n            pass\n    return [2, 4]\n",
    );
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
    // Public 1 passes, public 2 ([7, -3, 10, 5]) spins; later tests never run.
    assert_eq!(r.tests.len(), 2);
    assert_eq!(r.tests[0].result.verdict, Verdict::Accepted);
    let f = r.first_failure.clone().unwrap();
    assert_eq!(
        (f.kind, f.index, f.verdict),
        (TestKind::Public, 2, Verdict::TimeLimitExceeded)
    );
    assert_eq!(f.input.get("nums"), Some(&json!([7, -3, 10, 5])));
    assert_eq!(r.tests[1].result.time_us, 2_000_000);
    assert_eq!(
        failure_message(&r, MessageCode::TimeLimit),
        "time limit of 2000 ms exceeded"
    );
}

#[test]
fn time_limit_exceeded_typescript() {
    let r = run(
        Language::TypeScript,
        "export function twoSum(nums: number[], target: number): number[] {\n  for (;;) {}\n}\n",
    );
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
    assert_eq!(r.tests.len(), 1);
}

// ---- Runtime Error ----------------------------------------------------------------

#[test]
fn runtime_error_python_shows_user_traceback_only() {
    let src = "def helper(xs):\n    return xs[100]\n\ndef two_sum(nums, target):\n    return [helper(nums)]\n";
    let r = run(Language::Python, src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::Exception);
    assert!(msg.contains("IndexError"), "{msg}");
    assert!(
        msg.contains("File \"solution.py\", line 2, in helper"),
        "{msg}"
    );
    assert!(!msg.contains("main.py"), "harness frames leak: {msg}");
}

#[test]
fn runtime_error_typescript() {
    let src = "export function twoSum(nums: number[], target: number): number[] {\n  \
               throw new RangeError(\"nope\");\n}\n";
    let r = run(Language::TypeScript, src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::Exception);
    assert!(msg.starts_with("RangeError: nope"), "{msg}");
    assert!(msg.contains("at twoSum (solution.ts:2:"), "{msg}");
    assert!(
        !msg.contains("main.ts") && !msg.contains("file:///"),
        "{msg}"
    );
}

#[test]
fn runtime_error_when_process_exits_mid_test() {
    let src = "export function twoSum(nums: number[], target: number): number[] {\n  \
               process.exit(3);\n}\n";
    let r = run(Language::TypeScript, src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(failure_message(&r, MessageCode::ProcessExited).contains("exit code 0x3"));
}

#[test]
fn runtime_error_while_loading_module() {
    let r = run(Language::Python, "raise ValueError('at import')\n");
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(failure_message(&r, MessageCode::LoadError).contains("ValueError: at import"));
}

#[test]
fn runtime_error_when_return_value_is_not_json() {
    let r = run(
        Language::Python,
        "def two_sum(nums, target):\n    return {0, 1}\n",
    );
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::ReturnNotSerializable);
    assert!(msg.contains("set"), "{msg}");

    let src = "export function twoSum(nums: number[], target: number): number[] {\n  \
               return [0n, 1n] as unknown as number[];\n}\n";
    let r = run(Language::TypeScript, src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::ReturnNotSerializable);
    assert!(msg.contains("BigInt"), "{msg}");
}

// ---- Compilation Error ------------------------------------------------------------

#[test]
fn compilation_error_typescript_syntax() {
    let src =
        "export function twoSum(nums: number[], target: number): number[] {\n  return [0, 1\n}\n";
    let r = run(Language::TypeScript, src);
    assert_eq!(r.verdict, Verdict::CompilationError);
    assert!(r.tests.is_empty());
    assert!(r.first_failure.is_none());
    let out = compile_output(&r, MessageCode::SyntaxError);
    assert!(out.contains("ERR_INVALID_TYPESCRIPT_SYNTAX"), "{out}");
    assert!(out.contains("solution.ts:3"), "{out}");
    assert!(!out.contains("file:///"), "{out}");
}

#[test]
fn compilation_error_typescript_unsupported_syntax() {
    let src = "enum Dir { Up }\nexport function twoSum(nums: number[], target: number): number[] {\n  return [];\n}\n";
    let r = run(Language::TypeScript, src);
    assert_eq!(r.verdict, Verdict::CompilationError);
    assert!(
        compile_output(&r, MessageCode::UnsupportedSyntax)
            .contains("ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX")
    );
}

#[test]
fn compilation_error_python_syntax_and_missing_function() {
    let r = run(
        Language::Python,
        "def two_sum(nums, target)\n    return []\n",
    );
    assert_eq!(r.verdict, Verdict::CompilationError);
    assert!(compile_output(&r, MessageCode::SyntaxError).contains("SyntaxError"));

    let r = run(Language::Python, "def something_else():\n    pass\n");
    assert_eq!(r.verdict, Verdict::CompilationError);
    assert!(compile_output(&r, MessageCode::FunctionNotFound).contains("two_sum() is not defined"));

    let src =
        "export function twoSumm(nums: number[], target: number): number[] {\n  return [];\n}\n";
    let r = run(Language::TypeScript, src);
    assert!(compile_output(&r, MessageCode::FunctionNotFound).contains("twoSum() is not exported"));
}

#[test]
fn json_parse_error_at_load_is_a_runtime_error_not_compilation() {
    let src = "JSON.parse(\"{\");\nexport function twoSum(nums: number[], target: number): number[] {\n  return [];\n}\n";
    assert_eq!(
        run(Language::TypeScript, src).verdict,
        Verdict::RuntimeError
    );
}

// ---- User output never mixes with results ----------------------------------------

#[test]
fn user_prints_are_captured_per_test_python() {
    let src = r#"import os, sys
print("loading")
def two_sum(nums, target):
    print("nums =", nums)
    print('{"kind": "done"}')
    print("err line", file=sys.stderr)
    os.write(1, b"raw-without-newline")
    seen = {}
    for i, n in enumerate(nums):
        if target - n in seen:
            return [seen[target - n], i]
        seen[n] = i
"#;
    let r = run_small(Language::Python, src);
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.tests.len(), small_tests().len());
    assert_eq!(
        r.tests[0].stdout,
        "nums = [5, 8, 12, 1, 3]\n{\"kind\": \"done\"}\nerr line\n"
    );
}

#[test]
fn user_prints_are_captured_per_test_typescript() {
    let src = r#"import { writeSync } from "node:fs";
console.log("loading");
export function twoSum(nums: number[], target: number): number[] {
  console.log("nums =", nums);
  console.error('{"kind":"done"}');
  writeSync(1, "raw-without-newline");
  return target === 15 ? [2, 4] : [1, 0];
}
"#;
    let r = run(Language::TypeScript, src);
    assert_eq!(
        r.tests[0].stdout,
        "nums = [ 5, 8, 12, 1, 3 ]\n{\"kind\":\"done\"}\n"
    );
    assert_eq!(r.tests[0].result.verdict, Verdict::Accepted);
    let f = r.first_failure.unwrap();
    assert_eq!(f.verdict, Verdict::WrongAnswer);
    assert_eq!(f.stdout, "nums = [ 7, -3, 10, 5 ]\n{\"kind\":\"done\"}\n");
}

#[test]
fn printing_too_much_is_output_limit_exceeded() {
    let r = run(
        Language::Python,
        "def two_sum(nums, target):\n    print('x' * 100_000)\n    return [0, 1]\n",
    );
    assert_eq!(r.verdict, Verdict::OutputLimitExceeded);
    assert_eq!(r.tests[0].stdout.len(), 64 * 1024);
    assert_eq!(
        r.tests[0].result.message.as_ref().unwrap().code,
        MessageCode::TooMuchOutput
    );
}

#[test]
fn public_only_run() {
    let p = two_sum();
    let public: Vec<_> = p.public_tests().cloned().collect();
    let r = judge(
        &p,
        &public,
        Language::Python,
        &reference(Language::Python),
        &env(),
    )
    .unwrap();
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.total_tests, 3);
}

// ---- PHP --------------------------------------------------------------------------

const PHP_HEAD: &str = "<?php\nfunction twoSum(array $nums, int $target): array {\n";

#[test]
fn php_accepted_reference() {
    let r = run(Language::Php, &reference(Language::Php));
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.tests.len(), two_sum().tests.len());
    assert!(r.compile.is_none());
}

#[test]
fn php_wrong_answer() {
    let r = run(
        Language::Php,
        &format!("{PHP_HEAD}    return [2, 4];\n}}\n"),
    );
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    let f = r.first_failure.unwrap();
    assert_eq!((f.kind, f.index), (TestKind::Public, 2));
    assert_eq!(f.actual, Some(json!([2, 4])));
}

#[test]
fn php_list_keys_are_normalized() {
    // array_filter keeps keys; without re-indexing this would encode as an object.
    let src =
        format!("{PHP_HEAD}    return array_filter([3 => 0, 7 => 1], fn ($x) => true);\n}}\n");
    let r = run(Language::Php, &src);
    assert_eq!(r.tests[0].actual, Some(json!([0, 1])));
}

#[test]
fn php_time_limit_exceeded() {
    let r = run(
        Language::Php,
        &format!("{PHP_HEAD}    while (true) {{}}\n}}\n"),
    );
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
    assert_eq!(r.tests.len(), 1);
}

#[test]
fn php_runtime_error() {
    let src = "<?php\nfunction helper(array $a): int {\n    throw new LogicException('nope');\n}\n\
               function twoSum(array $nums, int $target): array {\n    return [helper($nums)];\n}\n";
    let r = run(Language::Php, src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::Exception);
    assert!(
        msg.starts_with("LogicException: nope in solution.php:3"),
        "{msg}"
    );
    assert!(msg.contains("at helper() (solution.php:6)"), "{msg}");
    assert!(!msg.contains("main.php"), "{msg}");
}

#[test]
fn php_memory_limit_exceeded() {
    let src =
        format!("{PHP_HEAD}    $s = str_repeat('x', 512 * 1024 * 1024);\n    return [0, 1];\n}}\n");
    let r = run(Language::Php, &src);
    assert_eq!(r.verdict, Verdict::MemoryLimitExceeded);
    assert_eq!(
        failure_message(&r, MessageCode::MemoryLimit),
        "memory limit exceeded"
    );
}

#[test]
fn php_compilation_errors() {
    let r = run(Language::Php, &format!("{PHP_HEAD}    return [0, 1\n}}\n"));
    assert_eq!(r.verdict, Verdict::CompilationError);
    let out = compile_output(&r, MessageCode::SyntaxError);
    assert!(out.contains("solution.php on line"), "{out}");

    // A fatal compile error PHP cannot throw as an exception.
    let src = format!(
        "<?php\nfunction count($x) {{ return 1; }}\n{}",
        &PHP_HEAD[6..]
    );
    let r = run(Language::Php, &format!("{src}    return [0, 1];\n}}\n"));
    assert!(compile_output(&r, MessageCode::CompileError).contains("Cannot redeclare"));

    let r = run(Language::Php, "<?php\nfunction twosum2() { return []; }\n");
    assert!(compile_output(&r, MessageCode::FunctionNotFound).contains("twoSum() is not defined"));
}

#[test]
fn php_return_value_not_json() {
    let r = run(Language::Php, &format!("{PHP_HEAD}    return [NAN];\n}}\n"));
    assert_eq!(r.verdict, Verdict::RuntimeError);
    failure_message(&r, MessageCode::ReturnNotSerializable);
}

#[test]
fn php_prints_and_warnings_are_captured_per_test() {
    let src = format!(
        "{PHP_HEAD}    echo 'nums = ', json_encode($nums), \"\\n\";\n    echo '{{\"kind\":\"done\"}}', \"\\n\";\n    \
         echo $undefined;\n    return $target === 15 ? [2, 4] : [];\n}}\n"
    );
    let r = run_small(Language::Php, &src);
    assert_eq!(r.tests[0].result.verdict, Verdict::Accepted);
    assert_eq!(
        r.tests[0].stdout,
        "nums = [5,8,12,1,3]\n{\"kind\":\"done\"}\nWarning: Undefined variable $undefined in solution.php on line 5\n"
    );
    assert_eq!(
        r.tests.len(),
        small_tests().len(),
        "a fake record must not end the run"
    );
}

#[test]
fn php_mbstring_is_available() {
    let src = format!("{PHP_HEAD}    return [mb_strlen('ñandú'), 1];\n}}\n");
    let r = run(Language::Php, &src);
    assert_eq!(r.tests[0].actual, Some(json!([5, 1])));
}

// ---- Rust -------------------------------------------------------------------------

const RS_HEAD: &str = "pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {\n";

/// A source that has never been compiled, so the cache cannot answer for it.
fn fresh(src: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("// fresh {nanos} {:?}\n{src}", std::thread::current().id())
}

#[test]
fn rust_accepted_reference_then_cached() {
    let src = fresh(&reference(Language::Rust));
    let r = run(Language::Rust, &src);
    assert_eq!(r.verdict, Verdict::Accepted);
    assert_eq!(r.tests.len(), two_sum().tests.len());
    let c = r.compile.unwrap();
    assert!(!c.cached && c.time_us > 0, "{c:?}");

    let again = run(Language::Rust, &src);
    assert_eq!(again.verdict, Verdict::Accepted);
    assert!(again.compile.unwrap().cached);
}

#[test]
fn rust_wrong_answer() {
    let r = run(Language::Rust, &format!("{RS_HEAD}    vec![2, 4]\n}}\n"));
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    let f = r.first_failure.unwrap();
    assert_eq!((f.kind, f.index), (TestKind::Public, 2));
    assert_eq!(f.actual, Some(json!([2, 4])));
}

#[test]
fn rust_time_limit_exceeded() {
    let r = run(Language::Rust, &format!("{RS_HEAD}    loop {{}}\n}}\n"));
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
    assert_eq!(r.tests.len(), 1);
}

#[test]
fn rust_panic_is_a_runtime_error() {
    let src = format!("{RS_HEAD}    let v: Vec<i64> = Vec::new();\n    vec![v[nums.len()]]\n}}\n");
    let r = run(Language::Rust, &src);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let msg = failure_message(&r, MessageCode::Exception);
    assert!(msg.starts_with("panicked at solution.rs:3:"), "{msg}");
    assert!(msg.contains("index out of bounds"), "{msg}");
}

#[test]
fn rust_process_exit_mid_test() {
    let r = run(
        Language::Rust,
        &format!("{RS_HEAD}    std::process::exit(3)\n}}\n"),
    );
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(failure_message(&r, MessageCode::ProcessExited).contains("exit code 0x3"));
}

#[test]
fn rust_compilation_errors() {
    let r = run(
        Language::Rust,
        &format!("{RS_HEAD}    let x: i64 = \"nope\";\n    vec![x]\n}}\n"),
    );
    assert_eq!(r.verdict, Verdict::CompilationError);
    assert!(r.tests.is_empty());
    assert!(r.compile.is_some());
    let out = compile_output(&r, MessageCode::CompileError);
    assert!(out.contains("error[E0308]: mismatched types"), "{out}");
    assert!(out.contains("--> solution.rs:2:"), "{out}");
    assert!(!out.contains("warning"), "warnings are silenced: {out}");
    assert!(!out.contains(":\\"), "absolute paths leak: {out}");

    let r = run(Language::Rust, "pub fn twosum() -> Vec<i64> { vec![] }\n");
    assert_eq!(r.verdict, Verdict::CompilationError);
    compile_output(&r, MessageCode::FunctionNotFound);
}

#[test]
fn rust_prints_are_captured_per_test() {
    let src = format!(
        "{RS_HEAD}    println!(\"nums = {{:?}}\", nums);\n    println!(\"{{{{\\\"kind\\\":\\\"done\\\"}}}}\");\n    \
         eprint!(\"no newline\");\n    if target == 15 {{ vec![2, 4] }} else {{ vec![] }}\n}}\n"
    );
    let r = run_small(Language::Rust, &src);
    assert_eq!(
        r.tests.len(),
        small_tests().len(),
        "a fake record must not end the run"
    );
    assert_eq!(r.tests[0].result.verdict, Verdict::Accepted);
    assert_eq!(
        r.tests[0].stdout,
        "nums = [5, 8, 12, 1, 3]\n{\"kind\":\"done\"}\nno newline"
    );
    let f = r.first_failure.unwrap();
    assert_eq!(
        f.stdout,
        "nums = [7, -3, 10, 5]\n{\"kind\":\"done\"}\nno newline"
    );
}

#[test]
fn rust_printing_too_much_is_output_limit_exceeded() {
    let src =
        format!("{RS_HEAD}    print!(\"{{}}\", \"x\".repeat(100_000));\n    vec![0, 1]\n}}\n");
    let r = run(Language::Rust, &src);
    assert_eq!(r.verdict, Verdict::OutputLimitExceeded);
    assert_eq!(r.tests[0].stdout.len(), 64 * 1024);
}

// --- Problem tooling in the sandbox (ADR 0016) ------------------------------------------

const GEN: &str = r#"import random

def generate(seed, size):
    rng = random.Random(seed)
    n = 5 if size == "small" else 50
    return {"nums": [rng.randint(-9, 9) for _ in range(n)], "target": rng.randint(-9, 9)}
"#;

#[test]
fn gen_py_runs_in_the_sandbox_deterministically() {
    let env = env();
    let a = kb_judge::generate_inputs(&env, GEN, &[1, 2, 3], "small", Default::default()).unwrap();
    let b = kb_judge::generate_inputs(&env, GEN, &[1, 2, 3], "small", Default::default()).unwrap();
    assert_eq!(a, b, "same seeds, same inputs");
    assert_eq!(a.len(), 3);
    assert_eq!(a[0]["nums"].as_array().unwrap().len(), 5);
    let max = kb_judge::generate_inputs(&env, GEN, &[7], "max", Default::default()).unwrap();
    assert_eq!(max[0]["nums"].as_array().unwrap().len(), 50);
}

#[test]
fn validate_py_reports_broken_constraints() {
    let env = env();
    let src = "def validate(inp):\n    if len(inp['nums']) < 2:\n        return '2 \u{2264} nums.length'\n    return None\n";
    let ok = json!({"nums": [1, 2], "target": 3});
    let bad = json!({"nums": [1], "target": 3});
    let r = kb_judge::validate_inputs(
        &env,
        src,
        &[ok.as_object().unwrap(), bad.as_object().unwrap()],
        Default::default(),
    )
    .unwrap();
    assert_eq!(r, [None, Some("2 \u{2264} nums.length".to_string())]);
}

#[test]
fn script_errors_and_timeouts_are_reported() {
    let env = env();
    let err = kb_judge::generate_inputs(
        &env,
        "def generate(seed, size):\n    raise ValueError('boom')\n",
        &[1],
        "small",
        Default::default(),
    )
    .unwrap_err()
    .to_string();
    assert!(
        err.contains("gen.py") && err.contains("ValueError: boom"),
        "{err}"
    );
    let limits = kb_judge::ScriptLimits {
        cpu_time_ms: 1000,
        ..Default::default()
    };
    let err = kb_judge::generate_inputs(
        &env,
        "def generate(seed, size):\n    while True: pass\n",
        &[1],
        "small",
        limits,
    )
    .unwrap_err()
    .to_string();
    assert!(err.contains("ran out of time"), "{err}");
}

/// Scripts get the same sandbox as user code: nothing outside their work folder and the
/// Python toolchain.
#[test]
fn scripts_cannot_read_or_write_outside_the_sandbox() {
    let env = env();
    let secret = repo().join("Cargo.toml");
    let read = format!(
        "def generate(seed, size):\n    open({:?}).read()\n    return {{}}\n",
        secret.display().to_string()
    );
    let err = kb_judge::generate_inputs(&env, &read, &[1], "small", Default::default())
        .unwrap_err()
        .to_string();
    assert!(err.contains("PermissionError"), "{err}");
    let target = repo().join("target").join("kb-judge-escape.txt");
    let write = format!(
        "def generate(seed, size):\n    open({:?}, 'w').write('x')\n    return {{}}\n",
        target.display().to_string()
    );
    let err = kb_judge::generate_inputs(&env, &write, &[1], "small", Default::default())
        .unwrap_err()
        .to_string();
    assert!(err.contains("PermissionError"), "{err}");
    assert!(!target.exists());
}
