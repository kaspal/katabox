//! The stress lab (expansion M8, ADR 0036): your solution against your own brute force on
//! many small inputs, until they disagree. Modelled on the shrinker (ADR 0030):
//! 1. inputs come from the problem's gen.py in the sandbox (checked by validate.py), or,
//!    without a Python runtime, from `kb_judge::randgen` (not checked: the UI says so);
//! 2. the brute force runs on them as custom tests (expected unknown); its outputs become
//!    the expected outputs;
//! 3. the solution runs on them through the problem's checker; the first disagreement
//!    stops the run.
//!
//! The run holds the judge slot (never overlaps a Run, Submit or shrink), uses the
//! ordinary judge path (one sandbox user at a time), checks Stop before every sandbox run,
//! and stops after [`MAX_ROUNDS`] rounds or [`MAX_TIME`].

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use kb_core::{JudgeEvent, Language, Message, ProblemMode, TestCase, TestKind, Verdict};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{App, AppError, AppErrorCode, CustomInput, Result};

/// Rounds one stress run may use.
pub const MAX_ROUNDS: u32 = 40;
/// Wall time one stress run may use (it finishes the run in progress).
pub const MAX_TIME: Duration = Duration::from_secs(60);
/// Inputs per round.
pub const BATCH: usize = 16;
/// The largest brute force kept (and imported).
pub const MAX_BRUTE_BYTES: usize = 64 * 1024;

/// Where the inputs come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum StressSource {
    /// The problem's gen.py, checked by its validate.py.
    Generator,
    /// Small random values from the signature, not checked against the constraints.
    Random,
}

/// Progress, after each round.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StressEvent {
    pub rounds: u32,
    pub max_rounds: u32,
    pub tested: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum StressStop {
    /// The solution and the brute force disagree on `input`.
    Found,
    /// The brute force itself failed (error, time limit, does not compile) on `input`.
    BruteFailed,
    /// No disagreement within the round and time budget.
    Budget,
    /// The user pressed Stop.
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StressResult {
    pub stop: StressStop,
    pub source: StressSource,
    pub rounds: u32,
    /// Inputs both programs ran on.
    pub tested: u32,
    /// Found / BruteFailed: the input.
    pub input: Option<CustomInput>,
    /// Found: the brute force's output.
    pub expected: Option<Value>,
    /// Found: the solution's output, when it returned one; BruteFailed: the brute's.
    pub actual: Option<Value>,
    pub verdict: Option<Verdict>,
    pub message: Option<Message>,
}

fn case(index: usize, input: Map<String, Value>, expected: Value) -> TestCase {
    TestCase {
        kind: TestKind::Custom,
        index: u32::try_from(index + 1).unwrap_or(u32::MAX),
        input,
        expected,
    }
}

fn unavailable(msg: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::StressUnavailable, msg.into())
}

impl App {
    /// Stop a running stress test after its current sandbox run.
    pub fn cancel_stress(&self) {
        self.stress_cancel.store(true, Ordering::SeqCst);
    }

    /// The saved brute force for `problem` in `language`.
    pub async fn stress_brute(&self, problem: &str, language: Language) -> Result<Option<String>> {
        Ok(self
            .store
            .user
            .stress_brute(problem, language.id())
            .await?
            .map(|b| b.code))
    }

    /// Keep the brute force for next time.
    pub async fn save_stress_brute(
        &self,
        problem: &str,
        language: Language,
        code: &str,
    ) -> Result<()> {
        if code.len() > MAX_BRUTE_BYTES {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "brute force too long",
            ));
        }
        self.problem(problem).await?;
        self.store
            .user
            .put_stress_brute(&kb_store::stress::StressBruteRow {
                problem_id: problem.into(),
                language: language.id().into(),
                code: code.into(),
                updated_at: self.now_ms(),
            })
            .await?;
        Ok(())
    }

    /// Run `code` against `brute` on generated inputs until they disagree.
    pub async fn stress<F>(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        brute: &str,
        mut on_event: F,
    ) -> Result<StressResult>
    where
        F: FnMut(StressEvent) + Send,
    {
        let _one = self.judge_slot()?;
        self.ensure_interview_language(problem, language).await?;
        self.ensure_calibrated().await;
        self.stress_cancel.store(false, Ordering::SeqCst);
        if brute.len() > MAX_BRUTE_BYTES {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "brute force too long",
            ));
        }
        let p = self.problem(problem).await?;
        if p.mode == ProblemMode::Sql {
            return Err(unavailable("SQL problems have no stress lab"));
        }
        let python = self.toolchains.runtime_installed(Language::Python);
        let generator = if python {
            self.store.content.generator(&p.id).await?
        } else {
            None
        };
        let validator = if python {
            self.store.content.validator(&p.id).await?
        } else {
            None
        };
        let source = if generator.is_some() {
            StressSource::Generator
        } else {
            StressSource::Random
        };
        if p.mode == ProblemMode::Program && generator.is_none() {
            return Err(unavailable(
                "a program problem needs its gen.py, and so the Python runtime",
            ));
        }
        let started = Instant::now();
        let cancelled = || self.stress_cancel.load(Ordering::SeqCst);
        let mut rounds = 0u32;
        let mut tested = 0u32;
        let result = |stop, rounds, tested| StressResult {
            stop,
            source,
            rounds,
            tested,
            input: None,
            expected: None,
            actual: None,
            verdict: None,
            message: None,
        };
        // A program's tests expect text; the brute's run only collects its outputs.
        let unknown = if p.mode == ProblemMode::Program {
            Value::String(String::new())
        } else {
            Value::Null
        };

        loop {
            if cancelled() {
                return Ok(result(StressStop::Cancelled, rounds, tested));
            }
            if rounds >= MAX_ROUNDS || started.elapsed() >= MAX_TIME {
                return Ok(result(StressStop::Budget, rounds, tested));
            }
            let seed0 = u64::from(rounds) * BATCH as u64 + 1;
            // 1. Inputs.
            let mut inputs: Vec<Map<String, Value>> = match &generator {
                Some(gen_src) => {
                    let env = self.env();
                    let gen_src = gen_src.clone();
                    let seeds: Vec<u64> = (seed0..seed0 + BATCH as u64).collect();
                    tokio::task::spawn_blocking(move || {
                        kb_judge::generate_inputs(&env, &gen_src, &seeds, "small", limits())
                    })
                    .await??
                }
                None => kb_judge::randgen::inputs(&p.signature, seed0, BATCH),
            };
            if let Some(v) = &validator {
                if cancelled() {
                    return Ok(result(StressStop::Cancelled, rounds, tested));
                }
                let env = self.env();
                let v = v.clone();
                let given = inputs.clone();
                let answers = tokio::task::spawn_blocking(move || {
                    let refs: Vec<&Map<String, Value>> = given.iter().collect();
                    kb_judge::validate_inputs(&env, &v, &refs, limits())
                })
                .await??;
                inputs = inputs
                    .into_iter()
                    .zip(answers)
                    .filter_map(|(x, a)| a.is_none().then_some(x))
                    .collect();
            }
            if p.mode == ProblemMode::Function {
                inputs.retain(|x| kb_judge::check_input(x, &p.signature).is_ok());
            }
            rounds += 1;
            if inputs.is_empty() {
                continue;
            }
            if cancelled() {
                return Ok(result(StressStop::Cancelled, rounds, tested));
            }
            // 2. The brute force's outputs.
            let tests: Vec<TestCase> = inputs
                .iter()
                .enumerate()
                .map(|(i, x)| case(i, x.clone(), unknown.clone()))
                .collect();
            let report = self
                .judge(p.clone(), tests, language, brute, |_: JudgeEvent| {})
                .await?;
            if report.verdict == Verdict::CompilationError {
                return Ok(StressResult {
                    verdict: Some(Verdict::CompilationError),
                    message: report.compile_output,
                    ..result(StressStop::BruteFailed, rounds, tested)
                });
            }
            let mut answered = Vec::new();
            for (i, x) in inputs.into_iter().enumerate() {
                let Some(o) = report.tests.get(i) else { break };
                match (o.result.verdict, &o.actual) {
                    (Verdict::Accepted | Verdict::WrongAnswer, Some(e)) => {
                        answered.push((x, e.clone()))
                    }
                    (v, actual) => {
                        return Ok(StressResult {
                            input: Some(x),
                            actual: actual.clone(),
                            verdict: Some(v),
                            message: o.result.message.clone(),
                            ..result(StressStop::BruteFailed, rounds, tested)
                        });
                    }
                }
            }
            if cancelled() {
                return Ok(result(StressStop::Cancelled, rounds, tested));
            }
            // 3. The solution, judged against the brute force's outputs.
            let tests: Vec<TestCase> = answered
                .iter()
                .enumerate()
                .map(|(i, (x, e))| case(i, x.clone(), e.clone()))
                .collect();
            let report = self
                .judge(p.clone(), tests, language, code, |_: JudgeEvent| {})
                .await?;
            if report.verdict == Verdict::CompilationError {
                return Ok(StressResult {
                    input: answered.first().map(|(x, _)| x.clone()),
                    verdict: Some(Verdict::CompilationError),
                    message: report.compile_output,
                    ..result(StressStop::Found, rounds, tested)
                });
            }
            let failed = report
                .tests
                .iter()
                .enumerate()
                .find(|(_, o)| o.result.verdict != Verdict::Accepted);
            match failed {
                Some((i, o)) => {
                    tested += u32::try_from(i + 1).unwrap_or(u32::MAX);
                    let (x, e) = answered[i].clone();
                    return Ok(StressResult {
                        input: Some(x),
                        expected: Some(e),
                        actual: o.actual.clone(),
                        verdict: Some(o.result.verdict),
                        message: o.result.message.clone(),
                        ..result(StressStop::Found, rounds, tested)
                    });
                }
                None => tested += u32::try_from(answered.len()).unwrap_or(u32::MAX),
            }
            on_event(StressEvent {
                rounds,
                max_rounds: MAX_ROUNDS,
                tested,
            });
        }
    }
}

fn limits() -> kb_judge::ScriptLimits {
    kb_judge::ScriptLimits {
        cpu_time_ms: 10_000,
        memory_mb: 512,
        max_output_bytes: 1024 * 1024,
    }
}
