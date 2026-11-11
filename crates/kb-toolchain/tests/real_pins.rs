//! The real pins, downloaded for real (network; opt-in):
//!
//! ```text
//! KB_REAL_PINS=1 cargo test -p kb-toolchain --release --test real_pins -- --nocapture
//! ```
//!
//! 1. Every artifact of every platform, from every mirror URL, through the app's own
//!    downloader (`Downloader::system()`: OS trust store, allow-list, redirects checked):
//!    size and SHA-256 (and npm's sha512) must match the manifest.
//! 2. Every language of this platform installed into a fresh root, as the app does:
//!    check files present, installed sizes within 1% of the manifest.
//!
//! A pin that never passed this must not ship (ADR 0015).

use std::path::Path;

use kb_core::Language;
use kb_toolchain::manifest::{Manifest, Platform};
use kb_toolchain::{CancellationToken, Downloader, InstallOptions, Manager};

fn enabled() -> bool {
    std::env::var("KB_REAL_PINS").is_ok_and(|v| v == "1")
}

fn dir_bytes(p: &Path) -> u64 {
    let mut total = 0;
    let mut stack = vec![p.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let m = std::fs::symlink_metadata(e.path()).unwrap();
            if m.is_dir() {
                stack.push(e.path());
            } else {
                total += m.len();
            }
        }
    }
    total
}

#[tokio::test(flavor = "multi_thread")]
async fn every_pinned_artifact_downloads_and_matches() {
    if !enabled() {
        eprintln!("skipped: set KB_REAL_PINS=1 (downloads about 1 GB)");
        return;
    }
    let m = Manifest::builtin();
    let dl = Downloader::system().unwrap();
    let mut failures = Vec::new();
    println!("| artifact | platform | url | bytes | sha256 | result |");
    println!("| --- | --- | --- | --- | --- | --- |");
    for a in &m.artifacts {
        for url in &a.urls {
            // One URL at a time: the artifact with only this mirror.
            let mut one = a.clone();
            one.urls = vec![url.clone()];
            let dir = tempfile::tempdir().unwrap();
            let cancel = CancellationToken::new();
            let r = kb_toolchain::fetch_for_tests(&dl, &one, dir.path(), &cancel).await;
            let result = match &r {
                Ok(()) => "OK: size and SHA-256 match".to_string(),
                Err(e) => {
                    failures.push(format!("{} {url}: {e}", a.id));
                    format!("FAILED: {e}")
                }
            };
            println!(
                "| {} | {} | {} | {} | {}… | {} |",
                a.id,
                a.platform,
                url,
                a.size,
                &a.sha256[..16],
                result
            );
        }
    }
    assert!(
        failures.is_empty(),
        "pins that did not verify:\n{}",
        failures.join("\n")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn every_language_installs_from_the_real_pins() {
    if !enabled() {
        eprintln!("skipped: set KB_REAL_PINS=1");
        return;
    }
    let Some(platform) = Platform::current() else {
        return;
    };
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(".katabox-sandbox-root"), b"").unwrap();
    let mgr = Manager::open(dir.path()).unwrap();
    let dl = Downloader::system().unwrap();
    let langs: Vec<Language> = mgr
        .status()
        .iter()
        .filter(|s| s.available)
        .map(|s| s.language)
        .collect();
    mgr.install(&langs, InstallOptions::default(), &dl, &|_| {})
        .await
        .unwrap();
    println!("| component | dest | installed bytes | manifest | result |");
    println!("| --- | --- | --- | --- | --- |");
    let m = Manifest::builtin();
    let mut bad = Vec::new();
    for s in mgr.status().iter().filter(|s| s.available) {
        assert!(s.installed, "{} not installed", s.language);
        for id in &s.components {
            let c = m.component(id, platform).unwrap();
            let path = mgr.layout().component_dir(id).unwrap();
            let got = dir_bytes(&path);
            let within = got.abs_diff(c.installed_size) * 100 <= c.installed_size;
            println!(
                "| {id} | {} | {got} | {} | {} |",
                c.dest,
                c.installed_size,
                if within { "OK" } else { "SIZE OFF" }
            );
            if !within {
                bad.push(format!("{id}: {got} vs {}", c.installed_size));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "installed sizes off by more than 1%: {bad:?}"
    );
}
