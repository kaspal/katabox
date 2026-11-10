//! Manual test command for the sandbox.
//!
//! ```text
//! kb-sandbox run --root <dir> [--toolchain <dir>] [--work <dir>] [--keep-work]
//!                [--cpu-ms N] [--wall-ms N] [--mem-mb N] [--procs N] [--lpac]
//!                [--env K=V] -- <program> [args...]
//! kb-sandbox cleanup --root <dir> [--toolchain <dir>]
//! ```
//!
//! `--root` is the KataBox sandbox root (must contain `.katabox-sandbox-root`; the dev
//! root `target/katabox` is created by `scripts/fetch-python-toolchain.ps1`).
//! `--toolchain` defaults to `<root>/toolchains/python-3.14`. A bare program name
//! (e.g. `python`) is resolved to `<toolchain>/<name>.exe`. Without `--work`, a fresh
//! `<root>/runs/cli-<pid>` folder is used; the runner deletes the work folder after the
//! run unless `--keep-work`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use kb_core::{Language, Limits};
use kb_runner::{SandboxSpec, cleanup, run};

const USAGE: &str = "usage:
  kb-sandbox run --root <dir> [--toolchain <dir>] [--work <dir>] [--keep-work] [--cpu-ms N] [--wall-ms N] [--mem-mb N] [--procs N] [--lpac] [--env K=V] -- <program> [args...]
  kb-sandbox cleanup --root <dir> [--toolchain <dir>]";

fn main() -> ExitCode {
    match real_main() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("kb-sandbox: {e}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn real_main() -> Result<ExitCode, String> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().ok_or("missing command")?;
    let mut root: Option<PathBuf> = None;
    let mut toolchain: Option<PathBuf> = None;
    let mut work: Option<PathBuf> = None;
    let mut keep_work = false;
    let mut limits = Limits::default_for(Language::Python);
    let mut wall_ms: Option<u64> = None;
    let mut lpac = false;
    let mut env: Vec<(String, String)> = Vec::new();
    let mut rest: Vec<String> = Vec::new();

    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} needs a value"));
        match a.as_str() {
            "--root" => root = Some(value("--root")?.into()),
            "--toolchain" => toolchain = Some(value("--toolchain")?.into()),
            "--work" => work = Some(value("--work")?.into()),
            "--keep-work" => keep_work = true,
            "--cpu-ms" => limits.cpu_time_ms = parse(&value("--cpu-ms")?)?,
            "--wall-ms" => wall_ms = Some(parse(&value("--wall-ms")?)?),
            "--mem-mb" => limits.memory_mb = parse(&value("--mem-mb")?)?,
            "--procs" => limits.max_processes = parse(&value("--procs")?)?,
            "--lpac" => lpac = true,
            "--env" => {
                let kv = value("--env")?;
                let (k, v) = kv.split_once('=').ok_or("--env needs KEY=VALUE")?;
                env.push((k.to_string(), v.to_string()));
            }
            "--" => {
                rest.extend(args.by_ref());
                break;
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    let root = absolute(&root.ok_or("--root is required")?)?;
    let toolchain = match toolchain {
        Some(t) => absolute(&t)?,
        None => root.join("toolchains").join("python-3.14"),
    };

    match cmd.as_str() {
        "cleanup" => {
            cleanup(&[toolchain]).map_err(|e| e.to_string())?;
            eprintln!("removed AppContainer profile and toolchain ACL entry");
            Ok(ExitCode::SUCCESS)
        }
        "run" => {
            let mut rest = rest.into_iter();
            let program = rest.next().ok_or("missing program after --")?;
            let program = resolve_program(&program, &toolchain);

            let work_dir = match work {
                Some(w) => absolute(&w)?,
                None => {
                    let dir = root
                        .join("runs")
                        .join(format!("cli-{}", std::process::id()));
                    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                    dir
                }
            };

            let mut spec = SandboxSpec::new(&root, program, &work_dir, &limits);
            spec.keep_work_dir = keep_work;
            spec.args = rest.collect();
            spec.read_only_dirs = vec![toolchain];
            spec.less_privileged = lpac;
            spec.env = env;
            if let Some(ms) = wall_ms {
                spec.wall_time = Duration::from_millis(ms);
            }
            let o = run(&spec).map_err(|e| e.to_string())?;

            use std::io::Write;
            let _ = std::io::stdout().write_all(&o.stdout);
            let _ = std::io::stderr().write_all(&o.stderr);
            eprintln!(
                "[kb-sandbox] termination={:?} exit={:#x} user={:?} kernel={:?} wall={:?} peak_mem={} KiB \
                 procs(total={}, max_active={}, after={}) mem_limit_hit={} proc_limit_hit={} lpac={}",
                o.termination,
                o.exit_code,
                o.user_time,
                o.kernel_time,
                o.wall_time,
                o.peak_memory_bytes / 1024,
                o.total_processes,
                o.max_active_processes,
                o.active_processes_after,
                o.memory_limit_hit,
                o.process_limit_hit,
                spec.less_privileged,
            );
            Ok(ExitCode::from(u8::try_from(o.exit_code).unwrap_or(1)))
        }
        other => Err(format!("unknown command {other}")),
    }
}

fn parse<T: std::str::FromStr>(s: &str) -> Result<T, String> {
    s.parse().map_err(|_| format!("not a number: {s}"))
}

fn absolute(p: &Path) -> Result<PathBuf, String> {
    std::path::absolute(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn resolve_program(name: &str, toolchain: &Path) -> PathBuf {
    let p = Path::new(name);
    if p.components().count() > 1 || p.is_absolute() {
        return p.to_path_buf();
    }
    let exe = if p.extension().is_some() {
        name.to_string()
    } else {
        format!("{name}.exe")
    };
    toolchain.join(exe)
}
