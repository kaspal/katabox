//! Judging: compares a program's JSON output with the expected answer.
//!
//! Harness code generation (one minijinja template per language) lands here in
//! the Core judge phase. The checkers below are complete and tested.

pub mod checker;

pub use checker::{CheckOutcome, Checker};
