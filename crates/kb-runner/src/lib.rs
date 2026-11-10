//! Sandboxed execution of user code.
//!
//! This is the only crate allowed to spawn user programs. Any change here needs
//! explicit maintainer sign-off (see CLAUDE.md).
//!
//! Layout:
//! - `windows/`: Job Object (memory, CPU, process count, kill-on-close) + AppContainer
//!   with zero capabilities (no network), ACL grants for toolchain and work folders.
//! - `linux/`: hakoniwa (user/mount/PID/net namespaces, Landlock, seccomp, rlimits) run
//!   from a helper process (`kb-sandbox`); fallback to Landlock + seccomp + rlimits when
//!   user namespaces are blocked, only on kernels with Landlock signal scoping (ADR 0007).
//! - `pipeline` (planned): materialize workspace, compile if needed, run harness,
//!   stream results.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use kb_core::Limits;

// The test-only escape hatch must never reach a release build (ADR 0007).
#[cfg(all(feature = "unscoped-fallback-for-tests", not(debug_assertions)))]
compile_error!("`unscoped-fallback-for-tests` is for the attack suite only; never ship it");

mod cmdline;
#[cfg(target_os = "linux")]
mod linux;
mod validate;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{LinuxSupport, helper_main, probe as linux_probe, set_helper_path};

pub use cmdline::quote_windows_arg;
pub use validate::ROOT_MARKER;

/// Name of the AppContainer profile every sandboxed process runs in.
pub const APP_CONTAINER_NAME: &str = "KataBox.Sandbox";

/// Everything needed to run one program inside the sandbox.
#[derive(Debug, Clone)]
pub struct SandboxSpec {
    /// KataBox-owned folder (e.g. the app data folder) that must contain [`ROOT_MARKER`].
    /// `work_dir` and every `read_only_dirs` entry must be strictly inside it. Drive roots,
    /// network paths and the user profile (or anything containing it) are refused.
    pub sandbox_root: PathBuf,
    /// Absolute path to the executable. Must live inside one of `read_only_dirs`.
    pub program: PathBuf,
    /// Arguments after the program name.
    pub args: Vec<String>,
    /// Current directory of the child. The sandbox gets read, write and execute here
    /// during the run only: afterwards the grant is revoked and the folder deleted,
    /// unless `keep_work_dir`. Must not contain links or overlap `read_only_dirs`.
    pub work_dir: PathBuf,
    /// Keep `work_dir` after the run (grant still revoked, links removed, and every
    /// explicit permission inside it reset to inherited).
    pub keep_work_dir: bool,
    /// Folders the sandbox may read and execute from (e.g. the toolchain). Nothing else
    /// on disk is granted. These read-only grants persist between runs.
    pub read_only_dirs: Vec<PathBuf>,
    /// Extra environment variables. The child never inherits the host environment. On
    /// Windows it gets `SystemRoot`, `windir`, `TEMP`/`TMP` (= `work_dir`) plus these; on
    /// Linux `HOME`, `TMPDIR` (= `work_dir`), `LANG=C.UTF-8`, `PATH` (the program's folder).
    pub env: Vec<(String, String)>,
    pub stdin: Vec<u8>,
    /// Total CPU time (user + kernel) for the whole process tree.
    pub cpu_time: Duration,
    /// Wall-clock limit. Catches sleeping or blocked programs that use no CPU.
    pub wall_time: Duration,
    /// Committed memory for the whole process tree.
    pub memory_bytes: u64,
    /// Maximum simultaneously alive processes, including the first one. On Windows the
    /// console host (conhost.exe) runs in the job but is not counted against this. On
    /// Linux this is `RLIMIT_NPROC`, which counts **threads**, not just processes.
    pub max_processes: u32,
    /// Combined stdout + stderr cap; exceeding it kills the tree.
    pub max_output_bytes: usize,
    /// Run as a Less Privileged AppContainer: also drops access granted to
    /// "ALL APPLICATION PACKAGES" (much of `C:\Windows` and `Program Files`).
    /// Off by default: python-build-standalone 3.14 dies at startup under LPAC with
    /// STATUS_ACCESS_DENIED (cause not yet found). See the attack suite for the gap.
    pub less_privileged: bool,
    /// Which Linux isolation to use. Ignored on Windows.
    pub linux_isolation: LinuxIsolation,
}

/// Linux isolation choice (ADR 0007).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinuxIsolation {
    /// Namespaces when user namespaces work, otherwise the fallback (which is refused on
    /// kernels without Landlock signal scoping). Decided once per process, before a run.
    #[default]
    Auto,
    /// Require user, mount, PID and network namespaces; error if unavailable.
    Namespaces,
    /// Landlock + seccomp + rlimits without namespaces; error if the kernel lacks
    /// Landlock ABI 6 (signal scoping, Linux 6.12+).
    Fallback,
}

/// Which isolation actually ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SandboxMode {
    /// Windows: Job Object + AppContainer.
    AppContainer,
    /// Windows: Job Object + Less Privileged AppContainer.
    LessPrivilegedAppContainer,
    /// Linux: user/mount/PID/network namespaces + Landlock + seccomp + rlimits.
    Namespaces,
    /// Linux: Landlock (with signal scoping) + seccomp + rlimits, no namespaces.
    Fallback,
}

impl SandboxSpec {
    /// A spec with the given limits; wall time defaults to 3x the CPU limit.
    pub fn new(
        sandbox_root: impl Into<PathBuf>,
        program: impl Into<PathBuf>,
        work_dir: impl Into<PathBuf>,
        limits: &Limits,
    ) -> Self {
        let cpu_time = Duration::from_millis(u64::from(limits.cpu_time_ms));
        SandboxSpec {
            sandbox_root: sandbox_root.into(),
            program: program.into(),
            args: Vec::new(),
            work_dir: work_dir.into(),
            keep_work_dir: false,
            read_only_dirs: Vec::new(),
            env: Vec::new(),
            stdin: Vec::new(),
            cpu_time,
            wall_time: cpu_time * 3,
            memory_bytes: u64::from(limits.memory_mb) * 1024 * 1024,
            max_processes: limits.max_processes,
            max_output_bytes: limits.max_output_kb as usize * 1024,
            less_privileged: false,
            linux_isolation: LinuxIsolation::Auto,
        }
    }
}

/// Why the process tree stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Termination {
    /// Every process exited on its own.
    Exited,
    /// User + kernel CPU time reached `cpu_time` and the tree was killed.
    CpuTimeLimit,
    /// The runner killed the tree at the wall-clock deadline.
    WallTimeLimit,
    /// The runner killed the tree because output exceeded `max_output_bytes`.
    OutputLimit,
}

/// Result of one sandboxed run.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// The isolation that ran.
    pub mode: SandboxMode,
    pub termination: Termination,
    /// Exit code of the first process (an OS kill code if the tree was killed).
    pub exit_code: u32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// An allocation was refused because of the job memory limit. Always false on Linux
    /// until cgroups land: rlimit refusals are not observable from outside (ADR 0007).
    pub memory_limit_hit: bool,
    /// A process creation was refused because of the process limit. On Linux this is
    /// inferred: the observed thread count reached `max_processes`.
    pub process_limit_hit: bool,
    pub user_time: Duration,
    pub kernel_time: Duration,
    pub wall_time: Duration,
    /// Job peak commit as Windows reports it. When an allocation is refused the refused
    /// size is included, so this can exceed `memory_bytes` when `memory_limit_hit`.
    /// Linux: the highest sampled sum of resident memory across the tree, or the largest
    /// single-process peak RSS, whichever is higher.
    pub peak_memory_bytes: u64,
    /// Processes ever created in the job (on Windows this includes conhost.exe). Linux:
    /// distinct sandbox processes seen while polling every 20 ms (short-lived ones can be
    /// missed).
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
    /// The requested isolation is not available on this system (e.g. user namespaces
    /// blocked, or fallback mode on a kernel without Landlock signal scoping).
    #[error("sandbox isolation unavailable: {0}")]
    Unavailable(String),
    #[error("invalid sandbox spec: {0}")]
    InvalidSpec(String),
    #[error("{op} failed with Win32 error {code}")]
    Win32 { op: &'static str, code: u32 },
    #[error("{op} failed with HRESULT {hr:#010x}")]
    HResult { op: &'static str, hr: u32 },
    #[error("sandbox internal error: {0}")]
    Internal(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Held for the whole of every `run` and `cleanup`. All runs share one AppContainer SID,
/// so two runs active at once could reach each other's work folders (ADR 0006). This
/// serializes runs within one process; it does not coordinate separate processes.
static RUN_LOCK: Mutex<()> = Mutex::new(());

fn run_lock() -> MutexGuard<'static, ()> {
    // A panic in an earlier run must not wedge every later one; the lock guards no data.
    RUN_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// Run `spec` in the sandbox and wait for the whole process tree to finish.
///
/// Only one sandboxed run happens at a time in this process: a second call blocks until
/// the first has finished, cleanup included.
///
/// The spec is validated before anything is created or granted. Once validation passes,
/// `work_dir` is cleaned up after the run even if the run fails: its grant is revoked
/// and it is deleted unless `keep_work_dir`. A cleanup failure is returned as an error.
pub fn run(spec: &SandboxSpec) -> Result<Outcome, SandboxError> {
    let _one_at_a_time = run_lock();
    #[cfg(windows)]
    {
        windows::run(spec)
    }
    #[cfg(target_os = "linux")]
    {
        linux::run(spec)
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        let _ = spec;
        Err(SandboxError::Unsupported)
    }
}

/// Remove what `run` leaves behind: the ACL entries on `dirs` and the AppContainer profile.
/// Waits for any run in progress, so the profile is never deleted under a running sandbox.
/// On Linux nothing persists between runs, so this only waits for the lock.
pub fn cleanup(dirs: &[PathBuf]) -> Result<(), SandboxError> {
    let _one_at_a_time = run_lock();
    #[cfg(windows)]
    {
        windows::cleanup(dirs)
    }
    #[cfg(target_os = "linux")]
    {
        let _ = dirs;
        Ok(())
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        let _ = dirs;
        Err(SandboxError::Unsupported)
    }
}
