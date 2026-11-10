# 6. Windows sandbox details

Date: 2026-11-09 · Status: proposed (needs maintainer sign-off)

## Context
ADR 0002 chose Job Objects + AppContainer on Windows. The Foundations spike
(`crates/kb-runner/src/windows/`, attack suite in `crates/kb-runner/tests/attacks_windows.rs`)
showed several details that the plan did not cover.

## Decision
- `unsafe` is allowed only inside `kb-runner/src/windows/` (workspace lint stays `deny`);
  every block carries a `SAFETY:` comment. Bindings: `windows-sys`, pinned exactly.
- One Job Object per run: job-wide memory cap, job CPU-time cap, active-process cap,
  kill-on-close, die-on-unhandled-exception, UI restrictions, no breakaway. The child is
  placed in the job at creation (`PROC_THREAD_ATTRIBUTE_JOB_LIST`) and inherits only its
  three stdio handles (`PROC_THREAD_ATTRIBUTE_HANDLE_LIST`).
- The runner also enforces CPU time from job accounting every 20 ms, because the job's
  own limit is checked lazily (measured ~2x overshoot), plus a wall-clock limit.
- AppContainer `KataBox.Sandbox`, zero capabilities. CreateProcess requires a registered
  profile, and a profile brings a writable, persistent folder (`Packages\katabox.sandbox\AC`,
  where Windows points TEMP/TMP) and registry key. Every run revokes the SID's access to
  both (fail closed if missing), so the per-run work folder is the only writable place.
- Plain AppContainer, not LPAC, for now: python-build-standalone 3.14 exits with
  STATUS_ACCESS_DENIED at startup under LPAC.
- The child never inherits the host environment.

## Consequences
- Known gap: the sandbox can read files Windows grants to ALL APPLICATION PACKAGES
  (much of `C:\Windows`, `Program Files`). Recorded by `known_gap_system_files_readable_without_lpac`.
- Not yet proven: whether UDP datagrams are dropped before leaving the machine (no reply
  ever arrives; TCP fails immediately with WSAEACCES). Needs an elevated packet capture.
- conhost.exe runs inside the job but does not count toward the process limit
  (16 user processes + 1 console host).
- The job's peak-memory counter includes a refused allocation, so it can read above
  the cap; the judge must not report it as memory used without adjustment.
- Python prints "Failed to find real location of …python.exe" on stderr because the
  sandbox cannot resolve parent folders; the harness must filter or avoid it.
- Running the attack suite on Windows requires `scripts/fetch-python-toolchain.ps1` first.
