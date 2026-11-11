//! The one error type every command returns: a stable code the UI translates, plus
//! English text for logs and as a fallback (the same rule as judge messages).

use kb_judge::JudgeError;
use kb_store::StoreError;
use serde::{Deserialize, Serialize};

/// Stable, machine-readable error code. Serialized in snake_case: **never rename a
/// variant**, only add new ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum AppErrorCode {
    /// No such problem, submission, snapshot or file.
    NotFound,
    /// A command argument is malformed.
    InvalidInput,
    /// A custom test input does not fit the signature, or the reference solution could
    /// not handle it.
    InvalidCustomInput,
    /// The judge does not support this language (yet).
    UnsupportedLanguage,
    /// The language's runtime is not installed in the sandbox root.
    ToolchainMissing,
    /// A Run or Submit is already in progress.
    JudgeBusy,
    /// The sandbox refused or failed to run the code.
    Sandbox,
    /// The judge failed (never the user's fault).
    JudgeInternal,
    /// A database error (content.db or user.db).
    Store,
    /// No language server for this language here (not installed or not supported).
    LanguageServerUnavailable,
    /// The language server is not running (stopped or restarting); connect again.
    LanguageServerNotConnected,
    /// Anything else, including the app failing to start.
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[error("{code:?}: {message}")]
pub struct AppError {
    pub code: AppErrorCode,
    /// English text: for logs, the developer console and as a fallback.
    pub message: String,
}

impl AppError {
    pub fn new(code: AppErrorCode, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
        }
    }
}

impl From<StoreError> for AppError {
    fn from(e: StoreError) -> Self {
        let code = match e {
            StoreError::NotFound(_) => AppErrorCode::NotFound,
            _ => AppErrorCode::Store,
        };
        AppError::new(code, e.to_string())
    }
}

impl From<JudgeError> for AppError {
    fn from(e: JudgeError) -> Self {
        let code = match e {
            JudgeError::Unsupported(_) => AppErrorCode::UnsupportedLanguage,
            JudgeError::Toolchain(_) => AppErrorCode::ToolchainMissing,
            JudgeError::Sandbox(_) => AppErrorCode::Sandbox,
            JudgeError::Problem(_) | JudgeError::Internal(_) | JudgeError::Io(_) => {
                AppErrorCode::JudgeInternal
            }
        };
        AppError::new(code, e.to_string())
    }
}

impl From<kb_bridge::BridgeError> for AppError {
    fn from(e: kb_bridge::BridgeError) -> Self {
        use kb_bridge::BridgeError as B;
        let code = match e {
            B::InvalidProblem(_) => AppErrorCode::InvalidInput,
            B::Unavailable(_) => AppErrorCode::LanguageServerUnavailable,
            B::NotConnected(_) => AppErrorCode::LanguageServerNotConnected,
            B::Io(_) => AppErrorCode::Internal,
        };
        AppError::new(code, e.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(e: tokio::task::JoinError) -> Self {
        AppError::new(AppErrorCode::Internal, format!("judge task failed: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::Language;

    #[test]
    fn codes_are_stable_snake_case() {
        let e = AppError::new(AppErrorCode::InvalidCustomInput, "x");
        assert_eq!(
            serde_json::to_value(&e).unwrap(),
            serde_json::json!({"code": "invalid_custom_input", "message": "x"})
        );
    }

    #[test]
    fn maps_store_and_judge_errors() {
        let nf: AppError = StoreError::NotFound("problem x".into()).into();
        assert_eq!(nf.code, AppErrorCode::NotFound);
        assert!(nf.message.contains("problem x"));
        let cases = [
            (
                JudgeError::Unsupported(Language::JavaScript),
                AppErrorCode::UnsupportedLanguage,
            ),
            (
                JudgeError::Toolchain("no".into()),
                AppErrorCode::ToolchainMissing,
            ),
            (
                JudgeError::Internal("x".into()),
                AppErrorCode::JudgeInternal,
            ),
        ];
        for (e, code) in cases {
            assert_eq!(AppError::from(e).code, code);
        }
    }
}
