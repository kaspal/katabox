//! Attack suite for the Windows sandbox (ADR 0002). Every attack must be stopped or
//! blocked. Needs the pinned Python toolchain:
//!
//! ```text
//! pwsh scripts/fetch-python-toolchain.ps1
//! cargo test -p kb-runner --test attacks_windows -- --nocapture
//! ```
//!
//! Tests run one at a time (SERIAL) so CPU-time measurements are not skewed.
#![cfg(windows)]

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use kb_core::{Language, Limits};
use kb_runner::{Outcome, SandboxSpec, Termination, run};

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

fn toolchain() -> PathBuf {
    let dir = std::env::var_os("KB_TEST_PYTHON_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/toolchains/python-3.14")
        });
    let dir = std::path::absolute(dir).unwrap();
    assert!(
        dir.join("python.exe").is_file(),
        "Python toolchain missing at {}. Run: pwsh scripts/fetch-python-toolchain.ps1",
        dir.display()
    );
    dir
}

/// Per-test folder: `work/` is the sandbox's only writable folder, `secret/` is a
/// sibling the sandbox must not see.
struct Arena {
    root: PathBuf,
    work: PathBuf,
    secret: PathBuf,
}

impl Arena {
    fn new(name: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("kbsb-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let work = root.join("work");
        let secret = root.join("secret");
        std::fs::create_dir_all(&work).unwrap();
        std::fs::create_dir_all(&secret).unwrap();
        std::fs::write(secret.join("canary.txt"), "TOP SECRET").unwrap();
        Arena { root, work, secret }
    }

    fn python(&self, code: &str, args: &[&str]) -> SandboxSpec {
        let tc = toolchain();
        let mut spec = SandboxSpec::new(
            tc.join("python.exe"),
            &self.work,
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
    assert!(
        o.user_time >= Duration::from_millis(1_900),
        "killed too early: {:?}",
        o.user_time
    );
    assert!(
        o.user_time < Duration::from_millis(2_500),
        "CPU limit overshot: {:?}",
        o.user_time
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
    let tc = toolchain();
    let mut spec = SandboxSpec::new(
        tc.join("python.exe"),
        &a.work,
        &Limits::default_for(Language::Python),
    );
    spec.args = ["-I", "-B", "-c", bomb, bomb]
        .iter()
        .map(|s| s.to_string())
        .collect();
    spec.read_only_dirs = vec![tc];
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
