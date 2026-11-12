//! The complexity profiler's calibration set (ADR 0024): known solutions, profiled in the
//! Windows sandbox, must get the right class at least 85% of the time, each run in under
//! 15 s. Needs the dev sandbox root with the toolchains:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --test complexity_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use kb_core::Language;
use kb_core::profile::ComplexityClass::{self, *};
use kb_judge::{JudgeEnv, Problem};
use kb_runner::ROOT_MARKER;
use serde_json::{Map, Value};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file(),
        "no sandbox root at {}",
        root.display()
    );
    JudgeEnv::dev(root)
}

/// The committed profile inputs of a problem, in increasing n.
fn inputs(dir: &Path) -> Vec<(u32, Map<String, Value>)> {
    let reference: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("profile/reference.json")).unwrap())
            .unwrap();
    reference["sizes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| {
            let n = n.as_u64().unwrap() as u32;
            let text =
                std::fs::read_to_string(dir.join(format!("profile/inputs/{n}.json"))).unwrap();
            (n, serde_json::from_str(&text).unwrap())
        })
        .collect()
}

/// (problem, solution file, language, expected class)
fn calibration_set() -> Vec<(&'static str, &'static str, Language, ComplexityClass)> {
    let problems: [(&str, ComplexityClass, ComplexityClass); 7] = [
        ("two-sum", Linear, Quadratic),
        ("balanced-streaks", Linear, Quadratic),
        ("fewest-hops", Linear, Quadratic),
        ("thermostat-swings", Linear, Quadratic),
        ("queue-swaps", Linearithmic, Quadratic),
        ("shelf-codes", Linearithmic, Quadratic),
        ("path-tiles", Linear, Exponential),
    ];
    let mut set = Vec::new();
    for (id, fast, slow) in problems {
        for lang in [Language::Python, Language::Rust] {
            set.push((id, "ref", lang, fast));
            set.push((id, "naive", lang, slow));
        }
    }
    set
}

#[test]
fn the_fit_names_the_right_class_at_least_85_percent_of_the_time() {
    let env = env();
    let set = calibration_set();
    let mut hits = 0;
    let mut slowest = Duration::ZERO;
    for &(id, file, lang, want) in &set {
        let dir = repo().join("content/problems").join(id);
        let problem = Problem::load(&dir).unwrap();
        let source =
            std::fs::read_to_string(dir.join(format!("solutions/{file}.{}", lang.extension())))
                .unwrap();
        let started = Instant::now();
        let run = kb_judge::profile::profile(
            &problem,
            lang,
            &source,
            &inputs(&dir),
            &env,
            kb_judge::profile::BUDGET,
        )
        .unwrap();
        let wall = started.elapsed();
        slowest = slowest.max(wall);
        let points: Vec<(u32, u32)> = run.points.iter().map(|p| (p.n, p.median_us)).collect();
        let estimate = kb_judge::complexity::estimate(&points);
        let got = estimate.as_ref().map(|e| e.class);
        let hit = got == Some(want);
        hits += usize::from(hit);
        println!(
            "{} {id:<18} {file:<5} {:<6} want {:<12} got {:<28} {wall:>6.1?} {points:?} stop {:?}",
            if hit { "ok  " } else { "MISS" },
            lang.id(),
            want.notation(),
            estimate
                .as_ref()
                .map(|e| format!(
                    "{} ({:.2}{})",
                    e.class.notation(),
                    e.confidence,
                    e.close_alternative
                        .map(|c| format!(", or {}", c.notation()))
                        .unwrap_or_default()
                ))
                .unwrap_or_else(|| "-".into()),
            run.stopped
        );
        assert!(
            wall < Duration::from_secs(15),
            "{id} {file} {lang:?} took {wall:?}"
        );
    }
    let rate = hits as f64 / set.len() as f64;
    println!(
        "hit rate {hits}/{} = {:.0}%; slowest run {slowest:.1?}",
        set.len(),
        rate * 100.0
    );
    assert!(rate >= 0.85, "hit rate {rate:.2}");
}
