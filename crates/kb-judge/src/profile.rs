//! Profile runs (ADR 0024): the user's code at increasing sizes, through the ordinary
//! judge path (one sandboxed run per size, no kb-runner change).
//!
//! Each run holds the same input [`REPETITIONS`] times; the point is the median of the
//! per-call times measured inside the harness (no process start, no parsing). Sizes run
//! from small to large under a total wall-clock budget: before each size the cost is
//! predicted from the previous point (as if it grew like n², plus the run's overhead),
//! and the profile stops early rather than run past the budget.

use std::time::{Duration, Instant};

use kb_core::profile::{ProfilePoint, ProfileRun, ProfileStop};
use kb_core::{Language, ProblemMode, TestCase, TestKind, Verdict};
use serde_json::{Map, Value};

use crate::JudgeError;
use crate::judge::{JudgeEnv, judge};
use crate::languages::definition;
use crate::limits;
use crate::problem::Problem;

/// One profile size and its input.
pub type SizedInput = (u32, Map<String, Value>);

/// Repetitions per size (the median is reported).
pub const REPETITIONS: usize = 3;
/// The whole profile's wall-clock budget (the build plan's goal is under 15 s).
pub const BUDGET: Duration = Duration::from_secs(12);
/// Wall-clock limit of a profile run, in multiples of its CPU cap (the app's runs use 3).
pub const MEASURE_WALL_FACTOR: u32 = 10;
/// No single size may use more CPU than this.
pub const SIZE_CAP_MS: u32 = 5_000;

/// The base limit whose CPU cap is at least `cap_ms` for `lang` at `scale`.
fn base_for_cap(lang: Language, scale: f64, cap_ms: u32) -> Result<u32, JudgeError> {
    let def = definition(lang)?;
    let mut base = 1u32;
    let mut step = 1u32 << 20;
    // Largest base whose cap stays within `cap_ms` (binary search on a monotone cap).
    while step > 0 {
        if limits::cpu_cap_ms(base + step, &def, scale) <= cap_ms {
            base += step;
        }
        step /= 2;
    }
    Ok(base)
}

fn median(mut v: Vec<u32>) -> u32 {
    v.sort_unstable();
    v[v.len() / 2]
}

/// The predicted wall time of the next size from the last measured one.
fn predict(last: &ProfilePoint, last_wall: Duration, next_n: u32) -> Duration {
    let ratio = f64::from(next_n) / f64::from(last.n.max(1));
    let work = f64::from(last.median_us) * REPETITIONS as f64;
    let overhead = (last_wall.as_secs_f64() * 1e6 - work).max(0.0);
    Duration::from_micros((overhead + work * ratio * ratio) as u64)
}

/// Points a program profile needs before its estimate is shown (as for a function:
/// `complexity::MIN_POINTS`).
pub const PROGRAM_MIN_POINTS: usize = 3;
/// A program profile is unreliable when its startup is more than this share of the
/// largest point's whole time (startup plus work): the work at the largest size must at
/// least equal the startup, or run-to-run jitter in the startup decides the shape.
pub const PROGRAM_STARTUP_SHARE: f64 = 0.5;

/// Whether a program profile's estimate is too uncertain to show.
pub fn program_unreliable(points: &[ProfilePoint], startup_us: u32) -> bool {
    let Some(largest) = points.iter().map(|p| p.median_us).max() else {
        return true;
    };
    let whole = f64::from(largest) + f64::from(startup_us);
    points.len() < PROGRAM_MIN_POINTS || f64::from(startup_us) > PROGRAM_STARTUP_SHARE * whole
}

/// The `(n, µs)` points a program profile's class is fitted to: each sample with the
/// startup added back. The fit's constant term absorbs the startup, and its relative
/// weights (1/t²) then match the noise, which is the startup's jitter, a few ms whatever
/// the size. Fitted without it, a small point after subtraction (1 ms ± 3 ms) outweighs
/// everything else and can make a quadratic look constant. The chart and
/// [`program_unreliable`] keep the subtracted samples.
pub fn program_fit_points(points: &[ProfilePoint], startup_us: u32) -> Vec<(u32, u32)> {
    points
        .iter()
        .map(|p| (p.n, p.median_us.saturating_add(startup_us)))
        .collect()
}

/// The smallest program in each language: what starting a process costs.
fn empty_program(lang: Language) -> &'static str {
    match lang {
        Language::Rust => {
            "fn main() {}
"
        }
        Language::Php => {
            "<?php
"
        }
        _ => {
            "
"
        }
    }
}

/// A program problem's startup: the language's empty program on empty stdin, the
/// fastest of three runs (0 if it cannot run, so nothing is taken off).
fn program_startup_us(
    problem: &Problem,
    lang: Language,
    env: &JudgeEnv,
) -> Result<u32, JudgeError> {
    let mut input = Map::new();
    input.insert(
        crate::problem::STDIN_PARAM.into(),
        Value::String(String::new()),
    );
    let tests: Vec<TestCase> = (0..REPETITIONS)
        .map(|i| TestCase {
            kind: TestKind::Custom,
            index: i as u32 + 1,
            input: input.clone(),
            expected: Value::Null,
        })
        .collect();
    let report = judge(problem, &tests, lang, empty_program(lang), env)?;
    Ok(report
        .tests
        .iter()
        .filter(|t| matches!(t.result.verdict, Verdict::Accepted | Verdict::WrongAnswer))
        .map(|t| u32::try_from(t.result.time_us).unwrap_or(u32::MAX))
        .min()
        .unwrap_or(0))
}

/// Profile `source` on the inputs, each `(n, input)`, in increasing n.
pub fn profile(
    problem: &Problem,
    lang: Language,
    source: &str,
    inputs: &[SizedInput],
    env: &JudgeEnv,
    budget: Duration,
) -> Result<ProfileRun, JudgeError> {
    // A measurement: the CPU cap per size decides a time limit, and the budget below
    // bounds the wall time, so a busy machine must not end a size early on the wall clock.
    let mut measuring = env.clone();
    measuring.wall_factor = measuring.wall_factor.max(MEASURE_WALL_FACTOR);
    // A program's samples are its runs' wall times, less the startup (see
    // `JudgeEnv::program_wall_time`); runs never overlap (the sandbox lock).
    measuring.program_wall_time = true;
    let env = &measuring;
    let started = Instant::now();
    let queued_at_start = crate::sandbox::queued();
    // Time queued behind other sandbox users is not this profile's: leave it out.
    let queued = || crate::sandbox::queued().saturating_sub(queued_at_start);
    let queued_ms = || u32::try_from(queued().as_millis()).unwrap_or(u32::MAX);
    // A program's time includes starting its process: measure that once and take it off.
    let startup_us = if problem.mode == ProblemMode::Program {
        program_startup_us(problem, lang, env)?
    } else {
        0
    };
    let mut points: Vec<ProfilePoint> = Vec::new();
    let mut last_wall = Duration::ZERO;
    for (k, (n, input)) in inputs.iter().enumerate() {
        let spent = started.elapsed().saturating_sub(queued());
        let left = budget.saturating_sub(spent);
        if let Some(last) = points.last()
            && predict(last, last_wall, *n) > left
        {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Budget { n: *n }),
                queued_ms: queued_ms(),
                startup_us,
            });
        }
        if k > 0 && left < Duration::from_millis(200) {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Budget { n: *n }),
                queued_ms: queued_ms(),
                startup_us,
            });
        }
        let cap_ms = (left.as_millis() as u32).clamp(500, SIZE_CAP_MS);
        let mut sized = problem.clone();
        sized.time_limit_ms = base_for_cap(lang, env.time_scale, cap_ms)?;
        let tests: Vec<TestCase> = (0..REPETITIONS)
            .map(|i| TestCase {
                kind: TestKind::Custom,
                index: i as u32 + 1,
                input: input.clone(),
                expected: Value::Null,
            })
            .collect();
        let run_started = Instant::now();
        let queued_before = queued();
        let report = judge(&sized, &tests, lang, source, env)?;
        last_wall = run_started
            .elapsed()
            .saturating_sub(queued().saturating_sub(queued_before));
        if report.verdict == Verdict::CompilationError {
            let output = report.compile_output.map(|m| m.text).unwrap_or_default();
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Compile { output }),
                queued_ms: queued_ms(),
                startup_us,
            });
        }
        let stop = report.tests.iter().find_map(|t| match t.result.verdict {
            // Outputs are not checked here (expected is null): any return value counts.
            Verdict::Accepted | Verdict::WrongAnswer => None,
            Verdict::TimeLimitExceeded => Some(ProfileStop::TimeLimit { n: *n }),
            v => Some(ProfileStop::Failed { n: *n, verdict: v }),
        });
        if let Some(stop) = stop {
            return Ok(ProfileRun {
                points,
                stopped: Some(stop),
                queued_ms: queued_ms(),
                startup_us,
            });
        }
        if report.tests.len() < REPETITIONS {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::TimeLimit { n: *n }),
                queued_ms: queued_ms(),
                startup_us,
            });
        }
        let samples: Vec<u32> = report
            .tests
            .iter()
            .map(|t| {
                u32::try_from(t.result.time_us)
                    .unwrap_or(u32::MAX)
                    .saturating_sub(startup_us)
            })
            .collect();
        points.push(ProfilePoint {
            n: *n,
            median_us: median(samples.clone()),
            samples_us: samples,
        });
    }
    Ok(ProfileRun {
        points,
        stopped: None,
        queued_ms: queued_ms(),
        startup_us,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A quadratic program measured in the calibration run (depot-routes, naive.rs):
    /// startup 21 ms, taken off. Fitted on the subtracted samples, the 1 ms point
    /// dominated and the class came out O(1); with the startup back it is O(n²).
    #[test]
    fn program_points_are_fitted_with_their_startup() {
        use kb_core::profile::ComplexityClass;
        let points: Vec<ProfilePoint> = [
            (500, 956),
            (1000, 10_254),
            (2000, 56_977),
            (4000, 269_467),
            (7000, 1_287_487),
        ]
        .into_iter()
        .map(|(n, us)| ProfilePoint {
            n,
            median_us: us,
            samples_us: vec![us],
        })
        .collect();
        let fitted = program_fit_points(&points, 21_284);
        assert_eq!(fitted[0], (500, 22_240));
        let e = crate::complexity::estimate(&fitted).unwrap();
        assert_eq!(e.class, ComplexityClass::Quadratic, "{e:?}");
        assert!(!program_unreliable(&points, 21_284));
    }

    #[test]
    fn the_cap_is_inverted_per_language() {
        let rust = definition(Language::Rust).unwrap();
        let python = definition(Language::Python).unwrap();
        let base = base_for_cap(Language::Rust, 1.0, 5_000).unwrap();
        assert!(limits::cpu_cap_ms(base, &rust, 1.0) <= 5_000);
        assert!(limits::cpu_cap_ms(base + 1, &rust, 1.0) > 5_000);
        let base = base_for_cap(Language::Python, 1.0, 5_000).unwrap();
        assert!(limits::cpu_cap_ms(base, &python, 1.0) <= 5_000);
    }

    #[test]
    fn medians_and_predictions() {
        assert_eq!(median(vec![30, 10, 20]), 20);
        let p = ProfilePoint {
            n: 1_000,
            median_us: 1_000,
            samples_us: vec![1_000; 3],
        };
        // 3 ms of work plus 0.5 s of overhead; 10× the size → up to 100× the work.
        let next = predict(&p, Duration::from_micros(503_000), 10_000);
        assert_eq!(next, Duration::from_micros(500_000 + 300_000));
    }
}
