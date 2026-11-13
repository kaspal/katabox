//! Judging for KataBox (ADR 0008).
//!
//! - [`codegen`]: starter stub and test harness from a problem's typed signature, rendered
//!   from the minijinja templates in `languages/<id>/`.
//! - [`judge()`]: runs a submission against a batch of tests in one sandboxed run
//!   (kb-runner), applies the problem's [`Checker`], and returns a [`Report`].
//! - [`checker`]: built-in output comparisons.
//! - [`tools`]: problem tooling (`gen.py`, `validate.py`) run in the sandbox like user code.

pub mod calibration;
pub mod checker;
pub mod codegen;
pub mod complexity;
pub mod debug;
mod judge;
pub mod languages;
pub mod limits;
pub mod problem;
pub mod profile;
pub mod program;
mod protocol;
pub mod signature;
pub mod tools;

pub use checker::{CheckOutcome, Checker};
pub use codegen::{harness, program_starter, signature_line, starter_code};
pub use judge::{BINARY_CACHE_SIZE, JudgeEnv, judge, judge_with_progress};
/// The judge's data types live in kb-core (so kb-store can store them without the judge);
/// re-exported here so this crate's API is unchanged.
pub use kb_core::{CompileInfo, Failure, JudgeEvent, Report, TestCase, TestOutcome};
pub use problem::{Problem, ProblemMode, check_input, program_signature};
pub use signature::{Param, Signature, Type};
pub use tools::{
    ScriptLimits, ScriptOutput, generate_inputs, generate_sized, run_python, run_script,
    validate_inputs,
};

use kb_core::Language;

#[derive(Debug, thiserror::Error)]
pub enum JudgeError {
    #[error("problem: {0}")]
    Problem(String),
    #[error("{0} is not supported by the judge yet")]
    Unsupported(Language),
    #[error("toolchain: {0}")]
    Toolchain(String),
    #[error("sandbox: {0}")]
    Sandbox(#[from] kb_runner::SandboxError),
    #[error("judge internal error: {0}")]
    Internal(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
