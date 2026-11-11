//! The judge's result types (produced by kb-judge, stored by kb-store). Plain data: the
//! serde shape is what user.db stores as a submission's report, so **never rename or
//! remove a field**; only add optional ones.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{Message, TestKind, TestResult, Verdict};

/// One test of a problem: arguments and expected return value.
#[derive(Debug, Clone, PartialEq)]
pub struct TestCase {
    pub kind: TestKind,
    /// 1-based position within its kind (public 1, 2, ...; hidden 1, 2, ...).
    pub index: u32,
    /// Argument values keyed by param name; always exactly the signature's params.
    pub input: Map<String, Value>,
    pub expected: Value,
}

/// Outcome of one test that ran (or was running when the batch stopped).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOutcome {
    #[serde(flatten)]
    pub result: TestResult,
    /// What the function returned, when it returned.
    pub actual: Option<Value>,
    /// The user's prints during this test (stdout and stderr, capped).
    pub stdout: String,
}

/// Details of the first test that did not pass. For hidden tests this is the only one
/// revealed, and only after submitting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Failure {
    pub kind: TestKind,
    pub index: u32,
    pub verdict: Verdict,
    pub input: Map<String, Value>,
    pub expected: Value,
    pub actual: Option<Value>,
    pub message: Option<Message>,
    pub stdout: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub verdict: Verdict,
    /// Tests that ran, in order. When the batch stopped early (time limit, crash) the
    /// last entry is the test that was running and later tests are absent.
    pub tests: Vec<TestOutcome>,
    pub total_tests: u32,
    pub first_failure: Option<Failure>,
    /// Compiler or parser output for `CompilationError`.
    pub compile_output: Option<Message>,
    /// Sum of the per-test times, in microseconds.
    pub time_us: u64,
    /// Peak memory of the whole run (capped at the limit).
    pub memory_kb: u32,
    /// The compile step, for compiled languages.
    pub compile: Option<CompileInfo>,
}

/// What the compile step did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileInfo {
    /// The binary came from the cache; nothing was compiled.
    pub cached: bool,
    /// Wall time of the compile in microseconds (0 when cached).
    pub time_us: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MessageCode;
    use serde_json::json;

    /// The stored JSON shape. Changing this breaks reports already in user.db.
    #[test]
    fn report_json_shape_is_stable() {
        let r = Report {
            verdict: Verdict::WrongAnswer,
            tests: vec![TestOutcome {
                result: TestResult {
                    index: 1,
                    kind: TestKind::Public,
                    verdict: Verdict::WrongAnswer,
                    time_us: 5,
                    memory_kb: 7,
                    message: Some(Message::new(MessageCode::OutputMismatch, "m")),
                },
                actual: Some(json!([1])),
                stdout: "p".into(),
            }],
            total_tests: 2,
            first_failure: Some(Failure {
                kind: TestKind::Public,
                index: 1,
                verdict: Verdict::WrongAnswer,
                input: json!({"a": 1}).as_object().unwrap().clone(),
                expected: json!([0]),
                actual: Some(json!([1])),
                message: None,
                stdout: String::new(),
            }),
            compile_output: None,
            time_us: 5,
            memory_kb: 7,
            compile: Some(CompileInfo {
                cached: true,
                time_us: 0,
            }),
        };
        let want = json!({
            "verdict": "wrong_answer",
            "tests": [{
                "index": 1, "kind": "public", "verdict": "wrong_answer", "time_us": 5,
                "memory_kb": 7, "message": {"code": "output_mismatch", "text": "m"},
                "actual": [1], "stdout": "p"
            }],
            "total_tests": 2,
            "first_failure": {
                "kind": "public", "index": 1, "verdict": "wrong_answer", "input": {"a": 1},
                "expected": [0], "actual": [1], "message": null, "stdout": ""
            },
            "compile_output": null,
            "time_us": 5,
            "memory_kb": 7,
            "compile": {"cached": true, "time_us": 0}
        });
        assert_eq!(serde_json::to_value(&r).unwrap(), want);
        let back: Report = serde_json::from_value(want).unwrap();
        assert_eq!(back, r);
    }
}
