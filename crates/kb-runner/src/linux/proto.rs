//! Messages between the runner (in the app) and the two helper stages.
//!
//! runner --stdin--> `kb-sandbox __supervise`: one JSON line ([`Job`]) followed by exactly
//! `stdin_len` raw bytes for the sandboxed program.
//! `__supervise` --argv--> `__contain`: one JSON argument ([`ContainJob`]).
//! `__contain` --status file--> `__supervise`: [`ContainStatus`] as JSON, written to a
//! private file the sandbox cannot reach (it is outside every allowed path).
//! `__supervise` --stdout--> runner: one JSON line ([`Report`]) followed by
//! `stdout_len` + `stderr_len` raw bytes.

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{SandboxMode, Termination};

/// Everything the supervisor needs. All paths are already validated and canonical.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub mode: SandboxMode,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub work_dir: PathBuf,
    pub read_only_dirs: Vec<PathBuf>,
    /// Individual host files the program needs (glibc core), as the loader looks them up.
    pub host_files: Vec<PathBuf>,
    pub env: Vec<(String, String)>,
    pub cpu_time: Duration,
    pub wall_time: Duration,
    pub memory_bytes: u64,
    pub max_processes: u32,
    pub max_output_bytes: usize,
    /// PID of the runner; the supervisor exits if its parent is not this process.
    pub runner_pid: i32,
    pub stdin_len: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ContainJob {
    pub job: Job,
    pub supervisor_pid: i32,
    pub status_path: PathBuf,
    /// Empty folder for the namespace's root (mount points). Lives in the supervisor's
    /// private folder, which it removes after every process is dead, even when it had
    /// to kill `__contain` (hakoniwa's own temp root would then be left in /tmp).
    pub root_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ContainStatus {
    Exited(i32),
    Signaled(i32),
    SetupFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Report {
    /// Set when the run could not be carried out (setup failure); the rest is then zero.
    pub error: Option<String>,
    pub mode: SandboxMode,
    pub termination: Termination,
    pub exit_code: u32,
    pub user_time: Duration,
    pub kernel_time: Duration,
    pub wall_time: Duration,
    pub peak_memory_bytes: u64,
    pub total_processes: u32,
    pub max_active_processes: u32,
    pub max_threads: u32,
    /// Inferred: the observed thread count reached `max_processes` (RLIMIT_NPROC counts
    /// threads; the refusal itself is not observable from outside).
    pub process_limit_hit: bool,
    pub active_processes_after: u32,
    pub stdout_len: usize,
    pub stderr_len: usize,
}
