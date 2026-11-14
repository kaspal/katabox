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
    /// The visualizer's structured snapshot, when it was asked for (ADR 0027).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<DebugSnapshot>,
}

/// A bounded, structured copy of the variables the visualizer asked for, taken at the
/// stop (ADR 0027). Objects are numbered within one snapshot; a shared node or a cycle
/// is a second `ref` to the same id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DebugSnapshot {
    /// The requested variables (that exist at the stop), then every other local holding
    /// a ListNode / TreeNode.
    #[serde(default)]
    pub roots: Vec<SnapRoot>,
    /// Indexed by [`SnapValue::Ref`] ids.
    #[serde(default)]
    pub objects: Vec<SnapObject>,
    /// A cap was hit: some containers list fewer items than their `len`, or some
    /// values were not followed.
    #[serde(default)]
    pub truncated: bool,
    /// Why there is no snapshot: `too_large`, `invalid`, or the harness's exception.
    #[serde(default)]
    pub error: Option<String>,
}

impl DebugSnapshot {
    /// An empty snapshot that only says why.
    pub fn failed(error: &str) -> DebugSnapshot {
        DebugSnapshot {
            roots: vec![],
            objects: vec![],
            truncated: true,
            error: Some(error.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SnapRoot {
    pub name: String,
    pub value: SnapValue,
}

/// A value in a snapshot: a reference to an object, or a scalar as display text (so big
/// integers never lose digits), or an object the snapshot does not look into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SnapValue {
    Ref {
        id: u32,
    },
    Int {
        v: String,
    },
    Float {
        v: String,
    },
    Str {
        v: String,
    },
    Bool {
        v: String,
    },
    None {
        v: String,
    },
    /// Any other object: its type name only.
    Opaque {
        v: String,
    },
}

/// A container or a node. `len` is the real length; `items` / `entries` may list fewer
/// (the snapshot's caps).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SnapObject {
    List {
        #[serde(rename = "type")]
        type_name: String,
        items: Vec<SnapValue>,
        len: u32,
        /// A typed number array (Python `array.array`, a JavaScript typed array).
        #[serde(default)]
        typed: bool,
    },
    Map {
        #[serde(rename = "type")]
        type_name: String,
        entries: Vec<(SnapValue, SnapValue)>,
        len: u32,
    },
    Set {
        #[serde(rename = "type")]
        type_name: String,
        items: Vec<SnapValue>,
        len: u32,
    },
    ListNode {
        val: SnapValue,
        next: SnapValue,
        /// The node's number among the input's nodes (stable across steps), if the
        /// harness built it.
        #[serde(default)]
        input: Option<u32>,
    },
    TreeNode {
        val: SnapValue,
        left: SnapValue,
        right: SnapValue,
        #[serde(default)]
        input: Option<u32>,
    },
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

    #[test]
    fn snapshots_decode_refs_scalars_and_nodes() {
        // Two roots share node 0 (identity); node 1 points back to node 0 (a cycle).
        let raw = serde_json::json!({
            "roots": [
                {"name": "slow", "value": {"t": "ref", "id": 0}},
                {"name": "fast", "value": {"t": "ref", "id": 0}},
                {"name": "i", "value": {"t": "int", "v": "123456789012345678901234567890"}}
            ],
            "objects": [
                {"kind": "list_node", "val": {"t": "int", "v": "1"}, "next": {"t": "ref", "id": 1}, "input": 0},
                {"kind": "list_node", "val": {"t": "str", "v": "x"}, "next": {"t": "ref", "id": 0}},
                {"kind": "map", "type": "dict", "entries": [[{"t": "str", "v": "k"}, {"t": "none", "v": "None"}]], "len": 3}
            ],
            "truncated": true
        });
        let s: DebugSnapshot = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(s.roots[0].value, s.roots[1].value);
        assert_eq!(
            s.roots[2].value,
            SnapValue::Int {
                v: "123456789012345678901234567890".into()
            }
        );
        let SnapObject::ListNode { next, input, .. } = &s.objects[1] else {
            panic!("{:?}", s.objects[1]);
        };
        assert_eq!((next, *input), (&SnapValue::Ref { id: 0 }, None));
        assert!(matches!(
            s.objects[0],
            SnapObject::ListNode { input: Some(0), .. }
        ));
        assert!(
            matches!(&s.objects[2], SnapObject::Map { len: 3, entries, .. } if entries.len() == 1)
        );
        assert!(s.truncated && s.error.is_none());
        // Round trip, and an unknown kind is an error (kb-judge turns it into `invalid`).
        assert_eq!(
            serde_json::from_value::<DebugSnapshot>(serde_json::to_value(&s).unwrap()).unwrap(),
            s
        );
        let bad = serde_json::json!({"roots": [], "objects": [{"kind": "blob"}]});
        assert!(serde_json::from_value::<DebugSnapshot>(bad).is_err());
        assert_eq!(
            DebugSnapshot::failed("too_large").error.as_deref(),
            Some("too_large")
        );
    }
}
