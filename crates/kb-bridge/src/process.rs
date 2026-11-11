//! Starting a server process: clean environment, its project as the working folder, no
//! console window, and on Windows a Job Object that kills the whole process tree
//! (tsserver, rustc queries, …) when the server stops, crashes or the app exits.
//!
//! The child is owned by a waiter task that awaits its exit exactly once (tokio's
//! `Child::wait` must not be cancelled and re-polled on Windows). Killing goes through
//! the Job Object on Windows and through the waiter elsewhere.

use std::io;
use std::process::Stdio;

use tokio::process::{ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::sync::oneshot;

use crate::spec::ServerSpec;

pub struct ServerProcess {
    pub stdin: ChildStdin,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
    /// Resolves once with the exit code (`None` when killed by a signal or unknown).
    pub exited: oneshot::Receiver<Option<i32>>,
    /// Closing the job (on drop or [`ServerProcess::kill`]) kills every process in it.
    #[cfg(windows)]
    job: Option<win32job::Job>,
    #[cfg(not(windows))]
    kill: Option<oneshot::Sender<()>>,
}

impl ServerProcess {
    /// Kill the server and everything it started.
    pub fn kill(&mut self) {
        #[cfg(windows)]
        drop(self.job.take());
        #[cfg(not(windows))]
        if let Some(k) = self.kill.take() {
            let _ = k.send(());
        }
    }
}

pub fn spawn(spec: &ServerSpec) -> io::Result<ServerProcess> {
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args)
        .env_clear()
        .envs(spec.env.iter().map(|(k, v)| (k, v)))
        .current_dir(&spec.cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(unix)]
    cmd.process_group(0);

    #[cfg(windows)]
    let job = {
        let mut info = win32job::ExtendedLimitInfo::new();
        info.limit_kill_on_job_close();
        win32job::Job::create_with_limit_info(&info)
            .map_err(|e| io::Error::other(format!("job object: {e}")))?
    };

    let mut child = cmd.spawn()?;

    #[cfg(windows)]
    {
        let handle = child
            .raw_handle()
            .ok_or_else(|| io::Error::other("server exited before it could be contained"))?;
        // `RawHandle` is a pointer; the job API takes the same value as an integer.
        job.assign_process(handle as isize)
            .map_err(|e| io::Error::other(format!("job object: {e}")))?;
    }

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no stdin"))?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let (exit_tx, exited) = oneshot::channel();

    #[cfg(windows)]
    tokio::spawn(async move {
        let code = child.wait().await.ok().and_then(|s| s.code());
        let _ = exit_tx.send(code);
    });
    #[cfg(windows)]
    let process = ServerProcess {
        stdin,
        stdout,
        stderr,
        exited,
        job: Some(job),
    };

    #[cfg(not(windows))]
    let (kill_tx, kill_rx) = oneshot::channel::<()>();
    #[cfg(not(windows))]
    tokio::spawn(async move {
        // Unix `wait` is cancel-safe, so racing it against the kill switch is fine.
        let status = tokio::select! {
            s = child.wait() => s,
            _ = kill_rx => {
                let _ = child.start_kill();
                child.wait().await
            }
        };
        let _ = exit_tx.send(status.ok().and_then(|s| s.code()));
    });
    #[cfg(not(windows))]
    let process = ServerProcess {
        stdin,
        stdout,
        stderr,
        exited,
        kill: Some(kill_tx),
    };

    Ok(process)
}
