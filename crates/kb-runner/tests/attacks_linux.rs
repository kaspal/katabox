//! Attack suite for the Linux sandbox (ADR 0002, 0007). Every attack must be stopped or
//! blocked, in BOTH modes: each test below exists as `<name>::namespaces` and
//! `<name>::fallback`. Needs the dev sandbox root with the pinned Python toolchain:
//!
//! ```text
//! scripts/fetch-toolchains.sh             # creates target/katabox
//! cargo test -p kb-runner --test attacks_linux -- --nocapture --test-threads=1
//! ```
//!
//! Fallback mode is refused on kernels without Landlock signal scoping (ABI 6, Linux
//! 6.12+). There, each `fallback` test asserts that refusal instead, unless the suite is
//! built with `--features unscoped-fallback-for-tests`, which lets fallback run without
//! the scope so its other layers can be tested; the signal tests then record the gap.
//!
//! Tests run one at a time (SERIAL) so CPU-time measurements are not skewed.
#![cfg(target_os = "linux")]

use std::net::TcpListener;
use std::os::unix::fs::{MetadataExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, Once};
use std::time::Duration;

use kb_core::{Language, Limits};
use kb_runner::{
    LinuxIsolation, Outcome, ROOT_MARKER, SandboxError, SandboxMode, SandboxSpec, Termination,
    linux_probe, run, set_helper_path,
};

static SERIAL: Mutex<()> = Mutex::new(());
static HELPER: Once = Once::new();

const MB: u64 = 1024 * 1024;

/// Shared Python helper: `t(label, fn)` prints `label OK ...` or `label ERR <errno> <type>`.
const PRELUDE: &str = r#"
import os, sys
def t(label, fn):
    try:
        r = fn()
        print(label, "OK", repr(r)[:80], flush=True)
    except BaseException as e:
        print(label, "ERR", getattr(e, "errno", None), type(e).__name__, flush=True)
import ctypes
libc = ctypes.CDLL(None, use_errno=True)
libc.syscall.restype = ctypes.c_long
def sc(nr, *args):
    r = libc.syscall(ctypes.c_long(nr), *[ctypes.c_long(a) if isinstance(a, int) else a for a in args])
    if r == -1:
        e = ctypes.get_errno()
        raise OSError(e, os.strerror(e))
    return r
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Ns,
    Fb,
}

impl Mode {
    fn isolation(self) -> LinuxIsolation {
        match self {
            Mode::Ns => LinuxIsolation::Namespaces,
            Mode::Fb => LinuxIsolation::Fallback,
        }
    }
    fn expected(self) -> SandboxMode {
        match self {
            Mode::Ns => SandboxMode::Namespaces,
            Mode::Fb => SandboxMode::Fallback,
        }
    }
}

fn setup() {
    HELPER.call_once(|| set_helper_path(env!("CARGO_BIN_EXE_kb-sandbox")));
}

fn signal_scoping() -> bool {
    setup();
    linux_probe().unwrap().signal_scoping()
}

/// Fallback runs here: either the kernel scopes signals, or the test-only feature is on.
fn fallback_runnable() -> bool {
    signal_scoping() || cfg!(feature = "unscoped-fallback-for-tests")
}

/// Run `$f(Mode)` as `$f::namespaces` and `$f::fallback`.
macro_rules! both_modes {
    ($($f:ident),* $(,)?) => {$(
        mod $f {
            #[test]
            fn namespaces() {
                let _s = super::SERIAL.lock().unwrap_or_else(|e| e.into_inner());
                super::$f(super::Mode::Ns);
            }
            #[test]
            fn fallback() {
                let _s = super::SERIAL.lock().unwrap_or_else(|e| e.into_inner());
                if super::fallback_runnable() {
                    super::$f(super::Mode::Fb);
                } else {
                    super::assert_fallback_refused(stringify!($f));
                }
            }
        }
    )*};
}

/// The KataBox-owned dev sandbox root (`target/katabox`, or `KB_TEST_ROOT`).
fn sandbox_root() -> PathBuf {
    setup();
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/katabox"));
    let root = std::fs::canonicalize(&root).unwrap_or(root);
    assert!(
        root.join(ROOT_MARKER).is_file() && python_in(&root).is_file(),
        "sandbox root or Python toolchain missing at {}. Run: scripts/fetch-toolchains.sh",
        root.display()
    );
    root
}

fn toolchain_in(root: &Path) -> PathBuf {
    root.join("toolchains").join("python-3.14")
}

fn python_in(root: &Path) -> PathBuf {
    toolchain_in(root).join("bin").join("python3")
}

fn toolchain() -> PathBuf {
    toolchain_in(&sandbox_root())
}

fn home() -> PathBuf {
    std::fs::canonicalize(std::env::var_os("HOME").unwrap()).unwrap()
}

/// Per-test folder `<root>/runs/<name>-<pid>`: `work/` is the sandbox's only writable
/// folder (deleted by the runner after each run), `secret/` is a sibling it must not see.
struct Arena {
    root: PathBuf,
    work: PathBuf,
    secret: PathBuf,
    mode: Mode,
}

impl Arena {
    fn new(name: &str, mode: Mode) -> Self {
        let root = sandbox_root()
            .join("runs")
            .join(format!("{name}-{mode:?}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let work = root.join("work");
        let secret = root.join("secret");
        std::fs::create_dir_all(&work).unwrap();
        std::fs::create_dir_all(&secret).unwrap();
        std::fs::write(secret.join("canary.txt"), "TOP SECRET").unwrap();
        Arena {
            root,
            work,
            secret,
            mode,
        }
    }

    /// A fresh, empty work folder next to `work/`.
    fn new_work(&self, name: &str) -> PathBuf {
        let dir = self.root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn python(&self, code: &str, args: &[&str]) -> SandboxSpec {
        self.python_in(&self.work, code, args)
    }

    fn python_in(&self, work: &Path, code: &str, args: &[&str]) -> SandboxSpec {
        let root = sandbox_root();
        let mut spec = SandboxSpec::new(
            &root,
            python_in(&root),
            work,
            &Limits::default_for(Language::Python),
        );
        let mut full = String::from(PRELUDE);
        full.push_str(code);
        spec.args = ["-I", "-B", "-c", &full]
            .iter()
            .map(|s| s.to_string())
            .collect();
        spec.args.extend(args.iter().map(|s| s.to_string()));
        spec.read_only_dirs = vec![toolchain_in(&root)];
        spec.linux_isolation = self.mode.isolation();
        spec
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let _ = make_writable(&self.root);
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn make_writable(p: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let meta = std::fs::symlink_metadata(p)?;
    if meta.is_dir() {
        std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o700))?;
        for e in std::fs::read_dir(p)? {
            make_writable(&e?.path())?;
        }
    }
    Ok(())
}

fn assert_fallback_refused(name: &str) {
    let a = Arena::new(name, Mode::Fb);
    match run(&a.python("print('must not run')\n", &[])) {
        Err(SandboxError::Unavailable(msg)) => {
            assert!(msg.contains("Landlock signal scoping"), "{msg}");
            eprintln!("[{name}] fallback refused as required: {msg}");
        }
        other => panic!("fallback without signal scoping was not refused: {other:?}"),
    }
    assert!(
        a.work.is_dir(),
        "a refused run must not touch the work folder"
    );
}

fn expect_rejected(name: &str, spec: &SandboxSpec) {
    match run(spec) {
        Err(SandboxError::InvalidSpec(msg)) => eprintln!("[{name}] rejected: {msg}"),
        other => panic!("[{name}] spec was not rejected: {other:?}"),
    }
}

fn go(name: &str, spec: &SandboxSpec) -> Outcome {
    let o = run(spec).expect("sandbox run failed");
    let stdout: String = String::from_utf8_lossy(&o.stdout)
        .chars()
        .take(900)
        .collect();
    let stderr = String::from_utf8_lossy(&o.stderr);
    let stderr_tail: String = stderr.lines().rev().take(3).collect::<Vec<_>>().join(" | ");
    eprintln!(
        "\n[{name}] mode={:?} termination={:?} exit={:#x} user={:?} kernel={:?} wall={:?} peak={} MiB \
         procs(total={} max_active={} after={}) mem_hit={} proc_hit={}\n  stdout: {}\n  stderr(last): {}",
        o.mode,
        o.termination,
        o.exit_code,
        o.user_time,
        o.kernel_time,
        o.wall_time,
        o.peak_memory_bytes / MB,
        o.total_processes,
        o.max_active_processes,
        o.active_processes_after,
        o.memory_limit_hit,
        o.process_limit_hit,
        stdout.trim().replace('\n', "\n          "),
        stderr_tail,
    );
    assert_eq!(o.active_processes_after, 0, "processes survived the run");
    if spec.linux_isolation != LinuxIsolation::Auto {
        let want = match spec.linux_isolation {
            LinuxIsolation::Namespaces => SandboxMode::Namespaces,
            _ => SandboxMode::Fallback,
        };
        assert_eq!(o.mode, want, "wrong mode reported");
    }
    o
}

fn line(o: &Outcome, label: &str) -> String {
    let out = String::from_utf8_lossy(&o.stdout).into_owned();
    out.lines()
        .find(|l| l.starts_with(&format!("{label} ")))
        .unwrap_or_else(|| {
            panic!(
                "no `{label}` line in stdout:\n{out}\nstderr:\n{}",
                String::from_utf8_lossy(&o.stderr)
            )
        })
        .to_string()
}

fn assert_blocked(o: &Outcome, label: &str) {
    let l = line(o, label);
    assert!(
        l.starts_with(&format!("{label} ERR")),
        "attack `{label}` was NOT blocked: {l}"
    );
}

/// Blocked with a specific errno (e.g. 1 = EPERM from seccomp).
fn assert_errno(o: &Outcome, label: &str, errno: i32) {
    let l = line(o, label);
    assert!(
        l.starts_with(&format!("{label} ERR {errno} ")),
        "attack `{label}` was not blocked with errno {errno}: {l}"
    );
}

fn assert_allowed(o: &Outcome, label: &str) {
    let l = line(o, label);
    assert!(
        l.starts_with(&format!("{label} OK")),
        "control `{label}` unexpectedly failed: {l}"
    );
}

const EPERM: i32 = 1;

both_modes!(
    baseline_print_1,
    infinite_loop_hits_cpu_limit,
    sleep_hits_wall_limit,
    allocate_2gb_is_refused,
    gradual_allocation_is_capped,
    fork_bomb_is_capped_and_cleaned_up,
    network_loopback_is_blocked,
    network_internet_and_dns_are_blocked,
    reading_outside_allowed_folders_is_blocked,
    writing_outside_work_folder_is_blocked,
    daemonized_child_dies_with_the_run,
    host_environment_does_not_leak,
    output_flood_is_cut_off,
    known_gap_glibc_core_readable,
    kernel_time_counts_toward_cpu_limit,
    work_dir_is_deleted_and_second_run_cannot_touch_it,
    kept_work_dir_is_sealed_against_later_runs,
    concurrent_run_calls_never_overlap,
    spec_pointing_at_user_profile_is_rejected,
    work_dir_with_preexisting_link_is_rejected,
    // Linux-specific.
    unix_sockets_cannot_reach_the_host,
    dangerous_syscalls_are_denied,
    signals_cannot_reach_other_processes,
    terminal_is_unreachable,
    sysv_ipc_of_the_host_is_unreachable,
);

// --- Ports of the 20 Windows attacks ----------------------------------------------

fn baseline_print_1(m: Mode) {
    let a = Arena::new("baseline", m);
    let mut spec = a.python("", &[]);
    spec.args = vec!["-I".into(), "-B".into(), "-c".into(), "print(1)".into()];
    let o = go("baseline", &spec);
    assert_eq!(o.termination, Termination::Exited);
    assert_eq!(o.exit_code, 0);
    assert_eq!(o.mode, m.expected());
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "1");
    assert_eq!(o.total_processes, 1);
}

fn infinite_loop_hits_cpu_limit(m: Mode) {
    let a = Arena::new("loop", m);
    let spec = a.python("while True:\n    pass\n", &[]); // cpu 2 s, wall 6 s
    let o = go("infinite-loop", &spec);
    assert_eq!(o.termination, Termination::CpuTimeLimit);
    let cpu = o.user_time + o.kernel_time;
    assert!(
        cpu >= Duration::from_millis(1_900),
        "killed too early: {cpu:?}"
    );
    assert!(
        cpu < Duration::from_millis(2_500),
        "CPU limit overshot: {cpu:?}"
    );
    assert!(
        o.wall_time < Duration::from_secs(4),
        "took too long: {:?}",
        o.wall_time
    );
}

fn sleep_hits_wall_limit(m: Mode) {
    let a = Arena::new("sleep", m);
    let mut spec = a.python("import time\ntime.sleep(60)\n", &[]);
    spec.wall_time = Duration::from_secs(3);
    let o = go("sleep", &spec);
    assert_eq!(o.termination, Termination::WallTimeLimit);
    assert!(
        o.wall_time < Duration::from_secs(4),
        "took too long: {:?}",
        o.wall_time
    );
}

fn allocate_2gb_is_refused(m: Mode) {
    let a = Arena::new("alloc2g", m);
    let spec = a.python("t('alloc2g', lambda: len(bytearray(2 * 1024**3)))\n", &[]);
    let o = go("alloc-2GB", &spec);
    assert_blocked(&o, "alloc2g");
    assert!(line(&o, "alloc2g").contains("MemoryError"));
    // Not observable through rlimits (ADR 0007); stays false until cgroups.
    assert!(!o.memory_limit_hit);
}

fn gradual_allocation_is_capped(m: Mode) {
    let a = Arena::new("allocslow", m);
    let code = "keep = []\ndef grow():\n    while True:\n        keep.append(bytearray(16 * 1024 * 1024))\nt('grow', grow)\nprint('held MiB', 16 * len(keep))\n";
    let spec = a.python(code, &[]);
    let o = go("alloc-gradual", &spec);
    assert_blocked(&o, "grow");
    let held: u64 = line(&o, "held")
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    assert!(held * MB <= spec.memory_bytes, "held {held} MiB");
    assert!(
        o.peak_memory_bytes <= spec.memory_bytes,
        "peak {} MiB",
        o.peak_memory_bytes / MB
    );
}

fn fork_bomb_is_capped_and_cleaned_up(m: Mode) {
    let a = Arena::new("forkbomb", m);
    // Every process spawns copies of itself forever.
    let bomb = "import subprocess, sys, time\ncode = sys.argv[1]\nwhile True:\n    try:\n        subprocess.Popen([sys.executable, '-I', '-B', '-c', code, code])\n    except OSError:\n        time.sleep(0.01)\n";
    let mut spec = a.python("", &[]);
    spec.args = ["-I", "-B", "-c", bomb, bomb]
        .iter()
        .map(|s| s.to_string())
        .collect();
    spec.wall_time = Duration::from_secs(5);
    spec.max_output_bytes = 4 * MB as usize; // don't let stderr noise end the test early
    let o = go("fork-bomb", &spec);
    assert!(o.process_limit_hit, "process limit never triggered");
    // Namespaces: RLIMIT_NPROC counts only the sandbox's tasks. Fallback: it counts all
    // of the user's tasks plus 16 headroom, so the cap is approximate (ADR 0007).
    let slack = if m == Mode::Ns { 0 } else { 16 };
    assert!(
        o.max_active_processes <= spec.max_processes + slack,
        "saw {} processes",
        o.max_active_processes
    );
    assert_ne!(o.termination, Termination::Exited);
}

fn network_loopback_is_blocked(m: Mode) {
    let a = Arena::new("loopback", m);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let code = "import socket\nport = int(sys.argv[1])\nt('tcp-loopback', lambda: socket.create_connection(('127.0.0.1', port), timeout=3).close())\n";
    let o = go("net-loopback", &a.python(code, &[&port]));
    assert_blocked(&o, "tcp-loopback");
    assert!(
        listener.accept().is_err(),
        "the listener received a connection from the sandbox"
    );
}

fn network_internet_and_dns_are_blocked(m: Mode) {
    let a = Arena::new("internet", m);
    let code = r#"import socket
t('tcp-1.1.1.1:443', lambda: socket.create_connection(('1.1.1.1', 443), timeout=3).close())
t('dns-example.com', lambda: socket.getaddrinfo('example.com', 443))
def udp():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.settimeout(2)
    s.sendto(b'\x12\x34\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07example\x03com\x00\x00\x01\x00\x01', ('1.1.1.1', 53))
    return s.recvfrom(512)
t('udp-1.1.1.1:53', udp)
t('raw-icmp', lambda: socket.socket(socket.AF_INET, socket.SOCK_RAW, socket.IPPROTO_ICMP))
t('netlink', lambda: socket.socket(socket.AF_NETLINK, socket.SOCK_RAW, 0))
"#;
    let o = go("net-internet", &a.python(code, &[]));
    // Unlike Windows, the datagram never leaves: socket() itself fails with EPERM.
    for label in ["tcp-1.1.1.1:443", "udp-1.1.1.1:53", "raw-icmp", "netlink"] {
        assert_errno(&o, label, EPERM);
    }
    assert_blocked(&o, "dns-example.com");
}

fn reading_outside_allowed_folders_is_blocked(m: Mode) {
    let a = Arena::new("readout", m);
    let canary = a.secret.join("canary.txt");
    let cargo_toml =
        std::fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml"))
            .unwrap();
    let home = home();
    let environ = format!("/proc/{}/environ", std::process::id());
    let code = r#"
t('read-canary', lambda: open(sys.argv[1]).read())
t('read-repo-Cargo.toml', lambda: open(sys.argv[2]).read())
t('list-home', lambda: os.listdir(sys.argv[3]))
t('list-secret-dir', lambda: os.listdir(sys.argv[4]))
t('read-host-proc-environ', lambda: open(sys.argv[5], 'rb').read())
t('read-etc-passwd', lambda: open('/etc/passwd').read())
t('list-root', lambda: os.listdir('/'))
t('read-own-work-file', lambda: (open('mine.txt', 'w').write('ok'), open('mine.txt').read()))
"#;
    let o = go(
        "read-outside",
        &a.python(
            code,
            &[
                canary.to_str().unwrap(),
                cargo_toml.to_str().unwrap(),
                home.to_str().unwrap(),
                a.secret.to_str().unwrap(),
                &environ,
            ],
        ),
    );
    for label in [
        "read-canary",
        "read-repo-Cargo.toml",
        "list-home",
        "list-secret-dir",
        "read-host-proc-environ",
        "read-etc-passwd",
        "list-root",
    ] {
        assert_blocked(&o, label);
    }
    assert_allowed(&o, "read-own-work-file"); // positive control: the work folder works
}

fn writing_outside_work_folder_is_blocked(m: Mode) {
    let a = Arena::new("writeout", m);
    let tc = toolchain();
    let code = r#"
t('write-toolchain', lambda: open(os.path.join(sys.argv[1], 'pwned.txt'), 'w').write('x'))
t('write-toolchain-lib', lambda: open(os.path.join(sys.argv[1], 'lib', 'python3.14', 'os.py'), 'a').write('#'))
t('chmod-toolchain', lambda: os.chmod(os.path.join(sys.argv[1], 'bin'), 0o777))
t('write-arena-root', lambda: open(os.path.join(sys.argv[2], 'pwned.txt'), 'w').write('x'))
t('write-tmp', lambda: open('/tmp/kb-pwned.txt', 'w').write('x'))
t('write-dev-shm', lambda: open('/dev/shm/kb-pwned', 'w').write('x'))
t('write-home', lambda: open(os.path.join(sys.argv[3], 'kb-pwned.txt'), 'w').write('x'))
t('chown-toolchain', lambda: os.chown(os.path.join(sys.argv[1], 'bin'), os.getuid(), os.getgid()))
t('xattr-toolchain', lambda: os.setxattr(os.path.join(sys.argv[1], 'bin'), 'user.kb', b'x'))
t('chmod-work', lambda: (open('m.txt', 'w').write('x'), os.chmod('m.txt', 0o600)))
t('write-work', lambda: open('ok.txt', 'w').write('x'))
"#;
    let home = home();
    let o = go(
        "write-outside",
        &a.python(
            code,
            &[
                tc.to_str().unwrap(),
                a.root.to_str().unwrap(),
                home.to_str().unwrap(),
            ],
        ),
    );
    for label in [
        "write-toolchain",
        "write-toolchain-lib",
        "chmod-toolchain",
        "write-arena-root",
        "write-tmp",
        "write-dev-shm",
        "write-home",
        "chown-toolchain",
        "xattr-toolchain",
    ] {
        assert_blocked(&o, label);
    }
    assert_allowed(&o, "write-work");
    // Namespaces allow metadata changes inside the work folder; the fallback denies the
    // whole chmod family (Landlock cannot scope it to a path).
    match m {
        Mode::Ns => assert_allowed(&o, "chmod-work"),
        Mode::Fb => assert_errno(&o, "chmod-work", EPERM),
    }
    let bin_mode = std::fs::metadata(tc.join("bin")).unwrap().mode() & 0o7777;
    assert_eq!(
        bin_mode & 0o022,
        0,
        "toolchain bin became group/world-writable: {bin_mode:o}"
    );
    assert!(!tc.join("pwned.txt").exists());
    assert!(!home.join("kb-pwned.txt").exists());
    assert!(!Path::new("/tmp/kb-pwned.txt").exists());
}

/// Port of `job_breakaway_is_refused`: Linux has no breakaway flag; the escape attempt is
/// a double-forked, setsid() daemon that outlives its parent. It must die with the run.
fn daemonized_child_dies_with_the_run(m: Mode) {
    let a = Arena::new("daemon", m);
    let code = r#"import time
if os.fork() == 0:
    os.setsid()
    if os.fork() == 0:
        while True:
            with open('alive.txt', 'a') as f:
                f.write('x')
            time.sleep(0.05)
    os._exit(0)
time.sleep(0.3)
print('parent exits', flush=True)
"#;
    let mut spec = a.python(code, &[]);
    spec.keep_work_dir = true;
    spec.wall_time = Duration::from_secs(2);
    let o = go("daemon", &spec);
    // Namespaces: PID 1 exiting kills the namespace. Fallback: the supervisor (subreaper)
    // keeps waiting for the orphan and kills it at the wall limit.
    match m {
        Mode::Ns => assert_eq!(o.termination, Termination::Exited),
        Mode::Fb => assert_eq!(o.termination, Termination::WallTimeLimit),
    }
    let size = || {
        std::fs::metadata(a.work.join("alive.txt"))
            .map(|m| m.len())
            .unwrap_or(0)
    };
    let before = size();
    assert!(before > 0, "daemon never ran");
    std::thread::sleep(Duration::from_millis(500));
    assert_eq!(
        size(),
        before,
        "the daemon is still writing after the run returned"
    );
}

fn host_environment_does_not_leak(m: Mode) {
    let a = Arena::new("env", m);
    let o = go(
        "env",
        &a.python("print('ENV', ' '.join(sorted(os.environ)))\n", &[]),
    );
    let keys: Vec<String> = line(&o, "ENV")
        .split_whitespace()
        .skip(1)
        .map(str::to_string)
        .collect();
    let allowed = ["HOME", "LANG", "LC_CTYPE", "PATH", "TMPDIR"];
    for k in &keys {
        assert!(
            allowed.contains(&k.as_str()),
            "unexpected variable {k} in sandbox env"
        );
    }
}

fn output_flood_is_cut_off(m: Mode) {
    let a = Arena::new("flood", m);
    let spec = a.python("while True:\n    sys.stdout.write('x' * 4096)\n", &[]);
    let o = go("output-flood", &spec);
    assert_eq!(o.termination, Termination::OutputLimit);
    assert!(o.stdout.len() <= spec.max_output_bytes);
}

/// Known gap, recorded rather than hidden (Windows has the same class of gap): the
/// dynamic loader and the glibc core libraries are readable, because the runtimes link
/// against them. Nothing else from the host's library folders is. If this starts
/// failing, the gap closed: flip it.
fn known_gap_glibc_core_readable(m: Mode) {
    let a = Arena::new("libread", m);
    let libc = [
        "/lib/x86_64-linux-gnu/libc.so.6",
        "/lib64/libc.so.6",
        "/lib/aarch64-linux-gnu/libc.so.6",
    ]
    .into_iter()
    .find(|p| Path::new(p).is_file())
    .unwrap();
    let other = Path::new(libc).with_file_name("libstdc++.so.6");
    let code = "t('read-libc', lambda: len(open(sys.argv[1], 'rb').read(64)))\nt('read-libstdc++', lambda: len(open(sys.argv[2], 'rb').read(64)))\n";
    let o = go(
        "known-gap-libc",
        &a.python(code, &[libc, other.to_str().unwrap()]),
    );
    assert_allowed(&o, "read-libc");
    if other.is_file() {
        assert_blocked(&o, "read-libstdc++");
    }
}

fn kernel_time_counts_toward_cpu_limit(m: Mode) {
    let a = Arena::new("kernelburn", m);
    // Rewriting 1 MiB in the page cache is mostly kernel time; a limit on user time alone
    // would let this run until the 6 s wall-clock limit.
    let code = "fd = os.open('burn.bin', os.O_RDWR | os.O_CREAT)\nbuf = b'x' * (1 << 20)\nwhile True:\n    os.lseek(fd, 0, 0)\n    os.write(fd, buf)\n";
    let o = go("kernel-burn", &a.python(code, &[]));
    assert_eq!(o.termination, Termination::CpuTimeLimit);
    let cpu = o.user_time + o.kernel_time;
    assert!(
        o.kernel_time >= Duration::from_millis(1_000),
        "test did not burn kernel time: {:?}",
        o.kernel_time
    );
    assert!(
        cpu >= Duration::from_millis(1_900),
        "killed too early: {cpu:?}"
    );
    assert!(
        cpu < Duration::from_millis(2_500),
        "CPU limit overshot: {cpu:?}"
    );
    assert!(
        o.wall_time < Duration::from_millis(3_500),
        "took too long: {:?}",
        o.wall_time
    );
}

// --- Nothing stays reachable between runs -------------------------------------------

/// Python that leaves things behind: nested files, a read-only file, a setuid file, a
/// folder it made unwritable, symlinks (to the secret folder = argv[1], the toolchain =
/// argv[2], and back into itself), hard links to files outside, a FIFO and a socket.
const LEAVE_TRACES: &str = r#"
open('run1.txt', 'w').write('written by run 1')
os.makedirs(os.path.join('sub', 'deeper'))
open(os.path.join('sub', 'deeper', 'inner.txt'), 'w').write('x')
import stat
open('readonly.txt', 'w').write('x')
t('chmod-readonly', lambda: os.chmod('readonly.txt', stat.S_IREAD))
t('setuid-own-file', lambda: (open('suid', 'w').write('x'), os.chmod('suid', 0o4755)))
t('symlink-to-secret', lambda: os.symlink(sys.argv[1], 'link-secret'))
t('symlink-to-toolchain', lambda: os.symlink(sys.argv[2], 'link-tool'))
t('symlink-to-self', lambda: os.symlink(os.path.abspath('sub'), 'link-self'))
t('read-via-symlink-secret', lambda: open(os.path.join('link-secret', 'canary.txt')).read())
t('write-via-symlink-toolchain', lambda: open(os.path.join('link-tool', 'pwned.txt'), 'w').write('x'))
t('hardlink-secret', lambda: os.link(os.path.join(sys.argv[1], 'canary.txt'), 'hard-canary'))
t('hardlink-toolchain', lambda: os.link(os.path.join(sys.argv[2], 'bin', 'python3.14'), 'hard-python'))
t('mkfifo', lambda: os.mkfifo('fifo'))
import socket
t('bind-unix-socket', lambda: socket.socket(socket.AF_UNIX).bind('sock'))
t('mknod-device', lambda: os.mknod('null2', stat.S_IFCHR | 0o666, os.makedev(1, 3)))
os.makedirs('locked/inner'); open('locked/inner/f', 'w').write('x')
t('chmod-lock-dir', lambda: os.chmod('locked', 0o500))
"#;

fn assert_link_escapes_blocked(o: &Outcome) {
    assert_blocked(o, "read-via-symlink-secret");
    assert_blocked(o, "write-via-symlink-toolchain");
    assert_blocked(o, "hardlink-secret");
    assert_blocked(o, "hardlink-toolchain");
    assert_blocked(o, "mknod-device");
}

/// The toolchain survived cleanup untouched and still runs.
fn assert_toolchain_intact(a: &Arena) {
    let tc = toolchain();
    assert!(tc.join("bin/python3.14").is_file());
    assert!(tc.join("lib/python3.14/os.py").is_file());
    assert!(!tc.join("pwned.txt").exists());
    assert_eq!(
        std::fs::metadata(tc.join("bin/python3.14"))
            .unwrap()
            .nlink(),
        1
    );
    let w = a.new_work("toolchain-check");
    let o = go(
        "toolchain-still-runs",
        &a.python_in(&w, "print('alive')\n", &[]),
    );
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "alive");
}

/// Python run in a *different* work folder, attacking the first run's folder (argv[1]).
const ATTACK_RUN1: &str = r#"
d = sys.argv[1]
t('read-run1-file', lambda: open(os.path.join(d, 'run1.txt')).read())
t('read-run1-nested', lambda: open(os.path.join(d, 'sub', 'deeper', 'inner.txt')).read())
t('list-run1-dir', lambda: os.listdir(d))
t('write-run1-dir', lambda: open(os.path.join(d, 'new.txt'), 'w').write('x'))
t('append-run1-file', lambda: open(os.path.join(d, 'run1.txt'), 'a').write('x'))
t('delete-run1-file', lambda: os.remove(os.path.join(d, 'run1.txt')))
t('mkdir-run1-sub', lambda: os.mkdir(os.path.join(d, 'sub', 'pwned')))
"#;

const ATTACK_LABELS: [&str; 7] = [
    "read-run1-file",
    "read-run1-nested",
    "list-run1-dir",
    "write-run1-dir",
    "append-run1-file",
    "delete-run1-file",
    "mkdir-run1-sub",
];

fn work_dir_is_deleted_and_second_run_cannot_touch_it(m: Mode) {
    let a = Arena::new("deleted", m);
    let tc = toolchain();
    let run1 = a.new_work("run1");
    let o = go(
        "run1-default",
        &a.python_in(
            &run1,
            LEAVE_TRACES,
            &[a.secret.to_str().unwrap(), tc.to_str().unwrap()],
        ),
    );
    assert_eq!(
        o.exit_code,
        0,
        "run 1 failed: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert_link_escapes_blocked(&o);
    assert!(!run1.exists(), "work folder was not deleted");
    assert_toolchain_intact(&a);
    // Deleting must not have followed the symlink into the secret folder.
    assert_eq!(
        std::fs::read_to_string(a.secret.join("canary.txt")).unwrap(),
        "TOP SECRET"
    );

    let run2 = a.new_work("run2");
    let o = go(
        "run2-vs-deleted-run1",
        &a.python_in(&run2, ATTACK_RUN1, &[run1.to_str().unwrap()]),
    );
    for label in ATTACK_LABELS {
        assert_blocked(&o, label);
    }
    assert!(!run1.exists(), "run 2 recreated run 1's folder");
}

fn kept_work_dir_is_sealed_against_later_runs(m: Mode) {
    let a = Arena::new("kept", m);
    let tc = toolchain();
    let run1 = a.new_work("run1");
    let mut spec = a.python_in(
        &run1,
        LEAVE_TRACES,
        &[a.secret.to_str().unwrap(), tc.to_str().unwrap()],
    );
    spec.keep_work_dir = true;
    let o = go("run1-keep", &spec);
    assert_eq!(
        o.exit_code,
        0,
        "run 1 failed: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert_link_escapes_blocked(&o);
    assert_allowed(&o, "symlink-to-self"); // links it can make; cleanup must drop them
    assert_allowed(&o, "mkfifo");

    // Host view after run 1: folder kept, links and specials removed, no setuid left.
    assert!(run1.join("run1.txt").is_file());
    for gone in ["link-self", "link-secret", "link-tool", "fifo", "sock"] {
        assert!(
            std::fs::symlink_metadata(run1.join(gone)).is_err(),
            "{gone} left behind"
        );
    }
    assert!(
        run1.join("sub/deeper/inner.txt").is_file(),
        "link target was harmed"
    );
    assert_eq!(
        std::fs::metadata(run1.join("suid")).unwrap().mode() & 0o7000,
        0,
        "setuid bit kept"
    );
    assert_eq!(
        std::fs::metadata(run1.join("locked")).unwrap().mode() & 0o700,
        0o700,
        "locked folder not reset"
    );
    assert_toolchain_intact(&a);

    let run2 = a.new_work("run2");
    let o = go(
        "run2-vs-kept-run1",
        &a.python_in(&run2, ATTACK_RUN1, &[run1.to_str().unwrap()]),
    );
    for label in ATTACK_LABELS {
        assert_blocked(&o, label);
    }
    assert_eq!(
        std::fs::read_to_string(run1.join("run1.txt")).unwrap(),
        "written by run 1"
    );
    assert!(!run1.join("new.txt").exists());
}

// --- One sandboxed run at a time ----------------------------------------------------

/// Same rule as Windows (ADR 0006): two threads call `run` at the same moment; the
/// sandboxed processes' wall-clock intervals must be disjoint.
fn concurrent_run_calls_never_overlap(m: Mode) {
    let a = Arena::new("concurrent", m);
    let code = "import time\nprint('start', repr(time.time()), flush=True)\ntime.sleep(1.0)\nprint('end', repr(time.time()), flush=True)\n";
    let specs = [
        a.python_in(&a.new_work("t1"), code, &[]),
        a.python_in(&a.new_work("t2"), code, &[]),
    ];
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let started = std::time::Instant::now();
    let handles: Vec<_> = specs
        .into_iter()
        .map(|spec| {
            let barrier = std::sync::Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                run(&spec).expect("sandbox run failed")
            })
        })
        .collect();
    let outcomes: Vec<Outcome> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let total = started.elapsed();
    let stamp = |o: &Outcome, label: &str| -> f64 {
        line(o, label)
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap()
    };
    let mut spans: Vec<(f64, f64)> = outcomes
        .iter()
        .map(|o| {
            assert_eq!(o.termination, Termination::Exited);
            (stamp(o, "start"), stamp(o, "end"))
        })
        .collect();
    spans.sort_by(|x, y| x.0.total_cmp(&y.0));
    let gap = spans[1].0 - spans[0].1;
    eprintln!(
        "\n[concurrent] run A {:.3}..{:.3}  run B {:.3}..{:.3}  (B starts {gap:+.3} s after A ends)  total {total:?}",
        spans[0].0, spans[0].1, spans[1].0, spans[1].1
    );
    assert!(gap >= 0.0, "sandboxed runs overlapped by {:.3} s", -gap);
    assert!(
        total >= Duration::from_secs(2),
        "runs finished too fast to have been serialized: {total:?}"
    );
}

// --- Specs must stay inside the KataBox root ----------------------------------------

fn spec_pointing_at_user_profile_is_rejected(m: Mode) {
    let a = Arena::new("profile", m);
    let home = home();
    let mode_before = std::fs::metadata(&home).unwrap().mode();
    let base = a.python("print('should never run')\n", &[]);

    let mut s = base.clone();
    s.work_dir = home.clone();
    expect_rejected("work_dir=HOME", &s);

    let mut s = base.clone();
    s.read_only_dirs.push(home.clone());
    expect_rejected("read_only_dirs+=HOME", &s);

    let mut s = base.clone();
    s.sandbox_root = home.clone();
    expect_rejected("sandbox_root=HOME", &s);

    let mut s = base.clone();
    s.sandbox_root = home.parent().unwrap().to_path_buf();
    expect_rejected("sandbox_root=parent of HOME", &s);

    let mut s = base.clone();
    s.work_dir = std::env::temp_dir();
    expect_rejected("work_dir=/tmp (outside root)", &s);

    let mut s = base.clone();
    s.work_dir = PathBuf::from("/");
    expect_rejected("work_dir=/", &s);

    let mut s = base.clone();
    s.read_only_dirs.push(PathBuf::from("/"));
    expect_rejected("read_only_dirs+=/", &s);

    let mut s = base.clone();
    s.read_only_dirs.push(PathBuf::from("/proc/self"));
    expect_rejected("read_only_dirs+=/proc/self", &s);

    // A symlink inside the root that points at HOME must not pass as "inside".
    let link = a.root.join("home-link");
    symlink(&home, &link).unwrap();
    let mut s = base.clone();
    s.work_dir = link.clone();
    expect_rejected("work_dir=symlink to HOME", &s);
    std::fs::remove_file(&link).unwrap();

    assert!(home.is_dir());
    assert_eq!(
        std::fs::metadata(&home).unwrap().mode(),
        mode_before,
        "HOME permissions changed"
    );
}

fn work_dir_with_preexisting_link_is_rejected(m: Mode) {
    let a = Arena::new("prelink", m);
    symlink(&a.secret, a.work.join("link")).unwrap();
    expect_rejected(
        "work_dir containing a symlink",
        &a.python("print(1)\n", &[]),
    );
    assert!(
        a.work.is_dir(),
        "rejected spec must not delete the work folder"
    );
    assert!(a.secret.join("canary.txt").is_file());
}

// --- Linux-specific attacks ---------------------------------------------------------

fn unix_sockets_cannot_reach_the_host(m: Mode) {
    use std::os::linux::net::SocketAddrExt;
    use std::os::unix::net::{SocketAddr, UnixListener};
    let a = Arena::new("unixsock", m);
    let name = format!("kb-attack-{}", std::process::id());
    let abstract_l =
        UnixListener::bind_addr(&SocketAddr::from_abstract_name(name.as_bytes()).unwrap()).unwrap();
    abstract_l.set_nonblocking(true).unwrap();
    let path = a.secret.join("host.sock");
    let path_l = UnixListener::bind(&path).unwrap();
    path_l.set_nonblocking(true).unwrap();
    let code = r#"import socket
def conn(addr):
    s = socket.socket(socket.AF_UNIX)
    s.settimeout(2)
    s.connect(addr)
    s.send(b'hi')
t('abstract-socket', lambda: conn('\0' + sys.argv[1]))
t('path-socket', lambda: conn(sys.argv[2]))
"#;
    let o = go(
        "unix-sockets",
        &a.python(code, &[&name, path.to_str().unwrap()]),
    );
    assert_blocked(&o, "abstract-socket");
    assert_blocked(&o, "path-socket");
    assert!(
        abstract_l.accept().is_err(),
        "abstract socket reached from the sandbox"
    );
    assert!(
        path_l.accept().is_err(),
        "path socket reached from the sandbox"
    );
}

/// Each would succeed (or fail differently) for an ordinary process; seccomp makes them
/// fail with EPERM (clone3: ENOSYS, so libc falls back to clone).
#[cfg(target_arch = "x86_64")]
fn dangerous_syscalls_are_denied(m: Mode) {
    let a = Arena::new("syscalls", m);
    let code = r#"
buf = ctypes.create_string_buffer(256)
t('unshare-userns', lambda: os.unshare(os.CLONE_NEWUSER))
t('clone-newuser', lambda: sc(56, 0x10000000 | 17, 0, 0, 0, 0))
t('clone3', lambda: sc(435, buf, 88))
t('ptrace-traceme', lambda: sc(101, 0, 0, 0, 0))
t('process_vm_readv', lambda: sc(310, os.getpid(), 0, 0, 0, 0, 0))
t('kcmp', lambda: sc(312, os.getpid(), os.getpid(), 0, 0, 0))
t('keyctl-session', lambda: sc(250, 0, -3, 0))
t('add_key', lambda: sc(248, b'user', b'kb', b'x', 1, -2))
t('io_uring_setup', lambda: sc(425, 4, buf))
t('userfaultfd', lambda: sc(323, 0))
t('perf_event_open', lambda: sc(298, buf, 0, -1, -1, 0))
t('bpf', lambda: sc(321, 0, buf, 48))
t('mount', lambda: sc(165, b'none', b'/tmp', b'tmpfs', 0, 0))
t('setns', lambda: sc(308, 0, 0))
t('chroot', lambda: sc(161, b'.'))
t('open_by_handle_at', lambda: sc(304, -100, buf, 0))
t('pidfd_send_signal', lambda: sc(424, os.pidfd_open(os.getpid()), 0, 0, 0))
t('kexec_load', lambda: sc(246, 0, 0, 0, 0))
t('modify_ldt', lambda: sc(154, 0, buf, 16))
import subprocess
other = subprocess.Popen([sys.executable, '-I', '-c', 'import time; time.sleep(5)'])
t('prlimit-other', lambda: sc(302, other.pid, 7, 0, buf))
other.kill(); other.wait()
t('prlimit-self', lambda: sc(302, 0, 7, 0, buf))
t('thread-start', lambda: (__import__('threading').Thread(target=lambda: None).start()))
"#;
    let o = go("syscalls", &a.python(code, &[]));
    for label in [
        "unshare-userns",
        "clone-newuser",
        "ptrace-traceme",
        "process_vm_readv",
        "kcmp",
        "keyctl-session",
        "add_key",
        "io_uring_setup",
        "userfaultfd",
        "perf_event_open",
        "bpf",
        "mount",
        "setns",
        "chroot",
        "open_by_handle_at",
        "pidfd_send_signal",
        "kexec_load",
        "modify_ldt",
        "prlimit-other",
    ] {
        assert_errno(&o, label, EPERM);
    }
    assert_errno(&o, "clone3", 38); // ENOSYS
    assert_allowed(&o, "prlimit-self"); // controls: ordinary things still work
    assert_allowed(&o, "thread-start");
}

#[cfg(not(target_arch = "x86_64"))]
fn dangerous_syscalls_are_denied(_m: Mode) {
    eprintln!("syscall numbers in this test are x86_64 only");
}

/// kill(-1), kill(0), tgkill(0) and friends are denied by seccomp in both modes; a host
/// process is unreachable (PID namespace / Landlock signal scope).
fn signals_cannot_reach_other_processes(m: Mode) {
    let a = Arena::new("signals", m);
    let mut canary = Command::new("sleep").arg("30").spawn().unwrap();
    let pid = canary.id().to_string();
    let me = std::process::id().to_string();
    let code = r#"import signal
host = int(sys.argv[1]); runner = int(sys.argv[2])
t('kill-minus-1', lambda: os.kill(-1, signal.SIGKILL))
t('kill-0', lambda: os.kill(0, 0))
t('killpg-own', lambda: os.killpg(os.getpgid(0), 0))
t('tgkill-0', lambda: sc(234, 0, 0, 0))
t('tkill-minus', lambda: sc(200, -1, 0))
t('rt_sigqueueinfo-minus-1', lambda: sc(129, -1, 0, ctypes.create_string_buffer(128)))
t('kill-host-canary', lambda: os.kill(host, signal.SIGKILL))
t('kill-runner', lambda: os.kill(runner, 0))
t('kill-self-0', lambda: os.kill(os.getpid(), 0))
"#;
    let o = go("signals", &a.python(code, &[&pid, &me]));
    let alive = canary.try_wait().unwrap().is_none();
    let _ = canary.kill();
    let _ = canary.wait();
    for label in [
        "kill-minus-1",
        "kill-0",
        "killpg-own",
        "tgkill-0",
        "tkill-minus",
        "rt_sigqueueinfo-minus-1",
    ] {
        assert_errno(&o, label, EPERM);
    }
    assert_allowed(&o, "kill-self-0");
    if m == Mode::Fb && !signal_scoping() {
        // unscoped-fallback-for-tests on a kernel without ABI 6: exactly the gap that
        // makes the product refuse fallback here.
        assert_allowed(&o, "kill-runner");
        eprintln!("[signals] KNOWN GAP in unscoped test mode: host processes are signalable");
        assert!(!alive, "expected the unscoped fallback to reach the canary");
        return;
    }
    assert_blocked(&o, "kill-host-canary");
    assert_blocked(&o, "kill-runner");
    assert!(alive, "the host canary process was killed from the sandbox");
}

fn terminal_is_unreachable(m: Mode) {
    let a = Arena::new("tty", m);
    let code = r#"import fcntl, termios
t('open-dev-tty', lambda: open('/dev/tty', 'w').write('x'))
t('tiocsti-stdin', lambda: fcntl.ioctl(0, termios.TIOCSTI, b'x'))
t('setsid', lambda: os.setsid())
"#;
    let o = go("tty", &a.python(code, &[]));
    assert_blocked(&o, "open-dev-tty");
    assert_errno(&o, "tiocsti-stdin", EPERM);
}

fn sysv_ipc_of_the_host_is_unreachable(m: Mode) {
    let Ok(out) = Command::new("ipcmk").args(["-M", "4096"]).output() else {
        eprintln!("ipcmk not installed; skipping");
        return;
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let id = text.split(':').nth(1).unwrap().trim().to_string();
    let a = Arena::new("sysv", m);
    let code = "libc.shmat.restype = ctypes.c_void_p\ndef attach():\n    r = libc.shmat(int(sys.argv[1]), None, 0)\n    if r in (None, ctypes.c_void_p(-1).value):\n        raise OSError(ctypes.get_errno(), 'shmat')\n    return r\nt('shmat-host-segment', attach)\n";
    let o = go("sysv-ipc", &a.python(code, &[&id]));
    let _ = Command::new("ipcrm").args(["-m", &id]).status();
    assert_blocked(&o, "shmat-host-segment");
}

// --- Mode selection -------------------------------------------------------------------

/// Auto picks namespaces when they work, otherwise fallback (if the kernel scopes
/// signals), and reports which ran.
#[test]
fn auto_mode_reports_the_mode_that_ran() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("auto", Mode::Ns);
    let mut spec = a.python("print('ran')\n", &[]);
    spec.linux_isolation = LinuxIsolation::Auto;
    let p = linux_probe().unwrap();
    eprintln!("\n[auto] probe: {p:?}");
    match run(&spec) {
        Ok(o) => {
            let want = if p.user_namespaces {
                SandboxMode::Namespaces
            } else {
                SandboxMode::Fallback
            };
            assert_eq!(o.mode, want);
            eprintln!("[auto] ran in {:?}", o.mode);
        }
        Err(SandboxError::Unavailable(msg)) => {
            assert!(
                !p.user_namespaces && !fallback_runnable(),
                "unexpected refusal: {msg}"
            );
            eprintln!("[auto] refused: {msg}");
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn probe_reports_kernel_features() {
    setup();
    let p = linux_probe().unwrap();
    eprintln!("\n[probe] {}", serde_json::to_string_pretty(&p).unwrap());
    assert!(!p.kernel.is_empty());
    assert!(p.seccomp, "seccomp unavailable");
}
