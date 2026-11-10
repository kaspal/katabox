//! Sandboxed execution of user code.
//!
//! This is the only crate allowed to spawn user programs. Any change here needs
//! explicit maintainer sign-off (see CLAUDE.md).
//!
//! Layout:
//! - `windows/`: Job Object (memory, CPU, process count, kill-on-close) + AppContainer
//!   with zero capabilities (no network), ACL grants for toolchain and work folders.
//! - `linux/` (planned): hakoniwa (user/mount/PID/net namespaces, Landlock, seccomp,
//!   rlimits); fallback to Landlock + seccomp + rlimits when namespaces are unavailable.
//! - `pipeline` (planned): materialize workspace, compile if needed, run harness,
//!   stream results.

use std::path::PathBuf;
use std::time::Duration;

use kb_core::Limits;

mod cmdline;
#[cfg(windows)]
mod windows;

pub use cmdline::quote_windows_arg;

/// Name of the AppContainer profile every sandboxed process runs in.
pub const APP_CONTAINER_NAME: &str = "KataBox.Sandbox";

/// Everything needed to run one program inside the sandbox.
#[derive(Debug, Clone)]
pub struct SandboxSpec {
    /// Absolute path to the executable. Must live inside one of `read_only_dirs`.
    pub program: PathBuf,
    /// Arguments after the program name.
    pub args: Vec<String>,
    /// Current directory of the child. The sandbox gets read, write and execute here.
    pub work_dir: PathBuf,
    /// Folders the sandbox may read and execute from (e.g. the toolchain). Nothing else
    /// on disk is granted.
    pub read_only_dirs: Vec<PathBuf>,
    /// Extra environment variables. The child never inherits the host environment; it
    /// gets `SystemRoot`, `windir`, `TEMP`/`TMP` (= `work_dir`) plus these.
    pub env: Vec<(String, String)>,
    pub stdin: Vec<u8>,
    /// Total user-mode CPU time for the whole process tree.
    pub cpu_time: Duration,
    /// Wall-clock limit. Catches sleeping or blocked programs that use no CPU.
    pub wall_time: Duration,
    /// Committed memory for the whole process tree.
    pub memory_bytes: u64,
    /// Maximum simultaneously alive processes, including the first one. On Windows the
    /// console host (conhost.exe) runs in the job but is not counted against this.
    pub max_processes: u32,
    /// Combined stdout + stderr cap; exceeding it kills the tree.
    pub max_output_bytes: usize,
    /// Run as a Less Privileged AppContainer: also drops access granted to
    /// "ALL APPLICATION PACKAGES" (much of `C:\Windows` and `Program Files`).
    /// Off by default: python-build-standalone 3.14 dies at startup under LPAC with
    /// STATUS_ACCESS_DENIED (cause not yet found). See the attack suite for the gap.
    pub less_privileged: bool,
}

impl SandboxSpec {
    /// A spec with the given limits; wall time defaults to 3x the CPU limit.
    pub fn new(program: impl Into<PathBuf>, work_dir: impl Into<PathBuf>, limits: &Limits) -> Self {
        let cpu_time = Duration::from_millis(u64::from(limits.cpu_time_ms));
        SandboxSpec {
            program: program.into(),
            args: Vec::new(),
            work_dir: work_dir.into(),
            read_only_dirs: Vec::new(),
            env: Vec::new(),
            stdin: Vec::new(),
            cpu_time,
            wall_time: cpu_time * 3,
            memory_bytes: u64::from(limits.memory_mb) * 1024 * 1024,
            max_processes: limits.max_processes,
            max_output_bytes: limits.max_output_kb as usize * 1024,
            less_privileged: false,
        }
    }
}

/// Why the process tree stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Termination {
    /// Every process exited on its own.
    Exited,
    /// The job's CPU time limit fired and the OS killed the tree.
    CpuTimeLimit,
    /// The runner killed the tree at the wall-clock deadline.
    WallTimeLimit,
    /// The runner killed the tree because output exceeded `max_output_bytes`.
    OutputLimit,
}

/// Result of one sandboxed run.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub termination: Termination,
    /// Exit code of the first process (an OS kill code if the tree was killed).
    pub exit_code: u32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// An allocation was refused because of the job memory limit.
    pub memory_limit_hit: bool,
    /// A process creation was refused because of the process limit.
    pub process_limit_hit: bool,
    pub user_time: Duration,
    pub kernel_time: Duration,
    pub wall_time: Duration,
    /// Job peak commit as Windows reports it. When an allocation is refused the refused
    /// size is included, so this can exceed `memory_bytes` when `memory_limit_hit`.
    pub peak_memory_bytes: u64,
    /// Processes ever created in the job (on Windows this includes conhost.exe).
    pub total_processes: u32,
    /// Highest active process count the runner observed while polling.
    pub max_active_processes: u32,
    /// Active processes left in the job after the run. Always 0 unless something is wrong.
    pub active_processes_after: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("sandboxing is not implemented on this platform yet")]
    Unsupported,
    #[error("invalid sandbox spec: {0}")]
    InvalidSpec(String),
    #[error("{op} failed with Win32 error {code}")]
    Win32 { op: &'static str, code: u32 },
    #[error("{op} failed with HRESULT {hr:#010x}")]
    HResult { op: &'static str, hr: u32 },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Run `spec` in the sandbox and wait for the whole process tree to finish.
pub fn run(spec: &SandboxSpec) -> Result<Outcome, SandboxError> {
    #[cfg(windows)]
    {
        windows::run(spec)
    }
    #[cfg(not(windows))]
    {
        let _ = spec;
        Err(SandboxError::Unsupported)
    }
}

/// Remove what `run` leaves behind: the ACL entries on `dirs` and the AppContainer profile.
pub fn cleanup(dirs: &[PathBuf]) -> Result<(), SandboxError> {
    #[cfg(windows)]
    {
        windows::cleanup(dirs)
    }
    #[cfg(not(windows))]
    {
        let _ = dirs;
        Err(SandboxError::Unsupported)
    }
}
