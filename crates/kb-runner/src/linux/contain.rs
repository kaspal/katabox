//! `kb-sandbox __contain`: started by the supervisor, single-threaded, so hakoniwa's
//! fork is safe here (it runs Rust code between fork and exec).
//!
//! In fallback mode it first puts itself in a Landlock domain that scopes signals and
//! abstract Unix sockets (ABI 6). The program inherits that domain, so it can signal only
//! processes inside it (this stage and hakoniwa's runc, both expendable), never the
//! supervisor, the app or anything else the user runs.

use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::process::ExitCode;

use hakoniwa::{Container, Namespace, Rlimit, Stdio};
use landlock::{ABI, Access, CompatLevel, Compatible, Ruleset, RulesetAttr, Scope};
use nix::sys::prctl;
use nix::sys::signal::Signal;
use nix::unistd::{Uid, getppid};

use super::policy;
use super::procs;
use super::proto::{ContainJob, ContainStatus};
use crate::SandboxMode;

/// Fallback RLIMIT_NPROC headroom over the user's measured task count: hakoniwa's runc
/// (+1) and the user's other processes starting threads meanwhile (measured: a desktop
/// session with an IDE drifts by 10+ tasks within a second).
const FALLBACK_NPROC_HEADROOM: u64 = 16;

pub(crate) fn main(arg: Option<&String>) -> ExitCode {
    let Some(cj) = arg.and_then(|a| serde_json::from_str::<ContainJob>(a).ok()) else {
        eprintln!("kb-sandbox __contain: bad request");
        return ExitCode::from(2);
    };
    if prctl::set_pdeathsig(Signal::SIGKILL).is_err() || getppid().as_raw() != cj.supervisor_pid {
        return ExitCode::from(2);
    }
    let status = run(&cj).unwrap_or_else(ContainStatus::SetupFailed);
    let written = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&cj.status_path)
        .and_then(|mut f| {
            f.write_all(
                serde_json::to_string(&status)
                    .unwrap_or_default()
                    .as_bytes(),
            )
        });
    if written.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}

/// Scope signals and abstract Unix sockets to this process's descendants.
/// Required in fallback mode; best effort (skipped below ABI 6) in namespace mode, where
/// the PID and network namespaces already give the same isolation.
fn scope_self(mode: SandboxMode) -> Result<(), String> {
    let required = mode == SandboxMode::Fallback && !cfg!(feature = "unscoped-fallback-for-tests");
    let attempt = Ruleset::default()
        .set_compatibility(CompatLevel::HardRequirement)
        .scope(Scope::from_all(ABI::V6))
        .and_then(|r| r.create())
        .and_then(|r| r.restrict_self());
    match attempt {
        Ok(_) => Ok(()),
        Err(e) if required => Err(format!("Landlock signal scoping (ABI 6) unavailable: {e}")),
        Err(_) => Ok(()),
    }
}

fn run(cj: &ContainJob) -> Result<ContainStatus, String> {
    let job = &cj.job;
    scope_self(job.mode)?;

    let mut c = match job.mode {
        SandboxMode::Namespaces => {
            // new(): user (current uid/gid mapped), mount and PID namespaces, fresh /proc,
            // and an empty root (read-only inside) in the supervisor's private folder.
            let mut c = Container::new();
            c.rootdir(&cj.root_dir)
                .unshare(Namespace::Network)
                .unshare(Namespace::Ipc)
                .unshare(Namespace::Uts)
                .hostname("katabox")
                .devfsmount("/dev");
            for d in &job.read_only_dirs {
                let p = d.to_string_lossy();
                c.bindmount_ro(&p, &p);
            }
            for f in &job.host_files {
                let target = f.to_string_lossy();
                let source = std::fs::canonicalize(f).map_err(|e| format!("{target}: {e}"))?;
                c.bindmount_ro(&source.to_string_lossy(), &target);
            }
            let w = job.work_dir.to_string_lossy();
            c.bindmount_rw(&w, &w);
            c
        }
        // No mount namespace, so the root folder is unused; setting it stops hakoniwa from
        // creating its own temp folder.
        SandboxMode::Fallback => {
            let mut c = Container::empty();
            c.rootdir(&cj.root_dir);
            c
        }
        other => return Err(format!("mode {other:?} is not a Linux mode")),
    };

    c.landlock_ruleset(policy::landlock(
        job.mode,
        &job.read_only_dirs,
        &job.work_dir,
        &job.host_files,
    ));
    c.seccomp_filter(policy::seccomp(job.mode));

    // Backstops; the supervisor enforces the real CPU and wall limits across the tree.
    let cpu_secs = job.cpu_time.as_secs() + 1;
    let nproc = match job.mode {
        // Counted per user inside the new user namespace: only the sandbox's own tasks.
        SandboxMode::Namespaces => u64::from(job.max_processes),
        // Counted against everything the user runs, which drifts; approximate (ADR 0007).
        _ => {
            procs::kernel_task_count(Uid::current().as_raw())
                .map_err(|e| format!("measuring the RLIMIT_NPROC count: {e}"))?
                + u64::from(job.max_processes)
                + FALLBACK_NPROC_HEADROOM
        }
    };
    c.setrlimit(Rlimit::As, job.memory_bytes, job.memory_bytes)
        .setrlimit(Rlimit::Data, job.memory_bytes, job.memory_bytes)
        .setrlimit(Rlimit::Cpu, cpu_secs, cpu_secs + 1)
        .setrlimit(Rlimit::Nproc, nproc, nproc)
        .setrlimit(
            Rlimit::Fsize,
            policy::MAX_FILE_BYTES,
            policy::MAX_FILE_BYTES,
        )
        .setrlimit(
            Rlimit::Nofile,
            policy::MAX_OPEN_FILES,
            policy::MAX_OPEN_FILES,
        )
        .setrlimit(Rlimit::Core, 0, 0)
        .setrlimit(Rlimit::Memlock, 0, 0)
        .setrlimit(Rlimit::Msgqueue, 0, 0);

    let program = job.program.to_string_lossy();
    let mut cmd = c.command(&program);
    cmd.args(&job.args)
        .envs(job.env.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .current_dir(&job.work_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let mut child = cmd.spawn().map_err(|e| format!("spawn: {e}"))?;
    let st = child.wait().map_err(|e| format!("wait: {e}"))?;
    Ok(match st.exit_code {
        Some(code) => ContainStatus::Exited(code),
        None if st.code > 128 => ContainStatus::Signaled(st.code - 128),
        None => ContainStatus::SetupFailed(st.reason),
    })
}
