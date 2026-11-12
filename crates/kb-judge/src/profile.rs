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
use kb_core::{Language, TestCase, TestKind, Verdict};
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

/// Profile `source` on the inputs, each `(n, input)`, in increasing n.
pub fn profile(
    problem: &Problem,
    lang: Language,
    source: &str,
    inputs: &[SizedInput],
    env: &JudgeEnv,
    budget: Duration,
) -> Result<ProfileRun, JudgeError> {
    let started = Instant::now();
    let mut points: Vec<ProfilePoint> = Vec::new();
    let mut last_wall = Duration::ZERO;
    for (k, (n, input)) in inputs.iter().enumerate() {
        let spent = started.elapsed();
        let left = budget.saturating_sub(spent);
        if let Some(last) = points.last()
            && predict(last, last_wall, *n) > left
        {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Budget { n: *n }),
            });
        }
        if k > 0 && left < Duration::from_millis(200) {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Budget { n: *n }),
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
        let report = judge(&sized, &tests, lang, source, env)?;
        last_wall = run_started.elapsed();
        if report.verdict == Verdict::CompilationError {
            let output = report.compile_output.map(|m| m.text).unwrap_or_default();
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::Compile { output }),
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
            });
        }
        if report.tests.len() < REPETITIONS {
            return Ok(ProfileRun {
                points,
                stopped: Some(ProfileStop::TimeLimit { n: *n }),
            });
        }
        let samples: Vec<u32> = report
            .tests
            .iter()
            .map(|t| u32::try_from(t.result.time_us).unwrap_or(u32::MAX))
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
