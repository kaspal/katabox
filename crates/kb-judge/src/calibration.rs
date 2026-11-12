//! Machine calibration (ADR 0017): a short, fixed CPU benchmark, run in the sandbox like
//! user code, tells how fast this machine is compared with the reference machine the
//! problems' limits were tuned on. The judge multiplies every time limit by the result
//! (`JudgeEnv::time_scale`), clamped to [`limits::MIN_SCALE`], [`limits::MAX_SCALE`].
//!
//! The benchmark is Python, or JavaScript on Node when Python isn't installed (one of the
//! two runtimes is always needed to solve problems in those languages; Node also comes
//! with TypeScript). Each has its own reference time, measured on the reference machine.
//! The workload mixes integer arithmetic, hash-map updates, string building and sorting,
//! and returns a checksum, so a run that skipped work is caught. It is timed in-process
//! with a high-resolution clock; the fastest of [`ROUNDS`] rounds counts, which filters
//! out other programs competing for the CPU.

use kb_core::Language;
use serde::Deserialize;

use crate::JudgeError;
use crate::judge::JudgeEnv;
use crate::limits;
use crate::tools::{ScriptLimits, run_script};

/// Timed rounds per benchmark run.
pub const ROUNDS: u32 = 5;

/// The runtimes the benchmark can use, in order of preference.
pub const BENCHMARK_LANGUAGES: [Language; 2] = [Language::Python, Language::TypeScript];

struct Bench {
    script: &'static str,
    /// Loop iterations per round.
    n: u64,
    /// The workload's result for `n`.
    checksum: u64,
    /// Fastest round on the reference machine, in microseconds.
    reference_us: u64,
}

fn bench(language: Language) -> Result<Bench, JudgeError> {
    match language {
        Language::Python => Ok(Bench {
            script: PYTHON,
            n: 2_500_000,
            checksum: PYTHON_CHECKSUM,
            reference_us: PYTHON_REFERENCE_US,
        }),
        Language::TypeScript => Ok(Bench {
            script: JAVASCRIPT,
            n: 8_000_000,
            checksum: JAVASCRIPT_CHECKSUM,
            reference_us: JAVASCRIPT_REFERENCE_US,
        }),
        other => Err(JudgeError::Unsupported(other)),
    }
}

// Reference machine results (see ADR 0017). Re-measure with
// `cargo test -p kb-judge --test calibration_windows -- --nocapture` when the workload or
// a runtime version changes.
const PYTHON_CHECKSUM: u64 = 468_927;
const PYTHON_REFERENCE_US: u64 = 216_000;
const JAVASCRIPT_CHECKSUM: u64 = 612_307;
const JAVASCRIPT_REFERENCE_US: u64 = 198_000;

const PYTHON: &str = r#"import json, sys, time

def work(n):
    acc = 0
    table = {}
    for i in range(n):
        acc = (acc * 31 + i) % 1000003
        k = acc & 4095
        table[k] = table.get(k, 0) + i
    words = sorted(str(v * 7 % 100003) for v in table.values())
    text = ",".join(words)
    return (acc + len(text) + sum(table.values()) % 1000003) % 1000000007

n, rounds = json.loads(sys.stdin.read())
times, checksum = [], None
for _ in range(rounds):
    start = time.perf_counter_ns()
    checksum = work(n)
    times.append((time.perf_counter_ns() - start) // 1000)
print(json.dumps({"rounds_us": times, "checksum": checksum}))
"#;

const JAVASCRIPT: &str = r#"function work(n) {
  let acc = 0;
  const table = new Map();
  for (let i = 0; i < n; i++) {
    acc = (acc * 31 + i) % 1000003;
    const k = acc & 4095;
    table.set(k, (table.get(k) ?? 0) + i);
  }
  const words = [...table.values()].map((v) => String((v * 7) % 100003)).sort();
  const text = words.join(",");
  let sum = 0;
  for (const v of table.values()) sum += v;
  return (acc + text.length + (sum % 1000003)) % 1000000007;
}

const chunks = [];
process.stdin.on("data", (c) => chunks.push(c));
process.stdin.on("end", () => {
  const [n, rounds] = JSON.parse(Buffer.concat(chunks).toString());
  const times = [];
  let checksum = null;
  for (let r = 0; r < rounds; r++) {
    const start = process.hrtime.bigint();
    checksum = work(n);
    times.push(Number((process.hrtime.bigint() - start) / 1000n));
  }
  process.stdout.write(JSON.stringify({ rounds_us: times, checksum }));
});
"#;

/// One benchmark run's result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calibration {
    /// Measured time ÷ the reference machine's time: above 1 is a slower machine.
    pub factor: f64,
    /// What the judge applies: `factor` clamped to the allowed range.
    pub applied: f64,
    /// The runtime that ran the benchmark (Python, or TypeScript for Node).
    pub runtime: Language,
    /// Fastest round here, in microseconds.
    pub measured_us: u64,
    /// Fastest round on the reference machine, in microseconds.
    pub reference_us: u64,
}

impl Calibration {
    /// The calibration for `measured_us` on `runtime`'s benchmark.
    pub fn from_measurement(runtime: Language, measured_us: u64) -> Result<Self, JudgeError> {
        let reference_us = bench(runtime)?.reference_us;
        let factor = measured_us as f64 / reference_us as f64;
        Ok(Calibration {
            factor,
            applied: limits::clamp_scale(factor),
            runtime,
            measured_us,
            reference_us,
        })
    }
}

#[derive(Deserialize)]
struct BenchOutput {
    rounds_us: Vec<u64>,
    checksum: u64,
}

/// Run `language`'s benchmark in the sandbox: the fastest round in microseconds and the
/// checksum it computed (not yet compared with the expected one).
pub fn run_benchmark(env: &JudgeEnv, language: Language) -> Result<(u64, u64), JudgeError> {
    let b = bench(language)?;
    let main = b.script;
    let stdin =
        serde_json::to_vec(&(b.n, ROUNDS)).map_err(|e| JudgeError::Internal(e.to_string()))?;
    let limits = ScriptLimits {
        cpu_time_ms: 60_000,
        memory_mb: 256,
        max_output_bytes: 64 * 1024,
    };
    let out = run_script(env, language, main, &[], &stdin, limits)?;
    if !out.succeeded() {
        let tail: Vec<&str> = out.stderr.lines().rev().take(4).collect();
        return Err(JudgeError::Internal(format!(
            "the {language} benchmark failed ({:?}, exit code {}): {}",
            out.termination,
            out.exit_code,
            tail.into_iter().rev().collect::<Vec<_>>().join(" / ")
        )));
    }
    let parsed: BenchOutput = serde_json::from_slice(&out.stdout).map_err(|e| {
        JudgeError::Internal(format!(
            "the {language} benchmark printed something else: {e}"
        ))
    })?;
    let best = parsed.rounds_us.iter().copied().min().unwrap_or(0).max(1);
    Ok((best, parsed.checksum))
}

/// Measure this machine with the first benchmark runtime `env` has.
pub fn calibrate(env: &JudgeEnv) -> Result<Calibration, JudgeError> {
    let Some(runtime) = BENCHMARK_LANGUAGES
        .into_iter()
        .find(|l| env.toolchain_program(*l).is_ok())
    else {
        return Err(JudgeError::Toolchain(
            "calibration needs Python or Node (TypeScript) installed".into(),
        ));
    };
    calibrate_with(env, runtime)
}

/// Measure this machine with `runtime`'s benchmark.
pub fn calibrate_with(env: &JudgeEnv, runtime: Language) -> Result<Calibration, JudgeError> {
    let (best, checksum) = run_benchmark(env, runtime)?;
    let expected = bench(runtime)?.checksum;
    if checksum != expected {
        return Err(JudgeError::Internal(format!(
            "the {runtime} benchmark returned {checksum}, expected {expected}"
        )));
    }
    Calibration::from_measurement(runtime, best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_is_measured_over_reference_and_clamped() {
        let r = PYTHON_REFERENCE_US;
        let same = Calibration::from_measurement(Language::Python, r).unwrap();
        assert_eq!((same.factor, same.applied), (1.0, 1.0));
        assert_eq!(same.reference_us, r);

        let slow = Calibration::from_measurement(Language::Python, r * 2).unwrap();
        assert_eq!((slow.factor, slow.applied), (2.0, 2.0));

        let very_slow = Calibration::from_measurement(Language::Python, r * 5).unwrap();
        assert_eq!(
            (very_slow.factor, very_slow.applied),
            (5.0, limits::MAX_SCALE)
        );

        let fast = Calibration::from_measurement(Language::TypeScript, JAVASCRIPT_REFERENCE_US / 4)
            .unwrap();
        assert!(fast.factor < 0.3, "{fast:?}");
        assert_eq!(fast.applied, limits::MIN_SCALE);
        assert_eq!(fast.runtime, Language::TypeScript);
    }

    #[test]
    fn only_python_and_node_benchmark() {
        assert!(Calibration::from_measurement(Language::Rust, 1).is_err());
        assert!(Calibration::from_measurement(Language::Php, 1).is_err());
    }

    #[test]
    fn calibrate_needs_a_runtime() {
        let env = JudgeEnv::new(std::env::temp_dir().join("kbj-no-runtimes"));
        assert!(matches!(calibrate(&env), Err(JudgeError::Toolchain(_))));
    }
}
