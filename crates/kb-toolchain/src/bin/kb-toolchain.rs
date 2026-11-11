//! `kb-toolchain`: install languages into a sandbox root, the same way the app does
//! (scripts/fetch-toolchains.* wrap it).
//!
//! ```text
//! kb-toolchain install [--root DIR] [--create-root] [--runtimes-only] (--all | python typescript php rust)
//! kb-toolchain uninstall [--root DIR] LANG
//! kb-toolchain list [--root DIR]
//! ```
//! The default root is the dev root, `target/katabox`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;

use kb_core::Language;
use kb_toolchain::layout::ROOT_MARKER;
use kb_toolchain::{ComponentPhase, Downloader, InstallEvent, InstallOptions, Manager};

fn dev_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("katabox")
}

struct Args {
    cmd: String,
    root: PathBuf,
    root_given: bool,
    create_root: bool,
    runtimes_only: bool,
    all: bool,
    languages: Vec<Language>,
}

fn parse() -> Result<Args, String> {
    let mut it = std::env::args().skip(1);
    let cmd = it
        .next()
        .ok_or("usage: kb-toolchain <install|uninstall|list> …")?;
    let mut a = Args {
        cmd,
        root: dev_root(),
        root_given: false,
        create_root: false,
        runtimes_only: false,
        all: false,
        languages: Vec::new(),
    };
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--root" => {
                a.root = it.next().ok_or("--root needs a folder")?.into();
                a.root_given = true;
            }
            "--create-root" => a.create_root = true,
            "--runtimes-only" => a.runtimes_only = true,
            "--all" => a.all = true,
            l => a
                .languages
                .push(Language::from_str(l).map_err(|e| e.to_string())?),
        }
    }
    Ok(a)
}

/// Create and mark the root only for the dev default or when asked (as the app does:
/// never mark a folder the user named without being told to).
fn prepare_root(a: &Args) -> Result<(), String> {
    if a.root.join(ROOT_MARKER).is_file() {
        return Ok(());
    }
    if a.root_given && !a.create_root {
        return Err(format!(
            "{} is not a sandbox root (no {ROOT_MARKER}); pass --create-root to make it one",
            a.root.display()
        ));
    }
    std::fs::create_dir_all(a.root.join("runs")).map_err(|e| e.to_string())?;
    std::fs::write(a.root.join(ROOT_MARKER), b"").map_err(|e| e.to_string())?;
    Ok(())
}

fn mb(n: u64) -> String {
    format!("{:.1} MB", n as f64 / 1_000_000.0)
}

fn main() -> ExitCode {
    let args = match parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = prepare_root(&args) {
        eprintln!("{e}");
        return ExitCode::from(2);
    }
    let manager = match Manager::open(&args.root) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let result = rt.block_on(run(&args, &manager));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(a: &Args, m: &Manager) -> Result<(), String> {
    match a.cmd.as_str() {
        "list" => {
            println!("root: {}", m.root().display());
            for s in m.status() {
                let state = if !s.available {
                    "not available on this platform".to_string()
                } else if s.installed {
                    format!("installed ({})", mb(s.installed_bytes))
                } else if s.runtime_installed {
                    "runtime only".to_string()
                } else {
                    format!("not installed (download {})", mb(s.download_bytes))
                };
                println!("  {:<11} {state}", s.language.id());
            }
            Ok(())
        }
        "install" => {
            let languages: Vec<Language> = if a.all {
                m.status()
                    .into_iter()
                    .filter(|s| s.available)
                    .map(|s| s.language)
                    .collect()
            } else {
                a.languages.clone()
            };
            if languages.is_empty() {
                return Err("name languages to install, or --all".into());
            }
            let dl = Downloader::system().map_err(|e| e.to_string())?;
            let last = std::sync::Mutex::new(std::collections::HashMap::<String, u64>::new());
            let on_event = |e: InstallEvent| match e {
                InstallEvent::Plan { items } => {
                    if items.is_empty() {
                        println!("Everything requested is already installed.");
                    }
                    for i in items {
                        println!(
                            "  {:<28} download {:>10}  installed {:>10}",
                            i.component,
                            mb(i.download_bytes),
                            mb(i.installed_bytes)
                        );
                    }
                }
                InstallEvent::Progress {
                    component,
                    phase,
                    done,
                    total,
                } => match phase {
                    ComponentPhase::Downloading if total > 0 => {
                        let pct = done * 100 / total;
                        let mut last = last.lock().unwrap();
                        let prev = last.entry(component.clone()).or_insert(u64::MAX);
                        if *prev == u64::MAX || pct >= *prev + 10 || pct == 100 && *prev != 100 {
                            *prev = pct;
                            println!("{component}: downloading {pct}%");
                        }
                    }
                    ComponentPhase::Queued | ComponentPhase::Downloading => {}
                    other => println!("{component}: {}", format!("{other:?}").to_lowercase()),
                },
            };
            let opts = InstallOptions {
                runtimes_only: a.runtimes_only,
            };
            m.install(&languages, opts, &dl, &on_event)
                .await
                .map_err(|e| e.to_string())?;
            println!("Done: {}", m.root().display());
            Ok(())
        }
        "uninstall" => {
            for &l in &a.languages {
                let removed = m.uninstall(l).map_err(|e| e.to_string())?;
                println!(
                    "{l}: removed {}",
                    if removed.is_empty() {
                        "nothing".to_string()
                    } else {
                        removed.join(", ")
                    }
                );
            }
            Ok(())
        }
        other => Err(format!("unknown command {other:?}")),
    }
}
