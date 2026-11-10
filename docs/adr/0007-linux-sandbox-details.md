# 7. Linux sandbox details

Date: 2026-11-09 · Status: proposed (needs maintainer sign-off: sandbox code)

## Context
ADR 0002 chose hakoniwa (namespaces, Landlock, seccomp, rlimits) on Linux with a
Landlock + seccomp fallback. The Linux side must keep the Windows API and rules from
ADR 0006 (same `SandboxSpec`, sandbox-root validation, work-folder lifecycle, one run at
a time). The spike (`crates/kb-runner/src/linux/`, attack suite in
`crates/kb-runner/tests/attacks_linux.rs`) turned up details the plan did not cover.

## Decision
- **No in-process fork.** hakoniwa runs Rust code between `fork` and `exec`, which can
  deadlock in a multithreaded process (the app, `cargo test`). Every run starts the
  `kb-sandbox` helper (shipped as a sidecar next to the app) in two stages:
  `__supervise` (child subreaper, detached from any terminal with `setsid`, pumps stdio,
  polls limits, kills the tree, reports JSON to the runner) and `__contain`
  (single-threaded; calls hakoniwa). Both die with their parent (`PR_SET_PDEATHSIG`,
  checked against the expected parent PID). No `unsafe` in `linux/`: nix, hakoniwa and
  landlock wrap every call. Pinned: hakoniwa `=1.8.1` (features `landlock`, `seccomp`),
  landlock `=0.4.7`, nix 0.29.
- **Two modes, chosen once, reported in `Outcome::mode`.** `SandboxSpec::linux_isolation`
  is `Auto` (default), `Namespaces` or `Fallback`. Auto uses namespaces when a real trial
  works (`__probe-userns`: new user, mount, network, IPC and UTS namespaces, id maps and a
  mount; Ubuntu's AppArmor restriction lets `unshare` succeed and fails later steps), else
  the fallback. A mode is never downgraded after a failure: the run errors. Namespace
  mode requires Landlock (any ABI) and seccomp.
- **Namespace mode:** user (own uid/gid mapped), mount, PID, network (no interface up),
  IPC and UTS namespaces. The root is an empty read-only tmp folder with read-only binds
  of each `read_only_dirs` entry and of the individual glibc core files (loader, libc,
  libm, libpthread, libdl, libutil, librt), a read-write bind of the work folder (the
  only writable path), a minimal `/dev` and the namespace's own `/proc`. All binds keep
  their host paths, so arguments that name files need no translation. Landlock applies
  the same rules on top. The program is PID 1: when it exits the kernel kills the rest.
- **Fallback mode:** no namespaces. Landlock (toolchain read+execute, work folder
  read/write/execute, glibc core files, `/dev/null`, `/dev/zero`, `/dev/urandom`,
  `/dev/random`; nothing else, not even `/proc`), seccomp, rlimits. **Refused unless the
  kernel has Landlock ABI 6 (signal and abstract-socket scoping, Linux 6.12+)**, with the
  error "fallback mode needs Landlock signal scoping (ABI 6, Linux 6.12+); kernel X has
  Landlock ABI N". `__contain` puts itself in a scope-only Landlock domain before calling
  hakoniwa, so the program (nested inside it) can signal only `__contain` and runc, never
  the supervisor, the app or anything else the user runs. The supervisor stays outside
  every domain. Extra seccomp denials in fallback, because Landlock below ABI 7 cannot
  scope them to a path: `socket` (all families: a pathname Unix socket such as the D-Bus
  session bus would otherwise be reachable), SysV IPC and POSIX message queues (names,
  not paths), and the chmod/chown/xattr families. The attack suite found the last one:
  without it the sandbox could `chmod 0777` the user's toolchain folder.
- **seccomp deny-list (both modes, EPERM):** ptrace, process_vm_readv/writev,
  process_madvise, pidfd_getfd, kcmp, perf_event_open, userfaultfd; the mount family
  (mount, umount2, pivot_root, chroot, move_mount, open_tree, fsopen, fsmount, fsconfig,
  fspick, mount_setattr), unshare, setns; bpf, keyctl, add_key, request_key, kexec_load,
  kexec_file_load, init/finit/delete_module, io_uring_*; reboot, swapon/off, acct,
  quotactl(_fd), syslog, settimeofday, clock_settime/adjtime, adjtimex, sethostname,
  setdomainname, vhangup, lookup_dcookie, uselib, nfsservctl, fanotify_init,
  open_by_handle_at, name_to_handle_at, iopl, ioperm, modify_ldt; pidfd_send_signal
  (denied outright: it takes a pidfd, there is no "pid ≤ 0" form). Conditional:
  kill, tkill, tgkill, rt_sigqueueinfo, rt_tgsigqueueinfo with pid ≤ 0 (low 32 bits zero
  or sign bit set, matching how the kernel reads `pid_t`); clone with any `CLONE_NEW*`
  flag; ioctl TIOCSTI/TIOCLINUX; prlimit64 on another process; socket of any family but
  `AF_UNIX` (namespace mode). clone3 returns ENOSYS so glibc falls back to clone, whose
  flags seccomp can read. Only the native architecture is in the filter, so another ABI
  (e.g. `int 0x80`) hits libseccomp's kill action.
- **Limits.** The supervisor polls every 20 ms: **CPU = user + kernel time of every live
  descendant (including their reaped children) + its own `RUSAGE_CHILDREN`**; wall clock;
  combined stdout + stderr. Any limit kills the whole tree (SIGKILL every descendant
  until none is left; orphans come back to the subreaper, so nothing leaves the tree).
  Reported CPU is the larger of the last sample and the final rusage. Backstops in the
  program: `RLIMIT_CPU` (limit + 1 s soft, +2 s hard), `RLIMIT_AS` and `RLIMIT_DATA` =
  `memory_bytes` (per process), `RLIMIT_NPROC`, `RLIMIT_FSIZE` 256 MiB, `RLIMIT_NOFILE`
  256, no core dumps, no locked memory, no message queues. The runner kills a supervisor
  that has not reported 10 s after the wall limit. Kill exit codes are the Windows ones
  (`0x4B42_0001..3`).
- **`RLIMIT_NPROC` counts threads (tasks), not processes.** In namespace mode it is
  counted per user *inside the new user namespace* (verified: only the sandbox's tasks
  count). In fallback it counts all the user's tasks, so the limit is the kernel's
  current count **measured by `__contain`** (it lowers its own soft limit and bisects for
  the smallest one at which spawning a no-op succeeds, ~10 ms) + `max_processes` + 16
  headroom. `/proc` alone undercounts: on this WSL machine ~90 of the user's tasks lived
  in sibling PID namespaces (other WSL distros), and the first version of the suite
  could not fork at all in fallback.
- **Validation** is the shared `validate.rs` (ADR 0006 rules), plus: `/proc`, `/sys`,
  `/dev` and `/run` are refused like the user profile. Mode choice and host-file lookup
  happen before the run; failing there leaves the work folder untouched.
- **Work-folder lifecycle.** Nothing is granted on disk on Linux (mounts and Landlock
  domains vanish with the processes), so after the supervisor reports every process dead
  the runner removes symlinks, FIFOs, sockets, device nodes and multiply-linked files
  without following them, clears setuid/setgid/sticky bits, makes folders owner-writable,
  then deletes the folder (or keeps it with `keep_work_dir`). A cleanup failure is an
  error, as on Windows. The status file and hakoniwa's root folder (mount points) live in
  a private 0700 folder of the supervisor, removed after every process is dead: with
  hakoniwa's default temp root, each run the supervisor had to kill left an empty
  skeleton in `/tmp` (79 from the first suite runs; 0 since).
- **One run at a time:** the same process-wide lock as Windows. Linux would not need it
  for isolation (each run has its own mount namespace or Landlock domain granting only
  its own folder), so parallel runs are a later, Linux-only option.
- **Environment:** `HOME` and `TMPDIR` = work folder, `LANG=C.UTF-8`, `PATH` = the
  program's folder, plus `spec.env`. Nothing from the host.
- **Test-only feature `unscoped-fallback-for-tests`.** Lets fallback run without the
  ABI 6 scope so the suite can test fallback's other layers on older kernels (this WSL
  machine, GitHub's runners). `compile_error!` in release builds; the app must never
  enable it.
- **Runtime dependency `libseccomp2`** (hakoniwa's `seccomp` feature links libseccomp;
  `libseccomp-dev` to build). See `docs/packaging.md`.
- **License:** hakoniwa is `LGPL-3.0-only WITH LGPL-3.0-linking-exception`, an approved
  exception in `deny.toml` (CI `cargo deny check licenses`). The exception allows static
  linking into the MIT/Apache app; changes to hakoniwa itself stay LGPL.

## Consequences
- Verified by the attack suite (25 attacks × 2 modes + Auto + probe = 52 tests, green
  three runs in a row on WSL 6.6 with `--features unscoped-fallback-for-tests`; without
  the feature every fallback test asserts the refusal). Measured:
  - CPU loop stopped at 2.02 s (limit 2 s), wall 2.04 s; a kernel-heavy loop (97% kernel
    time) stopped at 2.02 s; sleep stopped at the 3 s wall limit (3.02 s).
  - 2 GiB allocation refused (MemoryError); gradual allocation held < 256 MiB (peak RSS
    222 MiB); output flood stopped at 64 KiB in 45 ms.
  - Fork bomb: namespaces ≤ 15 live processes (16 tasks); fallback ≤ 31. Nothing
    survives either.
  - A double-forked `setsid` daemon dies with the run: namespaces when PID 1 exits
    (0.35 s); fallback at the wall limit (the subreaper keeps waiting for the orphan).
    No write happens after `run` returns.
  - Blocked in both modes: TCP/UDP/raw/netlink sockets (EPERM at `socket()`, so unlike
    Windows the datagram provably never leaves), DNS, abstract and pathname Unix
    sockets to host listeners, reading the secret sibling, the repo, `$HOME`,
    `/etc/passwd`, `/`, `/proc/<host pid>/environ`; writing the toolchain, the arena
    root, `/tmp`, `/dev/shm`, `$HOME`; chmod/chown/xattr on the toolchain;
    `/dev/tty`, TIOCSTI; SysV shared memory created on the host; every syscall in the
    deny-list (each checked for EPERM, clone3 for ENOSYS); kill(-1), kill(0), killpg,
    tgkill(0), tkill(-1), rt_sigqueueinfo(-1); killing a host process (namespaces: not
    visible). Symlinks to the secret/toolchain can be created but not followed; hard
    links to them fail; device nodes fail. After a deleted or kept run, a second run
    cannot read, list, write, append, delete or create in the first run's folder.
  - Concurrent `run` calls: B starts 39–53 ms after A ends.
- **Known gaps** (recorded, several with tests):
  - The glibc core files are readable (`known_gap_glibc_core_readable`); nothing else
    from the host library folders is (libstdc++ blocked).
  - Memory is limited per process (`RLIMIT_AS`), not per tree: the fallback fork bomb
    held 500 MiB RSS across 31 processes under a 256 MiB limit. `memory_limit_hit` is
    always false (refusals are not observable). **cgroups v2 (`memory.max`,
    `pids.max`, `cgroup.kill`) must land before the JS/TS runner** (see the spike below).
  - Fallback: the process cap is approximate (measured count + 16 headroom, other
    processes drift), CPU can overshoot by about (busy cores × 20 ms) + kill sweep
    (fork bomb: 2.9–3.1 s of CPU in 0.2 s wall on a 2 s limit), the whole chmod family
    and all sockets are denied (also inside the work folder), and if the app is
    SIGKILLed the program's orphans outlive it (still confined; `RLIMIT_CPU` ends
    busy ones). In namespace mode the PID namespace makes the app's death fatal to all.
  - Fallback: the program can signal `__contain` and runc (same Landlock domain). That
    only ends its own run, which is reported as an error.
  - `total_processes`, `max_active_processes` and `process_limit_hit` are inferred from
    20 ms samples (a sample never sees the refused task, so "observed tasks + 1 ≥
    limit" counts as a hit). `peak_memory_bytes` is the highest sampled RSS sum or the
    largest single-process peak.
  - Setup failures inside the final child (Landlock or seccomp load) show up as exit
    code 125 with `hakoniwa: …` on stderr, not as a setup error. Fail closed: the
    program never runs.
  - The supervisor's kill sweep has a theoretical PID-reuse race (a PID freed and reused
    between listing and SIGKILL); it needs the PID space to wrap within milliseconds.
  - Debuggers will need loopback TCP; namespace mode can bring up `lo` inside its own
    network namespace when that work starts.
- **Fallback cannot run on this WSL machine or on GitHub's runners** (kernels < 6.12).
  CI runs fallback with the test feature on `ubuntu-24.04` (AppArmor restricts user
  namespaces there, so Auto must pick fallback), checks the refusal without it, then
  lifts the restriction for namespace mode.
- Linux attack-suite setup: `scripts/fetch-python-toolchain.sh` (same pinned
  python-build-standalone 3.14.10+20261026 as Windows, SHA-256 from the release's
  `SHA256SUMS`; it also creates the marked dev root `target/katabox`).

### What this WSL machine supports (`kb-sandbox probe`)
| | |
| --- | --- |
| Kernel | 6.6.87.2-microsoft-standard-WSL2 (Ubuntu 24.04.5 userland) |
| Landlock | ABI 3: no TCP rules (4), no ioctl rules (5), **no signal/abstract-socket scoping (6)** → fallback refused |
| User namespaces | Work (`max_user_namespaces` 71889, no AppArmor LSM, so no Ubuntu restriction) → Auto = namespaces |
| seccomp | Available (libseccomp 2.5.5) |
| cgroup v2 | Unified; cpu, memory and pids delegated to `user@1000.service`, but a WSL shell runs in `/init.scope` (not writable); reaching a delegated cgroup needs `systemd-run --user --scope` (tested: works) |
| Yama | `ptrace_scope` 1 |

### Spike: other runtimes under these limits (`prlimit` + `unshare`, same mechanisms)
- **Node (v20.19.6, the local nvm build; v24 LTS not downloaded):** `RLIMIT_AS` ≤ 700 MiB
  kills startup ("Fatal process OOM in Failed to reserve virtual memory for CodeRange");
  ≥ 800 MiB starts. `RLIMIT_DATA` 128 MiB–1 GiB starts, and heap growth past it is fatal
  (capped). But **`RLIMIT_DATA` does not count shared mappings**: Python touched 1 GiB of
  `MAP_SHARED|MAP_ANONYMOUS` memory under a 256 MiB `RLIMIT_DATA` (`RLIMIT_AS` 256 MiB
  refuses it). So no rlimit gives JS a real memory cap: either V8 cannot start or the
  limit is bypassable. Hence cgroups before the JS/TS runner.
- **rustc 1.99.1** links through `cc -fuse-ld=lld` (multithreaded rust-lld). A small
  program: 0.22 s, 119 MB peak RSS. In a user namespace, success rates: `RLIMIT_AS`
  2 GiB 3/6, 4 GiB 6/6; `RLIMIT_NPROC` (threads) 16: 2/6, 32: 6/6; `RLIMIT_DATA` 512 MiB
  3/6 (failures are lld's `std::system_error` creating threads). The compile step
  therefore needs its own limits (≥ 4 GiB address space, ≥ 32 tasks, CPU ~20 s), and its
  sandbox needs the host C toolchain (`cc`, crt objects, libc development files)
  readable. That is the Rust runner's design work.
