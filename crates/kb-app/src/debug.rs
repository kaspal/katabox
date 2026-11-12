//! The debugger (ADR 0022): one session at a time, navigating a recorded trace.
//!
//! A session starts with a trace of one test (kb-judge replays it in the sandbox). The
//! debugger's position is a step of that trace; continue and the step commands are pure
//! functions over it ([`next_breakpoint`], [`step_over`], …). Each new position is shown
//! by replaying to it ([`kb_judge::debug::stop`]), which also evaluates the watches and
//! expands the variables the UI has open.

use std::collections::BTreeSet;

use kb_core::Language;
use kb_core::debug::{DebugOutcome, EvalResult, PrintChunk, StopInfo, Trace, Truncation};
use kb_judge::debug::{DebugLimit, DebugRun, StopRequest};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{App, AppError, AppErrorCode, Result};

/// A debugger command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum DebugAction {
    Continue,
    StepOver,
    StepInto,
    StepOut,
    StepBack,
    Restart,
}

/// What the debugger shows.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum DebugState {
    /// Stopped at a step: the line about to run and everything around it.
    Paused { line: u32, stop: StopInfo },
    /// The program ran to its end (or to the end of a truncated trace).
    Finished {
        outcome: Option<DebugOutcome>,
        truncated: Option<Truncation>,
    },
    /// The session could not continue: `load_error`, `time_limit`, `memory_limit`,
    /// `output_limit` or `diverged`.
    Failed {
        reason: String,
        message: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DebugView {
    pub session: u32,
    pub state: DebugState,
    /// Steps recorded in the trace.
    pub steps: u32,
    /// The program's prints up to the current step.
    pub prints: Vec<PrintChunk>,
    pub prints_truncated: bool,
}

/// What a new debug session is about.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DebugStart {
    pub problem: String,
    pub language: Language,
    pub code: String,
    /// The test's arguments, keyed by parameter name (public, custom or the first
    /// failing test the user already sees).
    pub input: Map<String, Value>,
    pub breakpoints: Vec<u32>,
    pub watches: Vec<String>,
}

/// The UI's view state, replayed at every stop.
#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DebugInspect {
    /// The selected stack frame (0 = innermost).
    pub frame: u32,
    /// Expanded variables: `[scope, name, i, j, …]`.
    pub expand: Vec<Vec<Value>>,
    pub watches: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct Session {
    id: u32,
    problem: String,
    language: Language,
    code: String,
    input: Map<String, Value>,
    trace: Trace,
    /// The current step, or `None` when the program has finished.
    at: Option<usize>,
    breakpoints: BTreeSet<u32>,
    inspect: DebugInspect,
}

/// The judge languages the debugger supports (Python today).
pub fn languages() -> Vec<Language> {
    kb_judge::languages::SUPPORTED
        .into_iter()
        .filter(|l| kb_judge::debug::supported(*l))
        .collect()
}

// --- Navigation over the trace (pure) -------------------------------------------------

/// The first step after `from` (or from the start when `None`) on a breakpoint line.
pub fn next_breakpoint(trace: &Trace, from: Option<usize>, lines: &BTreeSet<u32>) -> Option<usize> {
    let start = from.map_or(0, |f| f + 1);
    (start..trace.steps.len()).find(|&i| lines.contains(&trace.steps[i].line))
}

/// The next step in this frame or an outer one (calls are run through).
pub fn step_over(trace: &Trace, at: usize) -> Option<usize> {
    let depth = trace.steps[at].depth;
    (at + 1..trace.steps.len()).find(|&i| trace.steps[i].depth <= depth)
}

/// The next step, wherever it is (into a call).
pub fn step_into(trace: &Trace, at: usize) -> Option<usize> {
    (at + 1 < trace.steps.len()).then_some(at + 1)
}

/// The next step in an outer frame (after this function returns).
pub fn step_out(trace: &Trace, at: usize) -> Option<usize> {
    let depth = trace.steps[at].depth;
    (at + 1..trace.steps.len()).find(|&i| trace.steps[i].depth < depth)
}

/// The previous step (replay makes stepping back free).
pub fn step_back(at: usize) -> Option<usize> {
    at.checked_sub(1)
}

/// Where `action` takes the debugger from `at` (`None` = the program's end). Step back
/// from the end goes to the last step.
pub fn navigate(
    trace: &Trace,
    at: Option<usize>,
    action: DebugAction,
    breakpoints: &BTreeSet<u32>,
) -> Option<usize> {
    match (action, at) {
        (DebugAction::Restart, _) => next_breakpoint(trace, None, breakpoints),
        (DebugAction::StepBack, None) => trace.steps.len().checked_sub(1),
        (_, None) => None,
        (DebugAction::Continue, Some(a)) => next_breakpoint(trace, Some(a), breakpoints),
        (DebugAction::StepOver, Some(a)) => step_over(trace, a),
        (DebugAction::StepInto, Some(a)) => step_into(trace, a),
        (DebugAction::StepOut, Some(a)) => step_out(trace, a),
        (DebugAction::StepBack, Some(a)) => step_back(a).or(Some(a)),
    }
}

fn failed(reason: &str, message: Option<String>) -> DebugState {
    DebugState::Failed {
        reason: reason.into(),
        message,
    }
}

fn limit_reason(l: DebugLimit) -> &'static str {
    match l {
        DebugLimit::Time => "time_limit",
        DebugLimit::Memory => "memory_limit",
        DebugLimit::Output => "output_limit",
    }
}

fn no_session() -> AppError {
    AppError::new(AppErrorCode::NotFound, "no debug session is running")
}

// --- App ------------------------------------------------------------------------------

impl App {
    /// Record one test and stop at its first breakpoint (or show the result).
    pub async fn debug_start(&self, start: DebugStart) -> Result<DebugView> {
        if !kb_judge::debug::supported(start.language) {
            return Err(AppError::new(
                AppErrorCode::DebugUnsupported,
                format!("{} cannot be debugged yet", start.language),
            ));
        }
        let _one = self.judge_slot()?;
        // Debugging is allowed in a mock interview: it is the user's own code. The
        // session's languages still apply, as for Run.
        self.ensure_interview_language(&start.problem, start.language)
            .await?;
        let problem = self.problem(&start.problem).await?;
        kb_judge::check_input(&start.input, &problem.signature)
            .map_err(|e| AppError::new(AppErrorCode::InvalidInput, e))?;
        let env = self.env();
        let (code, input, language) = (start.code.clone(), start.input.clone(), start.language);
        let p = problem.clone();
        let run = tokio::task::spawn_blocking(move || {
            kb_judge::debug::trace(&p, language, &code, &input, &env)
        })
        .await??;
        let id = {
            let mut guard = self.debug.lock().unwrap_or_else(|e| e.into_inner());
            let id = guard.as_ref().map_or(1, |s| s.id.wrapping_add(1));
            *guard = None;
            id
        };
        let trace = match run {
            DebugRun::Trace(t) => t,
            other => return Ok(self.failed_view(id, &other)),
        };
        let breakpoints: BTreeSet<u32> = start.breakpoints.into_iter().collect();
        let at = next_breakpoint(&trace, None, &breakpoints);
        let session = Session {
            id,
            problem: start.problem,
            language: start.language,
            code: start.code,
            input: start.input,
            trace,
            at,
            breakpoints,
            inspect: DebugInspect {
                watches: start.watches,
                ..Default::default()
            },
        };
        self.show(session).await
    }

    /// Continue, step or restart.
    pub async fn debug_action(&self, session: u32, action: DebugAction) -> Result<DebugView> {
        let _one = self.judge_slot()?;
        let mut s = self.take_session(session)?;
        s.at = navigate(&s.trace, s.at, action, &s.breakpoints);
        // A new position starts at the innermost frame.
        s.inspect.frame = 0;
        self.show(s).await
    }

    /// Replace the breakpoints (line numbers of the user's file).
    pub fn debug_set_breakpoints(&self, session: u32, lines: Vec<u32>) -> Result<()> {
        let mut guard = self.debug.lock().unwrap_or_else(|e| e.into_inner());
        let s = guard
            .as_mut()
            .filter(|s| s.id == session)
            .ok_or_else(no_session)?;
        s.breakpoints = lines.into_iter().collect();
        Ok(())
    }

    /// Show the current step again with another frame, expansions or watches.
    pub async fn debug_inspect(&self, session: u32, inspect: DebugInspect) -> Result<DebugView> {
        let _one = self.judge_slot()?;
        let mut s = self.take_session(session)?;
        s.inspect = inspect;
        self.show(s).await
    }

    /// Evaluate one console expression in the selected frame at the current step. The
    /// program is replayed, so the expression never changes later steps.
    pub async fn debug_evaluate(
        &self,
        session: u32,
        frame: u32,
        expr: String,
    ) -> Result<EvalResult> {
        let _one = self.judge_slot()?;
        let s = self.take_session(session)?;
        let Some(at) = s.at else {
            self.put_session(s);
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "the program has finished: nothing to evaluate in",
            ));
        };
        let req = StopRequest {
            step: u32::try_from(at).unwrap_or(u32::MAX),
            frame,
            expand: vec![],
            evals: vec![expr.clone()],
        };
        let run = self.replay(&s, req).await;
        self.put_session(s);
        match run? {
            DebugRun::Stop(mut stop) if !stop.evals.is_empty() => Ok(stop.evals.remove(0)),
            DebugRun::Diverged => Err(AppError::new(
                AppErrorCode::DebugDiverged,
                "this program behaves differently on each run",
            )),
            _ => Ok(EvalResult {
                expr,
                ok: false,
                value: None,
                error: Some("the evaluation did not finish".into()),
                stdout: String::new(),
            }),
        }
    }

    /// End the session (nothing is running between actions, so this only forgets it).
    pub fn debug_end(&self, session: u32) {
        let mut guard = self.debug.lock().unwrap_or_else(|e| e.into_inner());
        if guard.as_ref().is_some_and(|s| s.id == session) {
            *guard = None;
        }
    }

    fn take_session(&self, session: u32) -> Result<Session> {
        let mut guard = self.debug.lock().unwrap_or_else(|e| e.into_inner());
        match guard.take() {
            Some(s) if s.id == session => Ok(s),
            other => {
                *guard = other;
                Err(no_session())
            }
        }
    }

    fn put_session(&self, s: Session) {
        *self.debug.lock().unwrap_or_else(|e| e.into_inner()) = Some(s);
    }

    async fn replay(&self, s: &Session, req: StopRequest) -> Result<DebugRun> {
        let problem = self.problem(&s.problem).await?;
        let env = self.env();
        let (code, input, trace, language) =
            (s.code.clone(), s.input.clone(), s.trace.clone(), s.language);
        Ok(tokio::task::spawn_blocking(move || {
            kb_judge::debug::stop(&problem, language, &code, &input, &trace, &req, &env)
        })
        .await??)
    }

    /// Replay to the session's position (if paused) and keep the session.
    async fn show(&self, s: Session) -> Result<DebugView> {
        let state = match s.at {
            None => DebugState::Finished {
                outcome: s.trace.outcome.clone(),
                truncated: s.trace.truncated,
            },
            Some(at) => {
                let req = StopRequest {
                    step: u32::try_from(at).unwrap_or(u32::MAX),
                    frame: s.inspect.frame,
                    expand: s.inspect.expand.clone(),
                    evals: s.inspect.watches.clone(),
                };
                match self.replay(&s, req).await {
                    Ok(DebugRun::Stop(stop)) => DebugState::Paused {
                        line: s.trace.steps[at].line,
                        stop,
                    },
                    Ok(other) => {
                        // Diverged or a limit: the session cannot go on.
                        return Ok(self.failed_view(s.id, &other));
                    }
                    Err(e) => {
                        self.put_session(s);
                        return Err(e);
                    }
                }
            }
        };
        let upto =
            s.at.map_or(u32::MAX, |a| u32::try_from(a).unwrap_or(u32::MAX));
        let view = DebugView {
            session: s.id,
            state,
            steps: u32::try_from(s.trace.steps.len()).unwrap_or(u32::MAX),
            prints: s
                .trace
                .prints
                .iter()
                .filter(|p| p.step < upto)
                .cloned()
                .collect(),
            prints_truncated: s.trace.prints_truncated && s.at.is_none(),
        };
        self.put_session(s);
        Ok(view)
    }

    fn failed_view(&self, id: u32, run: &DebugRun) -> DebugView {
        *self.debug.lock().unwrap_or_else(|e| e.into_inner()) = None;
        let state = match run {
            DebugRun::LoadError(m) => failed("load_error", Some(m.clone())),
            DebugRun::Limit(l) => failed(limit_reason(*l), None),
            DebugRun::Diverged | DebugRun::Trace(_) | DebugRun::Stop(_) => failed("diverged", None),
        };
        DebugView {
            session: id,
            state,
            steps: 0,
            prints: vec![],
            prints_truncated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::debug::Step;

    /// A trace from (line, depth) pairs.
    fn trace(steps: &[(u32, u32)]) -> Trace {
        Trace {
            steps: steps
                .iter()
                .map(|&(line, depth)| Step { line, depth })
                .collect(),
            truncated: None,
            prints: vec![],
            prints_truncated: false,
            outcome: None,
        }
    }

    fn bp(lines: &[u32]) -> BTreeSet<u32> {
        lines.iter().copied().collect()
    }

    // main: line 2 calls helper (lines 10, 11), line 3, line 4 (in a loop: 3, 4, 3, 4).
    fn sample() -> Trace {
        trace(&[
            (2, 1),
            (10, 2),
            (11, 2),
            (3, 1),
            (4, 1),
            (3, 1),
            (4, 1),
            (5, 1),
        ])
    }

    #[test]
    fn continue_goes_to_the_next_breakpoint_or_the_end() {
        let t = sample();
        assert_eq!(navigate(&t, None, DebugAction::Restart, &bp(&[4])), Some(4));
        assert_eq!(
            navigate(&t, Some(4), DebugAction::Continue, &bp(&[4])),
            Some(6)
        );
        assert_eq!(
            navigate(&t, Some(6), DebugAction::Continue, &bp(&[4])),
            None
        );
        assert_eq!(navigate(&t, None, DebugAction::Restart, &bp(&[])), None);
        assert_eq!(
            navigate(&t, None, DebugAction::Restart, &bp(&[11])),
            Some(2)
        );
    }

    #[test]
    fn step_over_into_and_out() {
        let t = sample();
        // Over the call on line 2: straight to line 3.
        assert_eq!(
            navigate(&t, Some(0), DebugAction::StepOver, &bp(&[])),
            Some(3)
        );
        // Into it: line 10, one frame deeper.
        assert_eq!(
            navigate(&t, Some(0), DebugAction::StepInto, &bp(&[])),
            Some(1)
        );
        // Out of the helper: back at line 3 in main.
        assert_eq!(
            navigate(&t, Some(1), DebugAction::StepOut, &bp(&[])),
            Some(3)
        );
        // Out of main: the program ends.
        assert_eq!(navigate(&t, Some(4), DebugAction::StepOut, &bp(&[])), None);
        // The last step steps into the end.
        assert_eq!(navigate(&t, Some(7), DebugAction::StepInto, &bp(&[])), None);
        assert_eq!(navigate(&t, Some(7), DebugAction::StepOver, &bp(&[])), None);
    }

    #[test]
    fn step_back_and_finished_positions() {
        let t = sample();
        assert_eq!(
            navigate(&t, Some(3), DebugAction::StepBack, &bp(&[])),
            Some(2)
        );
        assert_eq!(
            navigate(&t, Some(0), DebugAction::StepBack, &bp(&[])),
            Some(0)
        );
        assert_eq!(navigate(&t, None, DebugAction::StepBack, &bp(&[])), Some(7));
        assert_eq!(navigate(&t, None, DebugAction::Continue, &bp(&[4])), None);
        assert_eq!(navigate(&t, None, DebugAction::Restart, &bp(&[4])), Some(4));
    }

    #[test]
    fn recursion_steps_over_the_whole_recursive_call() {
        // f(2) -> f(1) -> f(0): line 2 is the check, line 3 the recursive call.
        let t = trace(&[(2, 1), (3, 1), (2, 2), (3, 2), (2, 3), (3, 1)]);
        assert_eq!(step_over(&t, 1), Some(5));
        assert_eq!(step_into(&t, 1), Some(2));
        assert_eq!(step_out(&t, 3), Some(5));
    }
}
