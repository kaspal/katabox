//! KataBox problem packs.
//!
//! `kb-pack validate [DIR] [--run] [--root ROOT]` — checks every problem folder under DIR
//! (default `content/problems`) against the package format in the plan: structure,
//! metadata, test files, and that each reference solution matches its generated stub.
//! `--run` also judges every reference solution in every language against all tests in
//! the sandbox (content pipeline step 2) and fails unless each one is Accepted. ROOT is
//! the sandbox root with the toolchains (default `target/katabox`, created by
//! `scripts/fetch-toolchains.ps1`).
//!
//! `kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]` — validates as
//! above (without `--run`), then compiles every problem into content.db (default
//! `target/content.db`, pack `core`, this crate's version). Refuses to build if any
//! problem fails validation.

mod build;
mod validate;

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage:\n  kb-pack validate [DIR] [--run] [--root ROOT]\n  \
                     kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("validate") => validate_cmd(args),
        Some("build") => build_cmd(args),
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
    while let Some(a) = args.next() {
        match a.as_str() {
            "--run" => run = true,
            "--root" => match args.next() {
                Some(r) => run_root = Some(r.into()),
                None => return usage("--root needs a value"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ => dir = a.into(),
        }
    }
    let code = validate::validate_all(&dir);
    if code != ExitCode::SUCCESS || !run {
        return code;
    }
    let root = run_root.unwrap_or_else(|| "target/katabox".into());
    validate::run_references(&dir, &root)
}

fn build_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut opts = build::BuildOptions {
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
    match build::build(&opts) {
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
