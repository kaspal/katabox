//! Domain types shared by every KataBox crate and by the UI (via tauri-specta).
//!
//! Keep this crate dependency-light: no I/O, no async, no platform code.

mod report;

pub use report::{CompileInfo, Failure, JudgeEvent, Report, TestCase, TestOutcome};

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Languages supported by the judge. Adding one means a `languages/<id>/` folder
/// (language.toml + harness template) and a variant here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Php,
    Rust,
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::Python,
        Language::JavaScript,
        Language::TypeScript,
        Language::Php,
        Language::Rust,
    ];

    /// Stable identifier used in file names and the database.
    pub fn id(self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Php => "php",
            Language::Rust => "rust",
        }
    }

    /// File extension for solution files.
    pub fn extension(self) -> &'static str {
        match self {
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::TypeScript => "ts",
            Language::Php => "php",
            Language::Rust => "rs",
        }
    }

    /// Whether a compile step runs before tests.
    pub fn is_compiled(self) -> bool {
        matches!(self, Language::Rust)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("unknown language: {0}")]
pub struct UnknownLanguage(pub String);

impl FromStr for Language {
    type Err = UnknownLanguage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::ALL
            .into_iter()
            .find(|l| l.id() == s || l.extension() == s)
            .ok_or_else(|| UnknownLanguage(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Final outcome of a run or submission. Names follow common judge conventions
/// (DMOJ / Judge0) so users recognize them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    OutputLimitExceeded,
    RuntimeError,
    CompilationError,
    InternalError,
}

impl Verdict {
    pub fn is_accepted(self) -> bool {
        self == Verdict::Accepted
    }

    /// Short code shown in compact UI (badges, history lists).
    pub fn code(self) -> &'static str {
        match self {
            Verdict::Accepted => "AC",
            Verdict::WrongAnswer => "WA",
            Verdict::TimeLimitExceeded => "TLE",
            Verdict::MemoryLimitExceeded => "MLE",
            Verdict::OutputLimitExceeded => "OLE",
            Verdict::RuntimeError => "RE",
            Verdict::CompilationError => "CE",
            Verdict::InternalError => "IE",
        }
    }
}

/// Which set a test case belongs to. Hidden tests are not listed before submit,
/// but the first failing one is revealed afterwards (see the plan).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TestKind {
    Example,
    Public,
    Custom,
    Hidden,
}

/// A language's time limit on one problem (ADR 0017): the problem's base limit × the
/// language's factor, at this machine's time scale. CPU time for the whole batch of tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LanguageLimit {
    pub language: Language,
    pub time_limit_ms: u32,
}

/// Resource limits for one test batch. The judge sets the CPU time from the problem's
/// base limit, the language and the machine's time scale (ADR 0017).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    pub cpu_time_ms: u32,
    pub memory_mb: u32,
    pub max_processes: u32,
    pub max_output_kb: u32,
}

impl Limits {
    /// Defaults from the plan: 2 s interpreted, 1 s compiled, 256 MB, 16 processes, 64 KB output.
    pub fn default_for(lang: Language) -> Self {
        Limits {
            cpu_time_ms: if lang.is_compiled() { 1_000 } else { 2_000 },
            memory_mb: 256,
            max_processes: 16,
            max_output_kb: 64,
        }
    }
}

/// Stable, machine-readable identifier of a judge message. The UI translates by code;
/// the English `text` next to it is for logs, the CLI and as a fallback. Serialized in
/// snake_case: **never rename a variant**, only add new ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum MessageCode {
    // Checker.
    /// The return value differs from the expected output.
    OutputMismatch,
    /// A custom checker was asked to run outside the sandbox.
    CustomCheckerUnsupported,
    // Compilation.
    /// The source does not parse (compiler or parser output in the text).
    SyntaxError,
    /// TypeScript syntax that type stripping cannot remove (`enum`, `namespace`, ...).
    UnsupportedSyntax,
    /// The solution does not define or export the required function.
    FunctionNotFound,
    /// The compiler rejected the code (compiler output in the text).
    CompileError,
    /// The compiler exceeded its own time, memory or process limits.
    CompileLimit,
    // Runtime, reported by the harness.
    /// The solution raised while its module was loading (traceback in the text).
    LoadError,
    /// The function raised (traceback in the text).
    Exception,
    /// The return value cannot be converted to JSON.
    ReturnNotSerializable,
    /// The language runtime reported running out of memory (e.g. Python `MemoryError`).
    OutOfMemory,
    /// The test printed more than the per-test output cap.
    TooMuchOutput,
    // Runtime, decided by the judge from the sandbox outcome.
    /// The batch hit its CPU or wall-clock limit during this test.
    TimeLimit,
    /// The batch hit the sandbox's total output limit during this test.
    OutputLimit,
    /// The batch hit the memory limit during this test.
    MemoryLimit,
    /// The process exited or crashed during this test (exit code in the text).
    ProcessExited,
    // Internal errors (never the user's fault).
    /// The harness stopped without reporting anything.
    HarnessSilent,
    /// The harness wrote a record the judge cannot read, or records out of order.
    BadHarnessRecord,
    /// The harness said it finished but some tests have no result.
    IncompleteResults,
}

/// A judge message: stable code plus English text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Message {
    pub code: MessageCode,
    pub text: String,
}

impl Message {
    pub fn new(code: MessageCode, text: impl Into<String>) -> Self {
        Message {
            code,
            text: text.into(),
        }
    }
}

/// Result of a single test case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TestResult {
    pub index: u32,
    pub kind: TestKind,
    pub verdict: Verdict,
    /// Time of this test in microseconds.
    pub time_us: u64,
    pub memory_kb: u32,
    /// Why the test did not pass, e.g. `output_mismatch` / "expected [0,1], got [1,2]".
    pub message: Option<Message>,
}

/// Aggregated outcome of a submission: the first non-accepted verdict wins.
pub fn overall_verdict(results: &[TestResult]) -> Verdict {
    results
        .iter()
        .map(|r| r.verdict)
        .find(|v| !v.is_accepted())
        .unwrap_or(Verdict::Accepted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_round_trips_through_id_and_extension() {
        for lang in Language::ALL {
            assert_eq!(lang.id().parse::<Language>(), Ok(lang));
            assert_eq!(lang.extension().parse::<Language>(), Ok(lang));
        }
        assert!("cobol".parse::<Language>().is_err());
    }

    #[test]
    fn verdict_serializes_as_snake_case() {
        let json = serde_json::to_string(&Verdict::TimeLimitExceeded).unwrap();
        assert_eq!(json, "\"time_limit_exceeded\"");
    }

    #[test]
    fn overall_verdict_is_first_failure() {
        let mk = |i, v| TestResult {
            index: i,
            kind: TestKind::Hidden,
            verdict: v,
            time_us: 1,
            memory_kb: 1,
            message: None,
        };
        assert_eq!(overall_verdict(&[]), Verdict::Accepted);
        let rs = [
            mk(0, Verdict::Accepted),
            mk(1, Verdict::WrongAnswer),
            mk(2, Verdict::TimeLimitExceeded),
        ];
        assert_eq!(overall_verdict(&rs), Verdict::WrongAnswer);
    }

    #[test]
    fn message_codes_serialize_as_stable_snake_case() {
        let m = Message::new(MessageCode::ReturnNotSerializable, "x");
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            r#"{"code":"return_not_serializable","text":"x"}"#
        );
        let c: MessageCode = serde_json::from_str("\"syntax_error\"").unwrap();
        assert_eq!(c, MessageCode::SyntaxError);
    }

    #[test]
    fn rust_gets_tighter_default_time() {
        assert_eq!(Limits::default_for(Language::Rust).cpu_time_ms, 1_000);
        assert_eq!(Limits::default_for(Language::Python).cpu_time_ms, 2_000);
    }
}
