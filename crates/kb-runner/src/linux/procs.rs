//! Process-tree bookkeeping for the supervisor, from /proc. The supervisor is a child
//! subreaper, so every process the sandbox starts stays among its descendants (orphans
//! are re-parented to it), and nothing can leave the tree it enumerates and kills.

use std::collections::VecDeque;
use std::fs;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, kill};
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::{Pid, SysconfVar, sysconf};

/// One sample of /proc/<pid>/stat.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Stat {
    pub ppid: i32,
    /// utime + cutime, in clock ticks.
    pub user_ticks: u64,
    /// stime + cstime, in clock ticks.
    pub system_ticks: u64,
    pub threads: u32,
    pub rss_pages: u64,
}

pub(crate) fn stat(pid: i32) -> Option<Stat> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // The command name (field 2) may contain spaces and parentheses; fields after the
    // last ')' are fixed. f[0] is field 3 (state).
    let rest = &s[s.rfind(')')? + 2..];
    let f: Vec<&str> = rest.split_whitespace().collect();
    let n = |i: usize| f.get(i).and_then(|v| v.parse::<i64>().ok());
    Some(Stat {
        ppid: n(1)? as i32,
        user_ticks: (n(11)? + n(13)?.max(0)) as u64,
        system_ticks: (n(12)? + n(14)?.max(0)) as u64,
        // A zombie still holds its RLIMIT_NPROC slot until reaped.
        threads: n(17)?.max(1) as u32,
        rss_pages: n(21)?.max(0) as u64,
    })
}

/// Direct children of `pid`, across all its threads.
fn children(pid: i32) -> Vec<i32> {
    let mut out = Vec::new();
    let Ok(tasks) = fs::read_dir(format!("/proc/{pid}/task")) else {
        return out;
    };
    for t in tasks.flatten() {
        if let Ok(list) = fs::read_to_string(t.path().join("children")) {
            out.extend(
                list.split_whitespace()
                    .filter_map(|c| c.parse::<i32>().ok()),
            );
        }
    }
    out
}

/// Every descendant of `root` (not `root` itself), breadth first.
pub(crate) fn descendants(root: i32) -> Vec<i32> {
    let mut out = Vec::new();
    let mut queue: VecDeque<i32> = children(root).into();
    while let Some(p) = queue.pop_front() {
        out.push(p);
        queue.extend(children(p));
    }
    out
}

pub(crate) fn clock_ticks_per_sec() -> u64 {
    sysconf(SysconfVar::CLK_TCK)
        .ok()
        .flatten()
        .map_or(100, |v| v as u64)
}

pub(crate) fn page_size() -> u64 {
    sysconf(SysconfVar::PAGE_SIZE)
        .ok()
        .flatten()
        .map_or(4096, |v| v as u64)
}

pub(crate) fn ticks(t: u64, hz: u64) -> Duration {
    Duration::from_micros(t * 1_000_000 / hz)
}

/// Reap every exited child without blocking. Returns what was reaped.
pub(crate) fn reap() -> Vec<(i32, WaitStatus)> {
    let mut out = Vec::new();
    loop {
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::StillAlive) | Err(_) => return out,
            Ok(ws) => {
                if let Some(pid) = ws.pid() {
                    out.push((pid.as_raw(), ws));
                }
            }
        }
    }
}

/// SIGKILL every descendant of `root` until none is left (processes forking while we
/// kill are re-parented to us and caught on the next pass), reaping as we go. Returns
/// what was reaped and how many descendants were still alive when `grace` ran out.
pub(crate) fn kill_tree(root: i32, grace: Duration) -> (Vec<(i32, WaitStatus)>, usize) {
    let deadline = Instant::now() + grace;
    let mut reaped = Vec::new();
    loop {
        let alive = descendants(root);
        reaped.extend(reap());
        if alive.is_empty() {
            return (reaped, 0);
        }
        for p in &alive {
            // ESRCH: it died between listing and killing; it will be reaped below.
            let _ = kill(Pid::from_raw(*p), Signal::SIGKILL);
        }
        if Instant::now() >= deadline {
            reaped.extend(reap());
            return (reaped, descendants(root).len());
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// The kernel's RLIMIT_NPROC count for this user, measured: fork succeeds iff
/// count + 1 <= soft limit, so find the smallest soft limit at which spawning a no-op
/// helper succeeds. /proc alone undercounts: tasks in sibling PID namespaces (other WSL
/// distros, some container setups) are charged to the same user but are not visible.
/// Must run in a single-threaded process; restores the soft limit before returning.
pub(crate) fn kernel_task_count(uid: u32) -> std::io::Result<u64> {
    use nix::sys::resource::{Resource, getrlimit, setrlimit};
    let (soft, hard) = getrlimit(Resource::RLIMIT_NPROC)?;
    let exe = std::env::current_exe()?;
    let spawns_at = |limit: u64| -> std::io::Result<bool> {
        setrlimit(Resource::RLIMIT_NPROC, limit.min(hard), hard)?;
        let r = std::process::Command::new(&exe)
            .arg("__noop")
            .env_clear()
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        match r {
            Ok(_) => Ok(true),
            Err(e) if e.raw_os_error() == Some(nix::errno::Errno::EAGAIN as i32) => Ok(false),
            Err(e) => Err(e),
        }
    };
    let result = (|| {
        // Lower bound: what /proc shows. Grow until a spawn succeeds, then bisect.
        let mut lo = user_task_count(uid); // known failing or unknown
        let mut step = 16;
        let mut hi = lo + step;
        while !spawns_at(hi)? {
            lo = hi;
            step *= 2;
            hi = lo + step;
            if hi >= hard {
                return Err(std::io::Error::other(
                    "RLIMIT_NPROC count probe ran past the hard limit",
                ));
            }
        }
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if spawns_at(mid)? {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        Ok(hi - 1)
    })();
    setrlimit(Resource::RLIMIT_NPROC, soft, hard)?;
    result
}

/// Tasks (threads) owned by `uid` that this PID namespace can see.
pub(crate) fn user_task_count(uid: u32) -> u64 {
    let mut total = 0;
    let Ok(dir) = fs::read_dir("/proc") else {
        return 0;
    };
    for e in dir.flatten() {
        let name = e.file_name();
        if !name.to_string_lossy().bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let Ok(status) = fs::read_to_string(e.path().join("status")) else {
            continue;
        };
        let field = |key: &str| {
            status
                .lines()
                .find_map(|l| l.strip_prefix(key))
                .and_then(|v| v.split_whitespace().next())
                .and_then(|v| v.parse::<u64>().ok())
        };
        if field("Uid:") == Some(u64::from(uid)) {
            total += field("Threads:").unwrap_or(1);
        }
    }
    total
}
