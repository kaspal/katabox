//! `kb-sandbox __supervise`: one per run, started by the runner.
//!
//! Outside every Landlock domain, so the sandbox cannot signal it. It becomes a child
//! subreaper (orphans come back to it), detaches from any controlling terminal, starts
//! `__contain`, pumps stdio, polls the tree every 20 ms for CPU time (user + kernel,
//! including reaped children), wall time and output, kills the whole tree on any limit,
//! and reports to the runner.

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use nix::sys::prctl;
use nix::sys::resource::{UsageWho, getrusage};
use nix::sys::signal::Signal;
use nix::sys::wait::WaitStatus;
use nix::unistd::{getppid, setsid};

use super::procs::{self, Stat};
use super::proto::{ContainJob, ContainStatus, Job, Report};
use crate::Termination;

/// Exit codes the runner uses when it kills the tree itself ("KB" + reason), same as
/// on Windows.
const KILLED_WALL_TIME: u32 = 0x4B42_0001;
const KILLED_OUTPUT: u32 = 0x4B42_0002;
const KILLED_CPU_TIME: u32 = 0x4B42_0003;

const POLL: Duration = Duration::from_millis(20);
const KILL_GRACE: Duration = Duration::from_secs(5);

pub(crate) fn main() -> ExitCode {
    let mut input = BufReader::new(std::io::stdin().lock());
    let mut header = String::new();
    let job: Job = match input
        .read_line(&mut header)
        .map_err(|e| e.to_string())
        .and_then(|_| serde_json::from_str(&header).map_err(|e| e.to_string()))
    {
        Ok(j) => j,
        Err(e) => return fail(&format!("bad request: {e}")),
    };
    let mut stdin_data = vec![0; job.stdin_len];
    if let Err(e) = input.read_exact(&mut stdin_data) {
        return fail(&format!("short stdin: {e}"));
    }
    drop(input);

    // Die with the runner; then make sure the runner did not die before that took hold.
    if prctl::set_pdeathsig(Signal::SIGKILL).is_err() || getppid().as_raw() != job.runner_pid {
        return fail("runner is gone");
    }
    // No controlling terminal for anything below us: /dev/tty then fails with ENXIO.
    if let Err(e) = setsid() {
        return fail(&format!("setsid: {e}"));
    }
    if let Err(e) = prctl::set_child_subreaper(true) {
        return fail(&format!("PR_SET_CHILD_SUBREAPER: {e}"));
    }

    let (report, stdout, stderr) = supervise(&job, stdin_data);
    let mut out = std::io::stdout().lock();
    let line = serde_json::to_string(&report).expect("report serializes");
    let ok = writeln!(out, "{line}")
        .and_then(|()| out.write_all(&stdout))
        .and_then(|()| out.write_all(&stderr))
        .and_then(|()| out.flush());
    if ok.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("kb-sandbox __supervise: {msg}");
    ExitCode::from(2)
}

fn empty_report(job: &Job, error: String) -> Report {
    Report {
        error: Some(error),
        mode: job.mode,
        termination: Termination::Exited,
        exit_code: 0,
        user_time: Duration::ZERO,
        kernel_time: Duration::ZERO,
        wall_time: Duration::ZERO,
        peak_memory_bytes: 0,
        total_processes: 0,
        max_active_processes: 0,
        max_threads: 0,
        process_limit_hit: false,
        active_processes_after: 0,
        stdout_len: 0,
        stderr_len: 0,
    }
}

/// A private folder for the status file, outside every path the sandbox is granted.
struct StatusDir(PathBuf);

impl StatusDir {
    fn new() -> std::io::Result<Self> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("kb-sandbox-{}-{nanos}", std::process::id()));
        std::fs::DirBuilder::new().mode(0o700).create(&dir)?;
        Ok(StatusDir(dir))
    }
    fn file(&self) -> PathBuf {
        self.0.join("status.json")
    }
    fn root(&self) -> std::io::Result<PathBuf> {
        let root = self.0.join("root");
        std::fs::DirBuilder::new().mode(0o700).create(&root)?;
        Ok(root)
    }
}

impl Drop for StatusDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn supervise(job: &Job, stdin_data: Vec<u8>) -> (Report, Vec<u8>, Vec<u8>) {
    let status_dir = match StatusDir::new() {
        Ok(d) => d,
        Err(e) => {
            return (
                empty_report(job, format!("status dir: {e}")),
                vec![],
                vec![],
            );
        }
    };
    let me = std::process::id() as i32;
    let root_dir = match status_dir.root() {
        Ok(r) => r,
        Err(e) => return (empty_report(job, format!("root dir: {e}")), vec![], vec![]),
    };
    let contain_job = ContainJob {
        job: job.clone(),
        supervisor_pid: me,
        status_path: status_dir.file(),
        root_dir,
    };
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            return (
                empty_report(job, format!("current_exe: {e}")),
                vec![],
                vec![],
            );
        }
    };
    let arg = serde_json::to_string(&contain_job).expect("job serializes");
    let mut child = match Command::new(&exe)
        .arg("__contain")
        .arg(arg)
        .env_clear()
        .current_dir("/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return (
                empty_report(job, format!("spawn __contain: {e}")),
                vec![],
                vec![],
            );
        }
    };
    let start = Instant::now();
    let contain_pid = child.id() as i32;

    let output = Arc::new(OutputState::default());
    let out_t = spawn_reader(child.stdout.take(), &output, job.max_output_bytes);
    let err_t = spawn_reader(child.stderr.take(), &output, job.max_output_bytes);
    let mut stdin_w = child.stdin.take();
    let in_t = thread::spawn(move || {
        if let Some(w) = stdin_w.as_mut() {
            let _ = w.write_all(&stdin_data); // the program may exit without reading
        }
    });
    // `child` is never waited on through std: the reaping loop below collects it.

    let hz = procs::clock_ticks_per_sec();
    let page = procs::page_size();
    let mut contain_exit: Option<WaitStatus> = None;
    let mut termination: Option<Termination> = None;
    let mut seen: HashSet<i32> = HashSet::new();
    let (mut max_active, mut max_threads, mut peak_rss) = (0u32, 0u32, 0u64);
    // Last sampled (user, kernel) time; reported if higher than the final rusage, which
    // misses time of processes reaped inside a dying PID namespace.
    let mut sampled: (Duration, Duration);

    loop {
        for (pid, ws) in procs::reap() {
            if pid == contain_pid {
                contain_exit = Some(ws);
            }
        }
        let tree = procs::descendants(me);
        let (mut user, mut system) = (0u64, 0u64);
        let (mut active, mut threads, mut rss) = (0u32, 0u32, 0u64);
        for pid in &tree {
            let Some(Stat {
                ppid,
                user_ticks,
                system_ticks,
                threads: t,
                rss_pages,
            }) = procs::stat(*pid)
            else {
                continue;
            };
            user += user_ticks;
            system += system_ticks;
            // `__contain` and hakoniwa's runc (its child) are ours, not the program's.
            if *pid != contain_pid && ppid != contain_pid {
                active += 1;
                threads += t;
                rss += rss_pages * page;
                seen.insert(*pid);
            }
        }
        let reaped = getrusage(UsageWho::RUSAGE_CHILDREN).ok();
        sampled = (
            procs::ticks(user, hz) + reaped.map_or(Duration::ZERO, |r| tv(r.user_time())),
            procs::ticks(system, hz) + reaped.map_or(Duration::ZERO, |r| tv(r.system_time())),
        );
        let cpu = sampled.0 + sampled.1;
        max_active = max_active.max(active);
        max_threads = max_threads.max(threads);
        peak_rss = peak_rss.max(rss);

        if termination.is_none() {
            if cpu >= job.cpu_time {
                termination = Some(Termination::CpuTimeLimit);
            } else if start.elapsed() >= job.wall_time {
                termination = Some(Termination::WallTimeLimit);
            } else if output.overflow.load(Ordering::SeqCst) {
                termination = Some(Termination::OutputLimit);
            }
        }
        if termination.is_some() || (tree.is_empty() && contain_exit.is_some()) {
            break;
        }
        thread::sleep(POLL);
    }

    // Always sweep: nothing may outlive the run.
    let (reaped, left) = procs::kill_tree(me, KILL_GRACE);
    for (pid, ws) in reaped {
        if pid == contain_pid {
            contain_exit = Some(ws);
        }
    }
    let wall_time = start.elapsed();
    let _ = in_t.join();
    let stdout = out_t.join().unwrap_or_default();
    let stderr = err_t.join().unwrap_or_default();

    let usage = getrusage(UsageWho::RUSAGE_CHILDREN).ok();
    let (user_time, kernel_time) = usage.map_or(sampled, |r| {
        (
            tv(r.user_time()).max(sampled.0),
            tv(r.system_time()).max(sampled.1),
        )
    });
    let max_rss = usage.map_or(0, |r| r.max_rss().max(0) as u64 * 1024);

    let status = read_status(&status_dir.file());
    let ran = matches!(
        status,
        Some(ContainStatus::Exited(_) | ContainStatus::Signaled(_))
    ) || termination.is_some();
    let mut error = None;
    let exit_code = match termination {
        Some(Termination::WallTimeLimit) => KILLED_WALL_TIME,
        Some(Termination::OutputLimit) => KILLED_OUTPUT,
        Some(Termination::CpuTimeLimit) => KILLED_CPU_TIME,
        _ => match status {
            Some(ContainStatus::Exited(c)) => c as u32,
            Some(ContainStatus::Signaled(s)) => 128 + s as u32,
            Some(ContainStatus::SetupFailed(msg)) => {
                error = Some(format!("sandbox setup failed: {msg}"));
                0
            }
            None => {
                error = Some(format!(
                    "sandbox helper exited without a status ({contain_exit:?})"
                ));
                0
            }
        },
    };
    if left > 0 {
        error = Some(format!("{left} sandbox processes survived SIGKILL"));
    }

    let report = Report {
        error,
        mode: job.mode,
        termination: termination.unwrap_or(Termination::Exited),
        exit_code,
        user_time,
        kernel_time,
        wall_time,
        peak_memory_bytes: peak_rss.max(max_rss),
        // The first process may finish before the first 20 ms sample.
        total_processes: (seen.len() as u32).max(u32::from(ran)),
        max_active_processes: max_active.max(u32::from(ran)),
        max_threads,
        // A sample never sees the task whose creation was refused, so a tree pinned at
        // one below the limit is the observable sign of refusals.
        process_limit_hit: max_threads + 1 >= job.max_processes,
        active_processes_after: left as u32,
        stdout_len: stdout.len(),
        stderr_len: stderr.len(),
    };
    (report, stdout, stderr)
}

fn tv(t: nix::sys::time::TimeVal) -> Duration {
    Duration::from_secs(t.tv_sec().max(0) as u64) + Duration::from_micros(t.tv_usec().max(0) as u64)
}

fn read_status(path: &Path) -> Option<ContainStatus> {
    let s = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&s).ok()
}

#[derive(Default)]
struct OutputState {
    total: AtomicUsize,
    overflow: AtomicBool,
    lock: Mutex<()>,
}

/// Read one stream until EOF, keeping bytes while the combined stdout + stderr total
/// stays within `max`. Past that, mark overflow (the supervisor then kills the tree)
/// and keep draining so the writer never blocks.
fn spawn_reader(
    stream: Option<impl Read + Send + 'static>,
    state: &Arc<OutputState>,
    max: usize,
) -> JoinHandle<Vec<u8>> {
    let state = Arc::clone(state);
    thread::spawn(move || {
        let mut kept = Vec::new();
        let Some(mut stream) = stream else {
            return kept;
        };
        let mut buf = [0u8; 16 * 1024];
        loop {
            let n = match stream.read(&mut buf) {
                Ok(0) | Err(_) => return kept,
                Ok(n) => n,
            };
            let _g = state.lock.lock().unwrap_or_else(|e| e.into_inner());
            let total = state.total.load(Ordering::SeqCst);
            let room = max.saturating_sub(total);
            let take = n.min(room);
            kept.extend_from_slice(&buf[..take]);
            state.total.store(total + take, Ordering::SeqCst);
            if take < n {
                state.overflow.store(true, Ordering::SeqCst);
            }
        }
    })
}
