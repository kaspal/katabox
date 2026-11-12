//! Complexity profiler types (ADR 0024): measured points and the estimated class.

use serde::{Deserialize, Serialize};

/// The growth classes the profiler can name, simplest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ComplexityClass {
    /// O(1)
    Constant,
    /// O(log n)
    Logarithmic,
    /// O(n)
    Linear,
    /// O(n log n)
    Linearithmic,
    /// O(n²)
    Quadratic,
    /// O(2ⁿ)
    Exponential,
}

impl ComplexityClass {
    pub const ALL: [ComplexityClass; 6] = [
        ComplexityClass::Constant,
        ComplexityClass::Logarithmic,
        ComplexityClass::Linear,
        ComplexityClass::Linearithmic,
        ComplexityClass::Quadratic,
        ComplexityClass::Exponential,
    ];

    /// The usual notation, e.g. `O(n log n)`.
    pub fn notation(self) -> &'static str {
        match self {
            ComplexityClass::Constant => "O(1)",
            ComplexityClass::Logarithmic => "O(log n)",
            ComplexityClass::Linear => "O(n)",
            ComplexityClass::Linearithmic => "O(n log n)",
            ComplexityClass::Quadratic => "O(n²)",
            ComplexityClass::Exponential => "O(2ⁿ)",
        }
    }
}

/// One measured size: the median of the repetitions, with every sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProfilePoint {
    pub n: u32,
    /// Median wall time of the call, measured inside the harness (microseconds).
    pub median_us: u32,
    pub samples_us: Vec<u32>,
}

/// The fit's answer. Always an estimate: the UI never shows it without saying so.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Estimate {
    pub class: ComplexityClass,
    /// 0 (a coin toss between the two best classes) to 1 (only one class fits).
    pub confidence: f64,
    /// The runner-up, when it fits almost as well: the UI shows both.
    pub close_alternative: Option<ComplexityClass>,
    /// The timings are mostly tiny (under a millisecond), where noise dominates.
    pub noisy: bool,
}

/// Why a profile stopped before its last size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ProfileStop {
    /// The next size would not fit in the time budget (predicted from the last points).
    Budget { n: u32 },
    /// A size ran out of time.
    TimeLimit { n: u32 },
    /// The solution failed at a size (runtime error, memory, output): no numbers from it.
    Failed { n: u32, verdict: crate::Verdict },
    /// The solution did not compile.
    Compile { output: String },
}

/// One profile run: the measured points in increasing n, and why it stopped early.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProfileRun {
    pub points: Vec<ProfilePoint>,
    pub stopped: Option<ProfileStop>,
}
