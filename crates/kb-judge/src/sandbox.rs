//! The cross-process sandbox lock. Every sandboxed run, from the app, kb-pack or a test
//! binary, takes an exclusive lock on `<sandbox_root>/.sandbox.lock` first, so runs from
//! separate processes queue instead of overlapping. All runs share one AppContainer SID
//! (ADR 0006), and kb-runner's own lock only covers one process.
//!
//! The lock wraps single `kb_runner::run` calls and never spans two, so a locked section
//! can't wait on itself; a thread-local guard turns an accidental nesting into a panic in
//! debug builds instead of a deadlock.

use std::cell::Cell;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use kb_runner::{Outcome, SandboxError, SandboxSpec};

/// The lock file's name inside the sandbox root.
pub const LOCK_FILE: &str = ".sandbox.lock";

thread_local! {
    static HELD: Cell<bool> = const { Cell::new(false) };
    static QUEUED: Cell<Duration> = const { Cell::new(Duration::ZERO) };
}

/// The total time this thread has waited for the lock so far. A caller with a time
/// budget (the complexity profiler) subtracts the difference across its work, so time
/// queued behind other sandbox users is not counted against it.
pub fn queued() -> Duration {
    QUEUED.with(Cell::get)
}

/// The held lock, released on drop.
#[derive(Debug)]
pub struct SandboxLock {
    file: fs::File,
}

impl SandboxLock {
    /// Wait for the lock on `root`'s sandbox (blocking).
    pub fn acquire(root: &Path) -> std::io::Result<SandboxLock> {
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(root.join(LOCK_FILE))?;
        fs4::FileExt::lock(&file)?;
        Ok(SandboxLock { file })
    }
}

impl Drop for SandboxLock {
    fn drop(&mut self) {
        let _ = fs4::FileExt::unlock(&self.file);
    }
}

/// `kb_runner::run` under the cross-process lock.
pub fn run(spec: &SandboxSpec) -> Result<Outcome, SandboxError> {
    debug_assert!(
        !HELD.with(Cell::get),
        "a sandbox run must not start another one"
    );
    let waiting = Instant::now();
    let _lock = SandboxLock::acquire(&spec.sandbox_root)?;
    QUEUED.with(|q| q.set(q.get() + waiting.elapsed()));
    HELD.with(|h| h.set(true));
    let outcome = kb_runner::run(spec);
    HELD.with(|h| h.set(false));
    outcome
}

/// Run `f` (for example `kb_runner::cleanup`) holding the lock of every existing root,
/// taken in path order so two such callers can't deadlock.
pub fn with_locks<T>(roots: &[std::path::PathBuf], f: impl FnOnce() -> T) -> std::io::Result<T> {
    let mut sorted: Vec<_> = roots.iter().filter(|r| r.is_dir()).collect();
    sorted.sort();
    sorted.dedup();
    let _locks = sorted
        .into_iter()
        .map(|r| SandboxLock::acquire(r))
        .collect::<std::io::Result<Vec<_>>>()?;
    Ok(f())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn a_second_holder_waits_until_the_first_lets_go() {
        let dir = std::env::temp_dir().join(format!("kb-judge-lock-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let first = SandboxLock::acquire(&dir).unwrap();
        let (tx, rx) = mpsc::channel();
        let d = dir.clone();
        let waiter = std::thread::spawn(move || {
            let _second = SandboxLock::acquire(&d).unwrap();
            tx.send(()).unwrap();
        });
        assert!(
            rx.recv_timeout(Duration::from_millis(300)).is_err(),
            "the second holder must wait"
        );
        drop(first);
        rx.recv_timeout(Duration::from_secs(10))
            .expect("the second holder gets the lock once it is free");
        waiter.join().unwrap();
        let _ = fs::remove_dir_all(&dir);
    }

    /// Not a test on its own: the child half of the cross-process test. With
    /// `KB_LOCK_CHILD=<dir>` it takes the lock, says so with a marker file, holds the lock
    /// for a second and exits.
    #[test]
    fn lock_holder_child() {
        let Ok(dir) = std::env::var("KB_LOCK_CHILD") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        let _lock = SandboxLock::acquire(&dir).unwrap();
        fs::write(dir.join("held"), b"1").unwrap();
        std::thread::sleep(Duration::from_millis(1500));
    }

    #[test]
    fn another_process_holding_the_lock_makes_us_wait() {
        let dir = std::env::temp_dir().join(format!("kb-judge-xlock-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "sandbox::tests::lock_holder_child",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("KB_LOCK_CHILD", &dir)
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let start = std::time::Instant::now();
        while !dir.join("held").exists() {
            assert!(
                start.elapsed() < Duration::from_secs(30),
                "the child never took the lock"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        let waited = std::time::Instant::now();
        let lock = SandboxLock::acquire(&dir).unwrap();
        let held_for = waited.elapsed();
        drop(lock);
        assert!(child.wait().unwrap().success());
        assert!(
            held_for >= Duration::from_millis(800),
            "we got the lock after {held_for:?}, while the child still held it"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
