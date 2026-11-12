//! The debugger's data (ADR 0022): a recorded trace of one test, and what the debugger
//! shows at a stop. kb-judge produces them by replaying the test in the sandbox; kb-app
//! navigates the trace; the UI shows them.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One step: the user's file is about to run `line`, `depth` frames deep (1 = the
/// solution function itself).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Step {
    pub line: u32,
    pub depth: u32,
}

/// Why a trace stopped before the program ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Truncation {
    /// The step cap.
    Steps,
    /// The trace's byte budget.
    Bytes,
    /// Too many distinct (line, depth) pairs (very deep recursion).
    Codes,
}

/// Prints made while the step `step` ran (or before the first step).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PrintChunk {
    pub step: u32,
    pub text: String,
}

/// How the traced call ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum OutcomeStatus {
    Ok,
    Error,
    Memory,
}

/// The traced call's result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DebugOutcome {
    pub status: OutcomeStatus,
    /// The returned value (`ok`).
    #[serde(default)]
    pub value: Value,
    /// The exception and its traceback (`error`).
    #[serde(default)]
    pub error: Option<String>,
    /// The line of the user's file that raised (`error`).
    #[serde(default)]
    pub line: Option<u32>,
    /// The last step that ran.
    pub step: u32,
}

/// One test, recorded step by step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Trace {
    pub steps: Vec<Step>,
    /// Set when recording stopped early; the recorded steps are still usable.
    pub truncated: Option<Truncation>,
    pub prints: Vec<PrintChunk>,
    pub prints_truncated: bool,
    /// How the call ended (`None` when truncated).
    pub outcome: Option<DebugOutcome>,
}

impl Trace {
    /// FNV-1a over (line, depth) of steps `0..=step`: the value a replay to `step` must
    /// report (the debug harness computes the same).
    pub fn prefix_hash(&self, step: usize) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for s in self.steps.iter().take(step + 1) {
            for v in [u64::from(s.line), u64::from(s.depth)] {
                h ^= v;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        h
    }
}

/// A variable or a child of one, as the debugger lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VarInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    /// `repr`, truncated.
    pub value: String,
    /// How many children it has (0 = not expandable).
    pub children: u32,
}

/// The children of the value at `path` = `[scope, name, i, j, …]`, where `scope` is
/// `"locals"` or `"globals"` and each `i` is a position among the parent's children.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Expansion {
    pub path: Vec<Value>,
    pub children: Vec<VarInfo>,
    /// Children not listed (beyond the first 100).
    pub more: u32,
}

/// A watch or console expression evaluated at the stop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EvalResult {
    pub expr: String,
    pub ok: bool,
    /// The value (`None` for a statement).
    #[serde(default)]
    pub value: Option<VarInfo>,
    #[serde(default)]
    pub error: Option<String>,
    /// What it printed.
    #[serde(default)]
    pub stdout: String,
}

/// One frame of the user's call stack, innermost first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StackFrame {
    pub function: String,
    pub line: u32,
}

/// What the debugger shows at one step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StopInfo {
    pub step: u32,
    pub stack: Vec<StackFrame>,
    /// The selected frame (index into `stack`).
    pub frame: u32,
    pub locals: Vec<VarInfo>,
    pub globals: Vec<VarInfo>,
    pub expanded: Vec<Expansion>,
    pub evals: Vec<EvalResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_hash_matches_the_python_harness() {
        // The value the harness computes for steps (3, 1), (4, 1): FNV-1a 64 over
        // 3, 1, 4, 1 as whole integers.
        let t = Trace {
            steps: vec![Step { line: 3, depth: 1 }, Step { line: 4, depth: 1 }],
            truncated: None,
            prints: vec![],
            prints_truncated: false,
            outcome: None,
        };
        let mut h: u64 = 0xcbf29ce484222325;
        for v in [3u64, 1, 4, 1] {
            h = (h ^ v).wrapping_mul(0x100000001b3);
        }
        assert_eq!(t.prefix_hash(1), h);
        assert_ne!(t.prefix_hash(0), h);
    }
}
