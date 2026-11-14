//! The sandbox's protections hold for program problems (ADR 0028): every attack a
//! submission can make, sent as an ordinary program through `kb_judge::judge` (one run
//! per test, the user's file as the program). kb-runner's own suite (attacks_windows,
//! 20 tests) covers the runner itself; these prove the program path adds no way around
//! it. Needs the dev sandbox root:
//!
//! ```text
//! cargo test -p kb-judge --test program_attacks_windows -- --nocapture --test-threads=1
//! ```
#![cfg(windows)]

use std::net::TcpListener;
use std::path::{Path, PathBuf};

use kb_core::{Language, Report, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, ProblemMode, judge};
use kb_runner::ROOT_MARKER;
use serde_json::{Map, Value};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn root() -> PathBuf {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    std::path::absolute(root).unwrap()
}

fn env() -> JudgeEnv {
    assert!(root().join(ROOT_MARKER).is_file(), "sandbox root missing");
    JudgeEnv::dev(root())
}

fn test(index: u32, stdin: &str) -> TestCase {
    let mut input = Map::new();
    input.insert("stdin".into(), Value::String(stdin.into()));
    TestCase {
        kind: TestKind::Public,
        index,
        input,
        expected: Value::String("unused".into()),
    }
}

fn problem(stdins: &[&str]) -> Problem {
    let tests = stdins
        .iter()
        .enumerate()
        .map(|(i, s)| test(i as u32 + 1, s))
        .collect();
    Problem::from_parts(
        "attacks",
        ProblemMode::Program,
        &Value::Null,
        "tokens",
        300,
        256,
        tests,
    )
    .unwrap()
}

fn run(lang: Language, src: &str, stdins: &[&str]) -> Report {
    let p = problem(stdins);
    judge(&p, &p.tests, lang, src, &env()).unwrap()
}

/// The program's stdout of test `i`.
fn out(r: &Report, i: usize) -> String {
    r.tests[i]
        .actual
        .as_ref()
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

/// `label BLOCKED ...` or `label ALLOWED ...` from the Python probe below.
fn verdict_of(text: &str, label: &str) -> String {
    text.lines()
        .find(|l| l.starts_with(&format!("{label} ")))
        .unwrap_or_else(|| panic!("no line for {label} in:\n{text}"))
        .to_string()
}

fn assert_blocked(text: &str, label: &str) {
    let l = verdict_of(text, label);
    assert!(l.contains("BLOCKED"), "{l}");
}

fn assert_allowed(text: &str, label: &str) {
    let l = verdict_of(text, label);
    assert!(l.contains("ALLOWED"), "{l}");
}

/// Each probe prints `label ALLOWED <result>` or `label BLOCKED <exception>`; its input
/// arrives on stdin, one argument per line.
const PROBE: &str = "import os, sys
args = sys.stdin.read().splitlines()
def t(label, f):
    try:
        print(label, 'ALLOWED', repr(f())[:80], flush=True)
    except BaseException as e:
        print(label, 'BLOCKED', type(e).__name__, flush=True)
";

#[test]
fn cpu_wall_and_kernel_time_end_in_a_time_limit() {
    for src in [
        "while True:\n    pass\n",
        "import time\ntime.sleep(60)\n",
        // About 96% kernel time: it still counts toward the CPU limit.
        "data = b'x' * (1 << 20)\nwhile True:\n    with open('burn.bin', 'wb') as f:\n        f.write(data)\n",
    ] {
        let r = run(Language::Python, src, &["1"]);
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded, "{src}");
    }
}

#[test]
fn memory_is_capped() {
    for src in [
        "x = bytearray(2 * 1024 ** 3)\n",
        "keep = []\nwhile True:\n    keep.append(bytearray(16 * 1024 * 1024))\n",
    ] {
        let r = run(Language::Python, src, &["1"]);
        assert_eq!(
            r.verdict,
            Verdict::MemoryLimitExceeded,
            "{src}: {:?}",
            r.tests[0].result
        );
    }
}

#[test]
fn a_fork_bomb_is_contained() {
    let src = "import subprocess, sys, time
code = 'import subprocess, sys, time\\nwhile True:\\n    try:\\n        subprocess.Popen([sys.executable, \"-I\", \"-c\", sys.argv[1], sys.argv[1]])\\n    except OSError:\\n        time.sleep(0.01)\\n'
while True:
    try:
        subprocess.Popen([sys.executable, '-I', '-c', code, code])
    except OSError:
        time.sleep(0.01)
";
    let r = run(Language::Python, src, &["1"]);
    assert_ne!(r.verdict, Verdict::Accepted);
    assert!(
        matches!(
            r.verdict,
            Verdict::TimeLimitExceeded | Verdict::RuntimeError | Verdict::OutputLimitExceeded
        ),
        "{:?}",
        r.tests[0].result
    );
}

#[test]
fn the_network_is_blocked() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let src = format!(
        "{PROBE}import socket
t('tcp-loopback', lambda: socket.create_connection(('127.0.0.1', int(args[0])), timeout=3).close())
t('tcp-1.1.1.1:443', lambda: socket.create_connection(('1.1.1.1', 443), timeout=3).close())
t('dns-example.com', lambda: socket.getaddrinfo('example.com', 443))
"
    );
    let r = run(Language::Python, &src, &[&port]);
    let text = out(&r, 0);
    for label in ["tcp-loopback", "tcp-1.1.1.1:443", "dns-example.com"] {
        assert_blocked(&text, label);
    }
    assert!(
        listener.accept().is_err(),
        "the listener got a connection from the sandbox"
    );
}

#[test]
fn reading_and_writing_outside_the_work_folder_is_blocked() {
    let cargo_toml = std::path::absolute(repo().join("Cargo.toml")).unwrap();
    let toolchain = root().join("toolchains/python-3.14");
    let profile = std::env::var("USERPROFILE").unwrap();
    let src = format!(
        "{PROBE}t('read-repo-Cargo.toml', lambda: open(args[0]).read())
t('list-userprofile', lambda: os.listdir(args[1]))
t('write-toolchain', lambda: open(os.path.join(args[2], 'pwned.txt'), 'w').write('x'))
t('write-TEMP', lambda: open(os.path.join(os.environ.get('TEMP', args[1]), 'pwned.txt'), 'w').write('x'))
import winreg
t('reg-HKCU', lambda: winreg.CreateKey(winreg.HKEY_CURRENT_USER, r'Software\\KataBoxPwned'))
t('write-work', lambda: open('mine.txt', 'w').write('ok'))
t('read-work', lambda: open('mine.txt').read())
"
    );
    let stdin = format!(
        "{}\n{}\n{}\n",
        cargo_toml.display(),
        profile,
        toolchain.display()
    );
    let r = run(Language::Python, &src, &[&stdin]);
    let text = out(&r, 0);
    for label in [
        "read-repo-Cargo.toml",
        "list-userprofile",
        "write-toolchain",
        "write-TEMP",
        "reg-HKCU",
    ] {
        assert_blocked(&text, label);
    }
    assert_allowed(&text, "write-work");
    assert_allowed(&text, "read-work");
    assert!(!toolchain.join("pwned.txt").exists());
}

#[test]
fn job_breakaway_and_the_host_environment() {
    let src = format!(
        "{PROBE}import subprocess
t('breakaway', lambda: subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)'],
    creationflags=subprocess.CREATE_BREAKAWAY_FROM_JOB).pid)
print('ENV', ' '.join(sorted(os.environ)))
"
    );
    let r = run(Language::Python, &src, &["1"]);
    let text = out(&r, 0);
    assert_blocked(&text, "breakaway");
    let env_line = verdict_of(&text, "ENV");
    let allowed = ["LOCALAPPDATA", "SYSTEMROOT", "TEMP", "TMP", "WINDIR"];
    for k in env_line.split_whitespace().skip(1) {
        assert!(
            allowed.contains(&k),
            "unexpected variable {k} in the program's environment"
        );
    }
}

#[test]
fn an_output_flood_is_cut_off() {
    let r = run(
        Language::Python,
        "import sys\nwhile True:\n    sys.stdout.write('x' * 4096)\n",
        &["1"],
    );
    assert_eq!(r.verdict, Verdict::OutputLimitExceeded);
    // What a report keeps of it is capped.
    assert!(r.tests[0].stdout.len() <= kb_judge::program::STORED_TEXT_BYTES + 64);
}

#[test]
fn each_test_runs_in_a_fresh_folder() {
    // Test 1 leaves a file behind; test 2 must not find it (nor the source of test 1's
    // folder): one run per test, each folder deleted after its run.
    let src = format!(
        "{PROBE}if args[0] == 'write':
    t('leave', lambda: open('leak.txt', 'w').write('secret'))
else:
    t('find', lambda: open('leak.txt').read())
    t('parent', lambda: [n for n in os.listdir('..') if n.startswith('judge-')])
"
    );
    let r = run(Language::Python, &src, &["write", "read"]);
    assert_allowed(&out(&r, 0), "leave");
    let text = out(&r, 1);
    assert_blocked(&text, "find");
    assert_blocked(&text, "parent");
}

#[test]
fn compiled_and_other_runtimes_get_the_same_walls() {
    let cargo_toml = std::path::absolute(repo().join("Cargo.toml")).unwrap();
    let stdin = format!("{}\n", cargo_toml.display());
    let cases = [
        (
            Language::Rust,
            "use std::io::Read;\nfn main() {\n    let mut s = String::new();\n    std::io::stdin().read_to_string(&mut s).unwrap();\n    match std::fs::read_to_string(s.trim()) {\n        Ok(_) => println!(\"read ALLOWED\"),\n        Err(e) => println!(\"read BLOCKED {}\", e.kind()),\n    }\n    match std::net::TcpStream::connect(\"1.1.1.1:443\") {\n        Ok(_) => println!(\"net ALLOWED\"),\n        Err(e) => println!(\"net BLOCKED {}\", e.kind()),\n    }\n}\n",
        ),
        (
            Language::TypeScript,
            "import { readFileSync } from \"node:fs\";\nimport { connect } from \"node:net\";\nconst path = readFileSync(0, \"utf8\").trim();\ntry { readFileSync(path); console.log(\"read ALLOWED\"); } catch (e) { console.log(\"read BLOCKED \" + (e as Error).name); }\nconst s = connect(443, \"1.1.1.1\");\nconst done = (text: string) => { console.log(text); s.destroy(); process.exit(0); };\ns.on(\"connect\", () => done(\"net ALLOWED\"));\ns.on(\"error\", (e) => done(\"net BLOCKED \" + (e as NodeJS.ErrnoException).code));\nsetTimeout(() => done(\"net BLOCKED timeout\"), 3000);\n",
        ),
        (
            Language::Php,
            "<?php\n$path = trim(stream_get_contents(STDIN));\n$r = @file_get_contents($path);\necho $r === false ? \"read BLOCKED\\n\" : \"read ALLOWED\\n\";\n$s = @fsockopen(\"1.1.1.1\", 443, $no, $err, 3);\necho $s === false ? \"net BLOCKED\\n\" : \"net ALLOWED\\n\";\n",
        ),
    ];
    for (lang, src) in cases {
        let r = run(lang, src, &[&stdin]);
        let text = out(&r, 0);
        assert_blocked(&text, "read");
        assert_blocked(&text, "net");
        println!("{lang}: {}", text.trim().replace('\n', " | "));
    }
}
