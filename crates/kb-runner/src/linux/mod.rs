//! Linux sandbox (ADR 0002, ADR 0007).
//!
//! The runner never forks in-process: hakoniwa runs Rust code between fork and exec,
//! which can deadlock in a multithreaded process (the app). Each run starts the
//! `kb-sandbox` helper twice over:
//!
//! ```text
//! app (runner) ── kb-sandbox __supervise   subreaper, limits, stdio, kill, report
//!                   └─ kb-sandbox __contain  [fallback: Landlock signal/abstract-socket scope]
//!                        └─ hakoniwa runc     namespaces, mounts
//!                             └─ program      Landlock fs, seccomp, rlimits
//! ```
//!
//! No `unsafe` here: nix, hakoniwa and landlock provide safe wrappers.

mod contain;
mod policy;
mod probe;
mod procs;
mod proto;
mod supervise;
mod workdir;

use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

pub use probe::{LinuxSupport, probe};

use crate::validate;
use crate::{LinuxIsolation, Outcome, SandboxError, SandboxMode, SandboxSpec};
use proto::{Job, Report};

static HELPER: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Where the `kb-sandbox` helper binary is. Default: next to the current executable
/// (the app ships it as a sidecar), or the current executable if it is `kb-sandbox`.
pub fn set_helper_path(path: impl Into<PathBuf>) {
    *HELPER.lock().unwrap_or_else(|e| e.into_inner()) = Some(path.into());
}

fn helper() -> Result<PathBuf, SandboxError> {
    if let Some(p) = HELPER.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        return Ok(p);
    }
    let exe = std::env::current_exe()?;
    if exe.file_name().is_some_and(|n| n == "kb-sandbox") {
        return Ok(exe);
    }
    let sidecar = exe.with_file_name("kb-sandbox");
    if sidecar.is_file() {
        Ok(sidecar)
    } else {
        Err(SandboxError::Unavailable(format!(
            "sandbox helper not found at {}",
            sidecar.display()
        )))
    }
}

/// Entry point for the helper's hidden subcommands. Returns `None` if `cmd` is not one.
pub fn helper_main(cmd: &str, rest: &[String]) -> Option<ExitCode> {
    match cmd {
        "__supervise" => Some(supervise::main()),
        "__contain" => Some(contain::main(rest.first())),
        "__probe-userns" => Some(probe::userns_main()),
        "__noop" => Some(ExitCode::SUCCESS),
        _ => None,
    }
}

/// Pick the mode before anything runs. Never downgrades after a failure: if namespaces
/// were chosen and setup fails, the run fails.
fn choose_mode(want: LinuxIsolation) -> Result<SandboxMode, SandboxError> {
    let s = probe()?;
    let ns_problem = if s.landlock_abi == 0 {
        Some("Landlock is not available".to_string())
    } else if !s.user_namespaces {
        Some(format!(
            "user namespaces are blocked: {}",
            s.user_namespaces_error.as_deref().unwrap_or("unknown")
        ))
    } else {
        None
    };
    let fallback_problem = if s.signal_scoping() || cfg!(feature = "unscoped-fallback-for-tests") {
        None
    } else {
        Some(format!(
            "fallback mode needs Landlock signal scoping (ABI 6, Linux 6.12+); kernel {} has \
             Landlock ABI {}",
            s.kernel, s.landlock_abi
        ))
    };
    let seccomp_problem = (!s.seccomp).then(|| "seccomp is not available".to_string());
    if let Some(p) = seccomp_problem {
        return Err(SandboxError::Unavailable(p));
    }
    match (want, ns_problem, fallback_problem) {
        (LinuxIsolation::Namespaces | LinuxIsolation::Auto, None, _) => Ok(SandboxMode::Namespaces),
        (LinuxIsolation::Namespaces, Some(p), _) => Err(SandboxError::Unavailable(p)),
        (LinuxIsolation::Fallback | LinuxIsolation::Auto, _, None) => Ok(SandboxMode::Fallback),
        (LinuxIsolation::Fallback, _, Some(f)) => Err(SandboxError::Unavailable(f)),
        (LinuxIsolation::Auto, Some(p), Some(f)) => {
            Err(SandboxError::Unavailable(format!("{p}; and {f}")))
        }
    }
}

pub(crate) fn run(spec: &SandboxSpec) -> Result<Outcome, SandboxError> {
    // Validate, choose the mode and find the host files before anything runs. Failing
    // here leaves the work folder untouched.
    let paths = validate::resolve(spec)?;
    let mode = choose_mode(spec.linux_isolation)?;
    let host_files = policy::host_files().map_err(SandboxError::Unavailable)?;

    let mut env = vec![
        (
            "HOME".to_string(),
            paths.work_dir.to_string_lossy().into_owned(),
        ),
        (
            "TMPDIR".to_string(),
            paths.work_dir.to_string_lossy().into_owned(),
        ),
        ("LANG".to_string(), "C.UTF-8".to_string()),
    ];
    if let Some(dir) = paths.program.parent() {
        env.push(("PATH".to_string(), dir.to_string_lossy().into_owned()));
    }
    env.extend(spec.env.iter().cloned());

    let job = Job {
        mode,
        program: paths.program.clone(),
        args: spec.args.clone(),
        work_dir: paths.work_dir.clone(),
        read_only_dirs: paths.read_only_dirs.clone(),
        host_files,
        env,
        cpu_time: spec.cpu_time,
        wall_time: spec.wall_time,
        memory_bytes: spec.memory_bytes,
        max_processes: spec.max_processes,
        max_output_bytes: spec.max_output_bytes,
        runner_pid: std::process::id() as i32,
        stdin_len: spec.stdin.len(),
    };
    let result = execute(&job, &spec.stdin);
    // `execute` returned, so the supervisor reported every sandboxed process dead (or
    // was killed, in which case the run is an error anyway).
    let cleanup = workdir::finish(&paths.work_dir, spec.keep_work_dir);
    match (result, cleanup) {
        (Ok(outcome), Ok(())) => Ok(outcome),
        (Err(e), _) | (Ok(_), Err(e)) => Err(e),
    }
}

/// Grace on top of the wall limit before the runner kills a supervisor that never
/// reported (it enforces the wall limit itself; this only catches a wedged helper).
const SUPERVISOR_GRACE: Duration = Duration::from_secs(10);

fn execute(job: &Job, stdin: &[u8]) -> Result<Outcome, SandboxError> {
    let mut child = Command::new(helper()?)
        .arg("__supervise")
        .env_clear()
        .current_dir("/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut w = child.stdin.take().expect("piped stdin");
    let mut header = serde_json::to_string(job).expect("job serializes");
    header.push('\n');
    let data = stdin.to_vec();
    let in_t = thread::spawn(move || {
        let _ = w
            .write_all(header.as_bytes())
            .and_then(|()| w.write_all(&data));
    });
    let mut out = child.stdout.take().expect("piped stdout");
    let out_t = thread::spawn(move || {
        let mut v = Vec::new();
        let _ = out.read_to_end(&mut v);
        v
    });
    let mut err = child.stderr.take().expect("piped stderr");
    let err_t = thread::spawn(move || {
        let mut v = Vec::new();
        let _ = err.read_to_end(&mut v);
        v
    });

    let deadline = Instant::now() + job.wall_time + SUPERVISOR_GRACE;
    let status = loop {
        if let Some(st) = child.try_wait()? {
            break Some(st);
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        thread::sleep(Duration::from_millis(10));
    };
    let _ = in_t.join();
    let raw = out_t.join().unwrap_or_default();
    let diag = String::from_utf8_lossy(&err_t.join().unwrap_or_default()).into_owned();
    let Some(status) = status else {
        return Err(SandboxError::Internal(format!(
            "sandbox supervisor did not finish within the wall limit + {SUPERVISOR_GRACE:?}; killed"
        )));
    };
    if !status.success() {
        return Err(SandboxError::Internal(format!(
            "sandbox supervisor failed ({status}): {}",
            diag.trim()
        )));
    }
    parse_report(&raw)
}

fn parse_report(raw: &[u8]) -> Result<Outcome, SandboxError> {
    let mut r = BufReader::new(raw);
    let mut line = String::new();
    r.read_line(&mut line)?;
    let report: Report = serde_json::from_str(&line)
        .map_err(|e| SandboxError::Internal(format!("bad supervisor report: {e}")))?;
    if let Some(e) = report.error {
        return Err(SandboxError::Internal(e));
    }
    let mut stdout = vec![0; report.stdout_len];
    let mut stderr = vec![0; report.stderr_len];
    r.read_exact(&mut stdout)?;
    r.read_exact(&mut stderr)?;
    Ok(Outcome {
        mode: report.mode,
        termination: report.termination,
        exit_code: report.exit_code,
        stdout,
        stderr,
        memory_limit_hit: false,
        process_limit_hit: report.process_limit_hit,
        user_time: report.user_time,
        kernel_time: report.kernel_time,
        wall_time: report.wall_time,
        peak_memory_bytes: report.peak_memory_bytes,
        total_processes: report.total_processes,
        max_active_processes: report.max_active_processes,
        active_processes_after: report.active_processes_after,
    })
}
