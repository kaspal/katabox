# 6. Windows sandbox details

Date: 2026-11-09 · Status: proposed (needs maintainer sign-off)

## Context
ADR 0002 chose Job Objects + AppContainer on Windows. The Foundations spike
(`crates/kb-runner/src/windows/`, attack suite in `crates/kb-runner/tests/attacks_windows.rs`)
showed several details that the plan did not cover. Review of the spike added the
work-folder lifecycle, the sandbox root, and kernel time in the CPU limit.

## Decision
- `unsafe` is allowed only inside `kb-runner/src/windows/` (workspace lint stays `deny`);
  every block carries a `SAFETY:` comment. Bindings: `windows-sys`, pinned exactly.
- One Job Object per run: job-wide memory cap, job CPU-time cap, active-process cap,
  kill-on-close, die-on-unhandled-exception, UI restrictions, no breakaway. The child is
  placed in the job at creation (`PROC_THREAD_ATTRIBUTE_JOB_LIST`) and inherits only its
  three stdio handles (`PROC_THREAD_ATTRIBUTE_HANDLE_LIST`). Only the function that owns
  the job holds its handle, so every return path closes it and kills what is left.
- **CPU limit = user + kernel time**, enforced by the runner from job accounting every
  20 ms. The job's own limit (user time only, checked lazily: ~2x overshoot measured) is
  kept as a backstop. A wall-clock limit catches programs that use no CPU.
- **Sandbox root.** Every spec names a KataBox-owned root that must contain the marker
  file `.katabox-sandbox-root` (the app data folder in the product, `target/katabox` in
  dev/CI). Before anything is created or granted, all paths are canonicalized (links
  resolved) and the spec is rejected unless `work_dir` and every read-only folder are
  strictly inside the root, do not overlap, and the program lives in a read-only folder.
  Drive roots, network paths, and the user profile or any folder containing it are
  refused for the root and for every folder. A work folder that already contains a
  link/reparse point is refused.
- **Work-folder lifecycle.** The work folder is the only writable place and only for
  the duration of one run. After the job is closed (all processes dead), the runner
  removes any links the sandbox created (never following them), revokes the run's grant,
  and deletes the folder. With `keep_work_dir` the folder stays, but every explicit
  permission inside it is reset to inherited. If deletion fails, the reset still runs and
  the run returns an error. Read-only grants (toolchains) persist between runs.
- AppContainer `KataBox.Sandbox`, zero capabilities. CreateProcess requires a registered
  profile, and a profile brings a writable, persistent folder (`Packages\katabox.sandbox\AC`,
  where Windows points TEMP/TMP) and registry key. Every run revokes the SID's access to
  both (fail closed if missing).
- Plain AppContainer, not LPAC, for now: python-build-standalone 3.14 exits with
  STATUS_ACCESS_DENIED at startup under LPAC.
- The child never inherits the host environment.

## Consequences
- Verified by the attack suite (19 tests): a second run cannot read, list, write, append,
  delete or create in a previous run's folder, whether it was deleted or kept; specs
  pointing at the user profile, its parent, `C:\`, a folder outside the root, or a
  junction to the profile are rejected without touching any ACL; a kernel-heavy loop
  (96% kernel time) is stopped at the 2 s CPU limit.
- Observed Windows behavior the design relies on but does not control: a sandboxed
  process can create a junction only to a folder it can write to (attempts at the secret
  folder and the toolchain fail with access denied, even with a raw
  FSCTL_SET_REPARSE_POINT), and it cannot change ACLs on its own files (`icacls /grant`
  is denied). Cleanup does not depend on either: it never follows links and resets
  permissions anyway.
- **Concurrent runs share one AppContainer SID**, so two runs active at the same time can
  reach each other's work folders. Run submissions one at a time, or move to one
  AppContainer profile per run before allowing parallel runs.
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
- A cleanup failure turns the run into an error (results are dropped rather than
  leaving a folder in an unknown state).
- Running the attack suite on Windows requires `scripts/fetch-python-toolchain.ps1`
  first; it creates the marked dev root `target/katabox`.
