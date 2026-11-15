//! "Find a smaller failing input" (ADR 0030). Starting from a failing input the user can
//! already see, it tries smaller inputs (kb_judge::shrink) round by round:
//! 1. the problem's validate.py keeps the valid ones;
//! 2. the reference solution gives their expected output (as for custom tests);
//! 3. the user's code runs on them, and the first one that still fails (the biggest cut)
//!    becomes the new input.
//!
//! Each step is one sandbox run. The shrink holds the judge slot, so it never overlaps a
//! Run or Submit; it stops at a run and time budget, when no candidate fails, or on Stop,
//! which is checked before every sandbox run.

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use kb_core::{JudgeEvent, Language, Message, ProblemMode, TestCase, TestKind, Verdict};
use kb_judge::shrink::{candidates, input_size};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{App, AppError, AppErrorCode, CustomInput, Result};

/// Sandbox runs one shrink may use.
pub const MAX_RUNS: u32 = 60;
/// Wall time one shrink may use (it finishes the run in progress).
pub const MAX_TIME: Duration = Duration::from_secs(30);
/// Candidates tried per round.
pub const BATCH: usize = 16;

/// Progress, after each round.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ShrinkEvent {
    pub runs: u32,
    pub max_runs: u32,
    pub original_size: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum ShrinkStop {
    /// No smaller candidate fails: this is as small as the moves get.
    Smallest,
    /// The run or time budget ran out.
    Budget,
    /// The user pressed Stop.
    Cancelled,
    /// The starting input does not fail (any more) with this code.
    NotFailing,
}

/// The smallest failing input found, with what the reference and the user's code gave.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ShrinkResult {
    pub input: CustomInput,
    pub expected: Value,
    pub actual: Option<Value>,
    pub verdict: Verdict,
    pub message: Option<Message>,
    pub original_size: u64,
    pub size: u64,
    pub runs: u32,
    pub stop: ShrinkStop,
}

/// What the user's code did on one input.
struct Failing {
    expected: Value,
    actual: Option<Value>,
    verdict: Verdict,
    message: Option<Message>,
}

struct Budget {
    runs: u32,
    started: Instant,
}

impl Budget {
    fn left(&self) -> bool {
        self.runs < MAX_RUNS && self.started.elapsed() < MAX_TIME
    }
}

fn case(index: usize, input: Map<String, Value>, expected: Value) -> TestCase {
    TestCase {
        kind: TestKind::Custom,
        index: u32::try_from(index + 1).unwrap_or(u32::MAX),
        input,
        expected,
    }
}

impl App {
    /// Stop a running shrink after its current sandbox run.
    pub fn cancel_shrink(&self) {
        self.shrink_cancel.store(true, Ordering::SeqCst);
    }

    /// Shrink `input`, on which `code` fails, to a smaller input that still fails.
    pub async fn shrink<F>(
        &self,
        problem: &str,
        language: Language,
        code: &str,
        input: CustomInput,
        mut on_event: F,
    ) -> Result<ShrinkResult>
    where
        F: FnMut(ShrinkEvent) + Send,
    {
        let _one = self.judge_slot()?;
        self.ensure_interview_language(problem, language).await?;
        self.ensure_calibrated().await;
        self.shrink_cancel.store(false, Ordering::SeqCst);
        let p = self.problem(problem).await?;
        if p.mode != ProblemMode::Function {
            return Err(AppError::new(
                AppErrorCode::ShrinkUnavailable,
                "only function problems can be shrunk",
            ));
        }
        kb_judge::check_input(&input, &p.signature)
            .map_err(|e| AppError::new(AppErrorCode::InvalidCustomInput, e))?;
        let reference = self
            .store
            .content
            .reference_solution(&p.id, language)
            .await?
            .ok_or_else(|| {
                AppError::new(
                    AppErrorCode::ShrinkUnavailable,
                    format!("{} has no {language} reference solution", p.id),
                )
            })?;
        let validator = if self.toolchains.runtime_installed(Language::Python) {
            self.store.content.validator(&p.id).await?
        } else {
            None
        };
        let original_size = input_size(&input);
        let mut budget = Budget {
            runs: 0,
            started: Instant::now(),
        };
        let cancelled = || self.shrink_cancel.load(Ordering::SeqCst);

        // The starting point: its expected output, and that it really fails.
        let Some(start) = self
            .first_failing(
                &p,
                language,
                code,
                &reference,
                vec![input.clone()],
                &mut budget,
            )
            .await?
        else {
            return Ok(ShrinkResult {
                input,
                expected: Value::Null,
                actual: None,
                verdict: Verdict::Accepted,
                message: None,
                original_size,
                size: original_size,
                runs: budget.runs,
                stop: ShrinkStop::NotFailing,
            });
        };
        let (mut current, mut failing) = (input, start.1);
        let mut stop = ShrinkStop::Smallest;
        on_event(ShrinkEvent {
            runs: budget.runs,
            max_runs: MAX_RUNS,
            original_size,
            size: original_size,
        });
        loop {
            if cancelled() {
                stop = ShrinkStop::Cancelled;
                break;
            }
            if !budget.left() {
                stop = ShrinkStop::Budget;
                break;
            }
            let mut cands: Vec<Map<String, Value>> = candidates(&current, &p.signature, BATCH)
                .into_iter()
                .filter(|c| kb_judge::check_input(c, &p.signature).is_ok())
                .collect();
            if let Some(v) = &validator
                && !cands.is_empty()
            {
                let env = self.env();
                let v = v.clone();
                let inputs = cands.clone();
                budget.runs += 1;
                let answers = tokio::task::spawn_blocking(move || {
                    let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
                    let limits = kb_judge::ScriptLimits {
                        cpu_time_ms: 10_000,
                        memory_mb: 512,
                        max_output_bytes: 1024 * 1024,
                    };
                    kb_judge::validate_inputs(&env, &v, &refs, limits)
                })
                .await??;
                cands = cands
                    .into_iter()
                    .zip(answers)
                    .filter_map(|(c, a)| a.is_none().then_some(c))
                    .collect();
            }
            if cands.is_empty() {
                break;
            }
            if cancelled() {
                stop = ShrinkStop::Cancelled;
                break;
            }
            match self
                .first_failing(&p, language, code, &reference, cands, &mut budget)
                .await?
            {
                Some((smaller, f)) => {
                    current = smaller;
                    failing = f;
                    on_event(ShrinkEvent {
                        runs: budget.runs,
                        max_runs: MAX_RUNS,
                        original_size,
                        size: input_size(&current),
                    });
                }
                None => break,
            }
        }
        Ok(ShrinkResult {
            size: input_size(&current),
            input: current,
            expected: failing.expected,
            actual: failing.actual,
            verdict: failing.verdict,
            message: failing.message,
            original_size,
            runs: budget.runs,
            stop,
        })
    }

    /// The reference's outputs for `inputs` (one run), then the user's code on those it
    /// could answer (one run): the first input the code fails, in order.
    async fn first_failing(
        &self,
        p: &kb_judge::Problem,
        language: Language,
        code: &str,
        reference: &str,
        inputs: Vec<Map<String, Value>>,
        budget: &mut Budget,
    ) -> Result<Option<(Map<String, Value>, Failing)>> {
        let tests: Vec<TestCase> = inputs
            .iter()
            .enumerate()
            .map(|(i, x)| case(i, x.clone(), Value::Null))
            .collect();
        budget.runs += 1;
        let report = self
            .judge(p.clone(), tests, language, reference, |_: JudgeEvent| {})
            .await?;
        if report.verdict == Verdict::CompilationError {
            return Err(AppError::new(
                AppErrorCode::JudgeInternal,
                format!("the {language} reference solution does not compile"),
            ));
        }
        // Only inputs the reference answered normally: their answers are the expected
        // outputs (as for custom tests).
        let answered: Vec<(Map<String, Value>, Value)> = inputs
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| {
                let o = report.tests.get(i)?;
                (matches!(o.result.verdict, Verdict::Accepted | Verdict::WrongAnswer))
                    .then(|| o.actual.clone())
                    .flatten()
                    .map(|e| (x, e))
            })
            .collect();
        if answered.is_empty() || !budget.left() {
            return Ok(None);
        }
        let tests: Vec<TestCase> = answered
            .iter()
            .enumerate()
            .map(|(i, (x, e))| case(i, x.clone(), e.clone()))
            .collect();
        budget.runs += 1;
        let report = self
            .judge(p.clone(), tests, language, code, |_: JudgeEvent| {})
            .await?;
        if report.verdict == Verdict::CompilationError {
            return Ok(None);
        }
        Ok(report
            .tests
            .iter()
            .enumerate()
            .find(|(_, o)| o.result.verdict != Verdict::Accepted)
            .and_then(|(i, o)| {
                let (x, e) = answered.get(i)?.clone();
                Some((
                    x,
                    Failing {
                        expected: e,
                        actual: o.actual.clone(),
                        verdict: o.result.verdict,
                        message: o.result.message.clone(),
                    },
                ))
            }))
    }
}
