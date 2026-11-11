//! The calibration benchmark (ADR 0017) in the real Windows sandbox, with the dev
//! toolchains (`pwsh scripts/fetch-toolchains.ps1`).
//!
//! `--nocapture` prints this machine's rounds and checksums; on the reference machine
//! those are the constants in `kb_judge::calibration`.
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::Language;
use kb_judge::JudgeEnv;
use kb_judge::calibration::{BENCHMARK_LANGUAGES, calibrate, calibrate_with, run_benchmark};
use kb_runner::ROOT_MARKER;

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file(),
        "sandbox root missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

#[test]
fn both_benchmarks_run_in_the_sandbox() {
    let env = env();
    for lang in BENCHMARK_LANGUAGES {
        let (best_us, checksum) = run_benchmark(&env, lang).unwrap();
        println!("{lang}: fastest round {best_us} us, checksum {checksum}");
        assert!(best_us > 1_000, "{lang}: {best_us} us is implausibly fast");
    }
    for lang in BENCHMARK_LANGUAGES {
        let c = calibrate_with(&env, lang).unwrap();
        println!("{lang}: {c:?}");
        assert_eq!(c.runtime, lang);
        assert!(c.factor > 0.0 && (0.5..=3.0).contains(&c.applied), "{c:?}");
    }
}

#[test]
fn python_is_preferred_then_node() {
    let env = env();
    assert_eq!(calibrate(&env).unwrap().runtime, Language::Python);
    // Without Python: Node.
    let root = env.sandbox_root.clone();
    let node_only =
        JudgeEnv::new(&root).with_toolchain(Language::TypeScript, root.join("toolchains/node-26"));
    assert_eq!(calibrate(&node_only).unwrap().runtime, Language::TypeScript);
}

#[test]
fn the_benchmark_cannot_reach_outside_the_sandbox() {
    // The same runner as user code: a script reading a file outside the sandbox root is
    // refused (the benchmark itself only computes).
    let env = env();
    let secret = std::env::temp_dir().join(format!("kbj-cal-secret-{}", std::process::id()));
    std::fs::write(&secret, "x").unwrap();
    let main = format!("open(r'{}').read()\nprint('read it')\n", secret.display());
    let out = kb_judge::run_script(
        &env,
        Language::Python,
        &main,
        &[],
        b"",
        kb_judge::ScriptLimits::default(),
    )
    .unwrap();
    std::fs::remove_file(&secret).unwrap();
    assert!(!out.succeeded(), "{out:?}");
    assert!(out.stderr.contains("PermissionError"), "{}", out.stderr);
    assert!(out.cpu_us > 0);
}
