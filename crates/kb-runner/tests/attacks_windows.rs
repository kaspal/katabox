//! Attack suite for the Windows sandbox (ADR 0002, 0006). Every attack must be stopped
//! or blocked. Needs the dev sandbox root with the pinned Python toolchain:
//!
//! ```text
//! pwsh scripts/fetch-python-toolchain.ps1      # creates target/katabox
//! cargo test -p kb-runner --test attacks_windows -- --nocapture
//! ```
//!
//! Tests run one at a time (SERIAL) so CPU-time measurements are not skewed.
#![cfg(windows)]

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::Duration;

use kb_core::{Language, Limits};
use kb_runner::{Outcome, ROOT_MARKER, SandboxError, SandboxSpec, Termination, run};

static SERIAL: Mutex<()> = Mutex::new(());

const MB: u64 = 1024 * 1024;

/// Shared Python helper: `t(label, fn)` prints `label OK ...` or `label ERR <winerror> <type>`.
const PRELUDE: &str = r#"
import os, sys
def t(label, fn):
    try:
        r = fn()
        print(label, "OK", repr(r)[:80], flush=True)
    except BaseException as e:
        print(label, "ERR", getattr(e, "winerror", None), type(e).__name__, flush=True)
"#;

/// The KataBox-owned dev sandbox root (`target/katabox`, or `KB_TEST_ROOT`).
fn sandbox_root() -> PathBuf {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file() && toolchain_in(&root).join("python.exe").is_file(),
        "sandbox root or Python toolchain missing at {}. Run: pwsh scripts/fetch-python-toolchain.ps1",
        root.display()
    );
    root
}

fn toolchain_in(root: &Path) -> PathBuf {
    root.join("toolchains").join("python-3.14")
}

fn toolchain() -> PathBuf {
    toolchain_in(&sandbox_root())
}

/// Per-test folder `<root>/runs/<name>-<pid>`: `work/` is the sandbox's only writable
/// folder (deleted by the runner after each run), `secret/` is a sibling it must not see.
struct Arena {
    root: PathBuf,
    work: PathBuf,
    secret: PathBuf,
}

impl Arena {
    fn new(name: &str) -> Self {
        let root = sandbox_root()
            .join("runs")
            .join(format!("{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let work = root.join("work");
        let secret = root.join("secret");
        std::fs::create_dir_all(&work).unwrap();
        std::fs::create_dir_all(&secret).unwrap();
        std::fs::write(secret.join("canary.txt"), "TOP SECRET").unwrap();
        Arena { root, work, secret }
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
        let tc = toolchain();
        let mut spec = SandboxSpec::new(
            sandbox_root(),
            tc.join("python.exe"),
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
        spec.read_only_dirs = vec![tc];
        spec
    }
}

/// `icacls` listing of `p`, as the host sees it.
fn icacls(p: &Path) -> String {
    let out = Command::new("icacls").arg(p).output().expect("run icacls");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// No AppContainer SID (S-1-15-2-…) and no ALL APPLICATION PACKAGES entry on `p`.
fn assert_no_package_aces(p: &Path) {
    let acl = icacls(p);
    assert!(
        !acl.contains("S-1-15-2-") && !acl.contains("APPLICATION PACKAGE"),
        "sandbox-reachable permission left on {}:\n{acl}",
        p.display()
    );
}

fn expect_rejected(name: &str, spec: &SandboxSpec) {
    match run(spec) {
        Err(SandboxError::InvalidSpec(msg)) => eprintln!("[{name}] rejected: {msg}"),
        other => panic!("[{name}] spec was not rejected: {other:?}"),
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn go(name: &str, spec: &SandboxSpec) -> Outcome {
    let o = run(spec).expect("sandbox run failed");
    let stdout: String = String::from_utf8_lossy(&o.stdout)
        .chars()
        .take(600)
        .collect();
    let stderr = String::from_utf8_lossy(&o.stderr);
    let stderr_tail: String = stderr.lines().rev().take(3).collect::<Vec<_>>().join(" | ");
    eprintln!(
        "\n[{name}] termination={:?} exit={:#x} user={:?} kernel={:?} wall={:?} peak={} MiB \
         procs(total={} max_active={} after={}) mem_hit={} proc_hit={}\n  stdout: {}\n  stderr(last): {}",
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
    o
}

fn line(o: &Outcome, label: &str) -> String {
    let out = String::from_utf8_lossy(&o.stdout).into_owned();
    out.lines()
        .find(|l| l.starts_with(&format!("{label} ")))
        .unwrap_or_else(|| panic!("no `{label}` line in stdout:\n{out}"))
        .to_string()
}

fn assert_blocked(o: &Outcome, label: &str) {
    let l = line(o, label);
    assert!(
        l.starts_with(&format!("{label} ERR")),
        "attack `{label}` was NOT blocked: {l}"
    );
}

fn assert_allowed(o: &Outcome, label: &str) {
    let l = line(o, label);
    assert!(
        l.starts_with(&format!("{label} OK")),
        "control `{label}` unexpectedly failed: {l}"
    );
}

// ---------------------------------------------------------------------------------

#[test]
fn baseline_print_1() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("baseline");
    let mut spec = a.python("", &[]);
    spec.args = vec!["-I".into(), "-B".into(), "-c".into(), "print(1)".into()];
    let o = go("baseline", &spec);
    assert_eq!(o.termination, Termination::Exited);
    assert_eq!(o.exit_code, 0);
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "1");
    // python.exe + the console host Windows attaches to the job.
    assert_eq!(o.total_processes, 2);
}

#[test]
fn infinite_loop_hits_cpu_limit() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("loop");
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

#[test]
fn sleep_hits_wall_limit() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("sleep");
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

#[test]
fn allocate_2gb_is_refused() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("alloc2g");
    let spec = a.python("t('alloc2g', lambda: len(bytearray(2 * 1024**3)))\n", &[]);
    let o = go("alloc-2GB", &spec);
    assert_blocked(&o, "alloc2g");
    assert!(line(&o, "alloc2g").contains("MemoryError"));
    assert!(o.memory_limit_hit, "job did not report a memory-limit hit");
}

#[test]
fn gradual_allocation_is_capped() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("allocslow");
    let code = "keep = []\ndef grow():\n    while True:\n        keep.append(bytearray(16 * 1024 * 1024))\nt('grow', grow)\nprint('held MiB', 16 * len(keep))\n";
    let spec = a.python(code, &[]);
    let o = go("alloc-gradual", &spec);
    assert_blocked(&o, "grow");
    assert!(o.memory_limit_hit);
    // The job's peak counter includes the refused chunk, so check what Python held.
    let held: u64 = line(&o, "held")
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    assert!(held * MB <= spec.memory_bytes, "held {held} MiB");
}

#[test]
fn fork_bomb_is_capped_and_cleaned_up() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("forkbomb");
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
    // +1: conhost.exe lives in the job but is not counted by ActiveProcessLimit.
    assert!(
        o.max_active_processes <= spec.max_processes + 1,
        "saw {} processes",
        o.max_active_processes
    );
    assert!(
        o.total_processes <= spec.max_processes + 1,
        "{} processes created",
        o.total_processes
    );
    assert_ne!(o.termination, Termination::Exited);
}

#[test]
fn network_loopback_is_blocked() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("loopback");
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

#[test]
fn network_internet_and_dns_are_blocked() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("internet");
    let code = r#"import socket
t('tcp-1.1.1.1:443', lambda: socket.create_connection(('1.1.1.1', 443), timeout=3).close())
t('dns-example.com', lambda: socket.getaddrinfo('example.com', 443))
def udp():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.settimeout(2)
    s.sendto(b'\x12\x34\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07example\x03com\x00\x00\x01\x00\x01', ('1.1.1.1', 53))
    return s.recvfrom(512)
t('udp-1.1.1.1:53', udp)
"#;
    let o = go("net-internet", &a.python(code, &[]));
    assert_blocked(&o, "tcp-1.1.1.1:443");
    assert_blocked(&o, "dns-example.com");
    // UDP sendto reports success and the reply never arrives. This proves no round
    // trip; whether the datagram is dropped before leaving needs a packet capture.
    assert_blocked(&o, "udp-1.1.1.1:53");
}

#[test]
fn reading_outside_allowed_folders_is_blocked() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("readout");
    let canary = a.secret.join("canary.txt");
    let cargo_toml = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
    let cargo_toml = std::path::absolute(cargo_toml).unwrap();
    let profile = std::env::var("USERPROFILE").unwrap();
    let code = r#"
t('read-canary', lambda: open(sys.argv[1]).read())
t('read-repo-Cargo.toml', lambda: open(sys.argv[2]).read())
t('list-userprofile', lambda: os.listdir(sys.argv[3]))
t('list-secret-dir', lambda: os.listdir(sys.argv[4]))
t('read-own-work-file', lambda: (open('mine.txt', 'w').write('ok'), open('mine.txt').read()))
"#;
    let o = go(
        "read-outside",
        &a.python(
            code,
            &[
                canary.to_str().unwrap(),
                cargo_toml.to_str().unwrap(),
                &profile,
                a.secret.to_str().unwrap(),
            ],
        ),
    );
    assert_blocked(&o, "read-canary");
    assert_blocked(&o, "read-repo-Cargo.toml");
    assert_blocked(&o, "list-userprofile");
    assert_blocked(&o, "list-secret-dir");
    assert_allowed(&o, "read-own-work-file"); // positive control: the work folder works
}

#[test]
fn writing_outside_work_folder_is_blocked() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("writeout");
    let tc = toolchain();
    let code = r#"
t('write-toolchain', lambda: open(os.path.join(sys.argv[1], 'pwned.txt'), 'w').write('x'))
t('write-toolchain-Lib', lambda: open(os.path.join(sys.argv[1], 'Lib', 'os.py'), 'a').write('#'))
t('write-arena-root', lambda: open(os.path.join(sys.argv[2], 'pwned.txt'), 'w').write('x'))
t('write-profile-TEMP', lambda: open(os.path.join(os.environ['TEMP'], 'pwned.txt'), 'w').write('x'))
import winreg
t('reg-HKCU-Software', lambda: winreg.CreateKey(winreg.HKEY_CURRENT_USER, r'Software\KataBoxPwned'))
t('reg-profile-storage', lambda: winreg.CreateKey(winreg.HKEY_CURRENT_USER,
    r'Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\katabox.sandbox\pwned'))
t('write-work', lambda: open('ok.txt', 'w').write('x'))
"#;
    let o = go(
        "write-outside",
        &a.python(code, &[tc.to_str().unwrap(), a.root.to_str().unwrap()]),
    );
    for label in [
        "write-toolchain",
        "write-toolchain-Lib",
        "write-arena-root",
        "write-profile-TEMP",
        "reg-HKCU-Software",
        "reg-profile-storage",
    ] {
        assert_blocked(&o, label);
    }
    assert_allowed(&o, "write-work");
    assert!(!tc.join("pwned.txt").exists());
}

#[test]
fn job_breakaway_is_refused() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("breakaway");
    let code = r#"import subprocess
t('breakaway', lambda: subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)'],
    creationflags=subprocess.CREATE_BREAKAWAY_FROM_JOB).pid)
"#;
    let o = go("breakaway", &a.python(code, &[]));
    assert_blocked(&o, "breakaway");
}

#[test]
fn host_environment_does_not_leak() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("env");
    let o = go(
        "env",
        &a.python("print('ENV', ' '.join(sorted(os.environ)))\n", &[]),
    );
    let keys: Vec<String> = line(&o, "ENV")
        .split_whitespace()
        .skip(1)
        .map(str::to_string)
        .collect();
    let allowed = ["LOCALAPPDATA", "SYSTEMROOT", "TEMP", "TMP", "WINDIR"];
    for k in &keys {
        assert!(
            allowed.contains(&k.as_str()),
            "unexpected variable {k} in sandbox env"
        );
    }
}

#[test]
fn output_flood_is_cut_off() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("flood");
    let spec = a.python("while True:\n    sys.stdout.write('x' * 4096)\n", &[]);
    let o = go("output-flood", &spec);
    assert_eq!(o.termination, Termination::OutputLimit);
    assert!(o.stdout.len() <= spec.max_output_bytes);
}

/// Known gap, recorded rather than hidden: a plain AppContainer can read files that
/// Windows grants to ALL APPLICATION PACKAGES (e.g. C:\Windows\win.ini). LPAC would close
/// this but python-build-standalone does not start under LPAC yet. If this test starts
/// failing, the gap closed: flip it into an assert_blocked.
#[test]
fn known_gap_system_files_readable_without_lpac() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("sysread");
    let o = go(
        "known-gap-win.ini",
        &a.python(
            "t('read-win.ini', lambda: len(open(r'C:\\Windows\\win.ini').read()))\n",
            &[],
        ),
    );
    assert_allowed(&o, "read-win.ini");
}

// --- CPU limit counts kernel time --------------------------------------------------

#[test]
fn kernel_time_counts_toward_cpu_limit() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("kernelburn");
    // Rewriting 1 MiB into the file cache is ~96% kernel time (measured); a limit on
    // user time alone would let this run until the 6 s wall-clock limit.
    let code = "fd = os.open('burn.bin', os.O_RDWR | os.O_CREAT | os.O_BINARY)\nbuf = b'x' * (1 << 20)\nwhile True:\n    os.lseek(fd, 0, 0)\n    os.write(fd, buf)\n";
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

// --- Nothing stays writable between runs --------------------------------------------

/// Python that leaves things behind: nested files, a read-only file, junctions written
/// with raw FSCTL_SET_REPARSE_POINT (to the secret folder = argv[1], to the toolchain =
/// argv[2], and a loop back into its own folder), and icacls grants to ALL APPLICATION
/// PACKAGES (S-1-15-2-1).
const LEAVE_TRACES: &str = r#"
open('run1.txt', 'w').write('written by run 1')
os.makedirs(os.path.join('sub', 'deeper'))
open(os.path.join('sub', 'deeper', 'inner.txt'), 'w').write('x')
import stat
open('readonly.txt', 'w').write('x'); os.chmod('readonly.txt', stat.S_IREAD)

import ctypes, ctypes.wintypes as w, struct
k32 = ctypes.WinDLL('kernel32', use_last_error=True)
k32.CreateFileW.restype = w.HANDLE
k32.CreateFileW.argtypes = [w.LPCWSTR, w.DWORD, w.DWORD, w.LPVOID, w.DWORD, w.DWORD, w.HANDLE]
k32.DeviceIoControl.argtypes = [w.HANDLE, w.DWORD, w.LPVOID, w.DWORD, w.LPVOID, w.DWORD,
                                ctypes.POINTER(w.DWORD), w.LPVOID]
def raw_junction(link, target):
    os.mkdir(link)
    h = k32.CreateFileW(link, 0x40000000, 0, None, 3, 0x02000000 | 0x00200000, None)
    if h in (None, ctypes.c_void_p(-1).value):
        raise ctypes.WinError(ctypes.get_last_error())
    sub = ('\\??\\' + target).encode('utf-16-le'); prn = target.encode('utf-16-le')
    body = struct.pack('<HHHH', 0, len(sub), len(sub) + 2, len(prn)) + sub + b'\0\0' + prn + b'\0\0'
    buf = struct.pack('<IHH', 0xA0000003, len(body), 0) + body
    n = w.DWORD()
    ok = k32.DeviceIoControl(h, 0x000900A4, buf, len(buf), None, 0, ctypes.byref(n), None)
    k32.CloseHandle(h)
    if not ok:
        raise ctypes.WinError(ctypes.get_last_error())
    return os.readlink(link)
t('junction-to-secret', lambda: raw_junction(os.path.abspath('link-secret'), sys.argv[1]))
t('junction-to-toolchain', lambda: raw_junction(os.path.abspath('link-tool'), sys.argv[2]))
t('junction-to-self', lambda: raw_junction(os.path.abspath('link-self'), os.path.abspath('sub')))

import subprocess
icacls = os.path.join(os.environ['SYSTEMROOT'], 'System32', 'icacls.exe')
def grant(target, perm):
    r = subprocess.run([icacls, target, '/grant', perm], capture_output=True, text=True)
    print('  icacls', target, '->', (r.stdout + r.stderr).strip().replace('\n', ' / ')[:110])
    if r.returncode:
        raise PermissionError(r.returncode)
for label, target, perm in [('grant-aap-file', 'run1.txt', '*S-1-15-2-1:(F)'),
                            ('grant-aap-subdir', 'sub', '*S-1-15-2-1:(OI)(CI)(F)'),
                            ('grant-aap-workdir', '.', '*S-1-15-2-1:(OI)(CI)(F)')]:
    t(label, lambda: grant(target, perm))
"#;

/// The junction escapes must fail; Windows only lets a sandboxed process point a mount
/// point at a folder it can write to.
fn assert_junction_escapes_blocked(o: &Outcome) {
    assert_blocked(o, "junction-to-secret");
    assert_blocked(o, "junction-to-toolchain");
}

/// The toolchain survived cleanup untouched and still runs.
fn assert_toolchain_intact(a: &Arena) {
    assert!(toolchain().join("python.exe").is_file());
    assert!(toolchain().join("Lib").join("os.py").is_file());
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

#[test]
fn work_dir_is_deleted_and_second_run_cannot_touch_it() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("deleted");
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
    assert_junction_escapes_blocked(&o);
    assert!(!run1.exists(), "work folder was not deleted");
    assert_toolchain_intact(&a);
    // Deleting must not have followed the junction into the secret folder.
    assert_eq!(
        std::fs::read_to_string(a.secret.join("canary.txt")).unwrap(),
        "TOP SECRET"
    );
    assert_no_package_aces(&a.secret);

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

#[test]
fn kept_work_dir_is_sealed_against_later_runs() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("kept");
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
    assert_junction_escapes_blocked(&o);
    assert_allowed(&o, "junction-to-self"); // the one link it can make; cleanup must drop it

    // Host view after run 1: folder kept, links removed, no sandbox-reachable entry anywhere.
    assert!(run1.join("run1.txt").is_file());
    assert!(
        std::fs::symlink_metadata(run1.join("link-self")).is_err(),
        "junction left behind"
    );
    assert!(
        run1.join("sub").join("deeper").join("inner.txt").is_file(),
        "link target was harmed"
    );
    assert_toolchain_intact(&a);
    for p in [
        run1.clone(),
        run1.join("run1.txt"),
        run1.join("readonly.txt"),
        run1.join("sub"),
        run1.join("sub").join("deeper"),
        run1.join("sub").join("deeper").join("inner.txt"),
    ] {
        assert_no_package_aces(&p);
    }

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

/// All runs share one AppContainer SID, so they must never overlap. Two threads call
/// `run` at the same moment; each sandboxed process reports when it started and ended
/// (wall clock), and those intervals must be disjoint.
#[test]
fn concurrent_run_calls_never_overlap() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("concurrent");
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
                barrier.wait(); // both threads call run() together
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

#[test]
fn spec_pointing_at_user_profile_is_rejected() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("profile");
    let profile = PathBuf::from(std::env::var_os("USERPROFILE").unwrap());
    let profile_acl_before = icacls(&profile);
    let base = a.python("print('should never run')\n", &[]);

    let mut s = base.clone();
    s.work_dir = profile.clone();
    expect_rejected("work_dir=USERPROFILE", &s);

    let mut s = base.clone();
    s.read_only_dirs.push(profile.clone());
    expect_rejected("read_only_dirs+=USERPROFILE", &s);

    let mut s = base.clone();
    s.sandbox_root = profile.clone();
    expect_rejected("sandbox_root=USERPROFILE", &s);

    let mut s = base.clone();
    s.sandbox_root = profile.parent().unwrap().to_path_buf();
    expect_rejected("sandbox_root=parent of USERPROFILE", &s);

    let mut s = base.clone();
    s.work_dir = profile.join("Documents");
    expect_rejected("work_dir=USERPROFILE\\Documents (outside root)", &s);

    let mut s = base.clone();
    s.work_dir = PathBuf::from(r"C:\");
    expect_rejected("work_dir=C:\\", &s);

    let mut s = base.clone();
    s.read_only_dirs.push(PathBuf::from(r"C:\"));
    expect_rejected("read_only_dirs+=C:\\", &s);

    // A junction inside the root that points at the profile must not pass as "inside".
    let link = a.root.join("profile-link");
    let made = Command::new("cmd")
        .args(["/c", "mklink", "/J"])
        .arg(&link)
        .arg(&profile)
        .output()
        .unwrap();
    assert!(made.status.success(), "mklink failed: {made:?}");
    let mut s = base.clone();
    s.work_dir = link.clone();
    expect_rejected("work_dir=junction to USERPROFILE", &s);
    std::fs::remove_dir(&link).unwrap(); // removes the junction only

    // Nothing was granted or deleted along the way.
    assert!(profile.is_dir());
    assert_eq!(
        icacls(&profile),
        profile_acl_before,
        "user profile ACL changed"
    );
    assert_no_package_aces(&profile);
}

#[test]
fn work_dir_with_preexisting_link_is_rejected() {
    let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let a = Arena::new("prelink");
    let link = a.work.join("link");
    let made = Command::new("cmd")
        .args(["/c", "mklink", "/J"])
        .arg(&link)
        .arg(&a.secret)
        .output()
        .unwrap();
    assert!(made.status.success(), "mklink failed: {made:?}");
    expect_rejected(
        "work_dir containing a junction",
        &a.python("print(1)\n", &[]),
    );
    assert_no_package_aces(&a.secret);
    assert!(
        a.work.is_dir(),
        "rejected spec must not delete the work folder"
    );
}
