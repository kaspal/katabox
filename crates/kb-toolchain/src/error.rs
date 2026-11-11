use std::path::PathBuf;

/// Why an install, uninstall or download failed. [`Error::code`] is the stable code the
/// app shows (`errors.<code>`).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("download failed for {artifact}: {detail}")]
    Download { artifact: String, detail: String },
    #[error("cannot reach {host}: {detail}")]
    Network { host: String, detail: String },
    #[error("{url} is not an allowed download location")]
    HostNotAllowed { url: String },
    #[error("{artifact}: SHA-256 mismatch (expected {expected}, got {actual})")]
    Checksum {
        artifact: String,
        expected: String,
        actual: String,
    },
    #[error("{artifact}: size mismatch (expected {expected} bytes, got {actual})")]
    Size {
        artifact: String,
        expected: u64,
        actual: u64,
    },
    #[error("{artifact}: unsafe archive entry {entry:?}: {why}")]
    UnsafeArchive {
        artifact: String,
        entry: String,
        why: String,
    },
    #[error("not enough disk space: {needed} bytes needed, {available} available")]
    NotEnoughSpace { needed: u64, available: u64 },
    #[error("{} is in use; close programs using it and try again", path.display())]
    FileLocked { path: PathBuf },
    #[error("cancelled")]
    Cancelled,
    #[error("another install or uninstall is running")]
    Busy,
    #[error("{0} is not available on this platform")]
    UnsupportedPlatform(String),
    #[error("invalid sandbox root {}: {why}", path.display())]
    InvalidRoot { path: PathBuf, why: String },
    #[error("manifest: {0}")]
    Manifest(String),
    #[error("{context}: {source}")]
    Io {
        context: String,
        #[source]
        source: std::io::Error,
    },
}

/// Stable error codes (snake_case on the wire). Never rename one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    DownloadFailed,
    NetworkUnavailable,
    HostNotAllowed,
    ChecksumMismatch,
    SizeMismatch,
    UnsafeArchive,
    NotEnoughSpace,
    FileLocked,
    InstallCancelled,
    InstallBusy,
    UnsupportedPlatform,
    Internal,
}

impl Error {
    pub fn code(&self) -> ErrorCode {
        match self {
            Error::Download { .. } => ErrorCode::DownloadFailed,
            Error::Network { .. } => ErrorCode::NetworkUnavailable,
            Error::HostNotAllowed { .. } => ErrorCode::HostNotAllowed,
            Error::Checksum { .. } => ErrorCode::ChecksumMismatch,
            Error::Size { .. } => ErrorCode::SizeMismatch,
            Error::UnsafeArchive { .. } => ErrorCode::UnsafeArchive,
            Error::NotEnoughSpace { .. } => ErrorCode::NotEnoughSpace,
            Error::FileLocked { .. } => ErrorCode::FileLocked,
            Error::Cancelled => ErrorCode::InstallCancelled,
            Error::Busy => ErrorCode::InstallBusy,
            Error::UnsupportedPlatform(_) => ErrorCode::UnsupportedPlatform,
            Error::InvalidRoot { .. } | Error::Manifest(_) => ErrorCode::Internal,
            Error::Io { source, .. } if is_disk_full(source) => ErrorCode::NotEnoughSpace,
            Error::Io { .. } => ErrorCode::Internal,
        }
    }

    pub(crate) fn io(context: impl Into<String>, source: std::io::Error) -> Error {
        if is_disk_full(&source) {
            return Error::NotEnoughSpace {
                needed: 0,
                available: 0,
            };
        }
        Error::Io {
            context: context.into(),
            source,
        }
    }
}

fn is_disk_full(e: &std::io::Error) -> bool {
    // ERROR_DISK_FULL (112) / ERROR_HANDLE_DISK_FULL (39) on Windows, ENOSPC (28) on Linux.
    e.kind() == std::io::ErrorKind::StorageFull
        || matches!(e.raw_os_error(), Some(112 | 39) if cfg!(windows))
        || matches!(e.raw_os_error(), Some(28) if cfg!(unix))
}
