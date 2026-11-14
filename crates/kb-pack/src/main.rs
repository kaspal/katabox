//! KataBox problem packs.
//!
//! `kb-pack validate [DIR] [--run] [--learn] [--root ROOT]` — checks every problem folder under DIR
//! and the Learn topics in content/learn (ADR 0029; `--learn`: only the topics)
//! (default `content/problems`) against the package format in the plan: structure,
//! metadata, test files, and that each reference solution matches its generated stub.
//! `--run` adds the sandbox checks (ADR 0016): `validate.py` accepts every test; every
//! reference, in every language, is Accepted within half the time limit; with `gen.py`
//! and `solutions/brute.py`, the references agree with brute on 500 seeded inputs; and
//! `solutions/naive.py` gets Time Limit Exceeded. ROOT is the sandbox root with the
//! toolchains (default `target/katabox`, created by `scripts/fetch-toolchains.ps1`).
//!
//! `kb-pack generate PROBLEM_DIR [--size max] --seeds 1,2 [--from N] [--root ROOT]` —
//! writes `tests/hidden/<N..>.json` from `gen.py` (outputs from `solutions/ref.py`), all
//! run in the sandbox. Generated tests are committed: the build never runs scripts.
//!
//! `kb-pack profile PROBLEM_DIR [--root ROOT]` — for a problem with `profile = true`,
//! writes the complexity profiler's reference (`profile/reference.json`: the hashes of
//! the inputs gen.py makes at each of `profile_sizes`, and the reference solutions'
//! times; committed) and the input cache (`profile/inputs/`, gitignored), all run in the
//! sandbox (ADR 0024).
//!
//! `kb-pack profile --inputs [DIR] [--root ROOT]` — regenerates the input cache of one
//! problem, or of every profiled problem under DIR (default `content/problems`), and
//! verifies it against the committed hashes. `kb-pack build` packs a profile only when
//! its cache is complete.
//!
//! `kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]` — validates as
//! above (without `--run`), then compiles every problem into content.db (default
//! `target/content.db`, pack `core`, this crate's version). Refuses to build if any
//! problem fails validation.

use kb_pack::{BuildOptions, build, run_references, validate_all, validate_learn_only};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage:\n  kb-pack validate [DIR] [--run] [--learn] [--root ROOT]\n  \
                     kb-pack generate PROBLEM_DIR [--size max|small] --seeds 1,2 [--from N] [--root ROOT]\n  \
                     kb-pack profile PROBLEM_DIR [--root ROOT]\n  \
                     kb-pack profile --inputs [DIR] [--root ROOT]
                       kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("validate") => validate_cmd(args),
        Some("build") => build_cmd(args),
        Some("generate") => generate_cmd(args),
        Some("profile") => profile_cmd(args),
        _ => usage(""),
    }
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("{problem}");
    }
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn validate_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir = PathBuf::from("content/problems");
    let mut run_root: Option<PathBuf> = None;
    let mut run = false;
    let mut learn_only = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--run" => run = true,
            "--learn" => learn_only = true,
            "--root" => match args.next() {
                Some(r) => run_root = Some(r.into()),
                None => return usage("--root needs a value"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ => dir = a.into(),
        }
    }
    let root = run_root.unwrap_or_else(|| "target/katabox".into());
    if learn_only {
        return validate_learn_only(&dir, run.then_some(root.as_path()));
    }
    let code = validate_all(&dir);
    if code != ExitCode::SUCCESS || !run {
        return code;
    }
    run_references(&dir, &root)
}

fn generate_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir: Option<PathBuf> = None;
    let mut size = "max".to_string();
    let mut seeds: Vec<u64> = Vec::new();
    let mut first: Option<u32> = None;
    let mut root = PathBuf::from("target/katabox");
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        let set = match a.as_str() {
            "--size" => value("--size").map(|v| size = v),
            "--root" => value("--root").map(|v| root = v.into()),
            "--from" => value("--from").and_then(|v| {
                v.parse()
                    .map(|n| first = Some(n))
                    .map_err(|_| "--from needs a number".to_string())
            }),
            "--seeds" => value("--seeds").and_then(|v| {
                v.split(',')
                    .map(|s| s.trim().parse::<u64>().map_err(|_| format!("bad seed {s}")))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|s| seeds = s)
            }),
            s if s.starts_with("--") => Err(format!("unknown option {s}")),
            _ => {
                dir = Some(a.into());
                Ok(())
            }
        };
        if let Err(e) = set {
            return usage(&e);
        }
    }
    let Some(dir) = dir else {
        return usage("generate needs a problem folder");
    };
    if seeds.is_empty() {
        return usage("generate needs --seeds");
    }
    // Default: after the last numbered hidden test.
    let first = first.unwrap_or_else(|| {
        std::fs::read_dir(dir.join("tests/hidden"))
            .map(|rd| {
                rd.flatten()
                    .filter_map(|e| {
                        e.path()
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .and_then(|s| s.parse::<u32>().ok())
                    })
                    .max()
                    .unwrap_or(0)
                    + 1
            })
            .unwrap_or(1)
    });
    let env = match kb_pack::env_for(&root) {
        Ok(e) => e,
        Err(e) => return usage(&e),
    };
    match kb_pack::generate_tests(&dir, &env, &size, &seeds, first) {
        Ok(files) => {
            for f in files {
                let bytes = std::fs::metadata(&f).map(|m| m.len()).unwrap_or(0);
                println!("wrote {} ({bytes} bytes)", f.display());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack generate: {e}");
            ExitCode::FAILURE
        }
    }
}

fn profile_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir: Option<PathBuf> = None;
    let mut root = PathBuf::from("target/katabox");
    let mut inputs_only = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--inputs" => inputs_only = true,
            "--root" => match args.next() {
                Some(v) => root = v.into(),
                None => return usage("--root needs a value"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ => dir = Some(a.into()),
        }
    }
    if inputs_only {
        let dir = dir.unwrap_or_else(|| "content/problems".into());
        let env = match kb_pack::env_for(&root) {
            Ok(e) => e,
            Err(e) => return usage(&e),
        };
        return match kb_pack::profile::fill_caches(&dir, &env) {
            Ok(lines) => {
                for l in lines {
                    println!("{l}");
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("kb-pack profile --inputs: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let Some(dir) = dir else {
        return usage("profile needs a problem folder");
    };
    let env = match kb_pack::env_for(&root) {
        Ok(e) => e,
        Err(e) => return usage(&e),
    };
    match kb_pack::profile::profile_problem(&dir, &env) {
        Ok(lines) => {
            for l in lines {
                println!("{l}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack profile: {e}");
            ExitCode::FAILURE
        }
    }
}

fn build_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut opts = BuildOptions {
        dir: "content/problems".into(),
        out: "target/content.db".into(),
        pack_id: "core".into(),
        pack_version: env!("CARGO_PKG_VERSION").into(),
    };
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        let set = match a.as_str() {
            "--out" => value("--out").map(|v| opts.out = v.into()),
            "--pack-id" => value("--pack-id").map(|v| opts.pack_id = v),
            "--pack-version" => value("--pack-version").map(|v| opts.pack_version = v),
            s if s.starts_with("--") => Err(format!("unknown option {s}")),
            _ => {
                opts.dir = a.into();
                Ok(())
            }
        };
        if let Err(e) = set {
            return usage(&e);
        }
    }
    match build(&opts) {
        Ok(n) => {
            println!("built {} ({n} problem(s))", opts.out.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack build: {e}");
            ExitCode::FAILURE
        }
    }
}
