//! The complexity profiler command (ADR 0024): run a solution on the problem's profile
//! inputs, fit a growth class and put the reference solution's line next to it.

use kb_core::Language;
use kb_core::profile::{Estimate, ProfilePoint, ProfileStop};
use serde::Serialize;

use crate::{App, AppError, AppErrorCode, Result};

/// The reference solution's time at one size, at this machine's time scale.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReferencePoint {
    pub n: u32,
    pub us: u32,
}

/// What a profile measured and what the fit estimates. Every class is an estimate: the
/// UI always says so.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProfileView {
    /// The user's solution, in increasing n (the sizes that ran in time).
    pub points: Vec<ProfilePoint>,
    /// The reference solution in the same language, at every size.
    pub reference: Vec<ReferencePoint>,
    /// `None` with fewer than three points.
    pub estimate: Option<Estimate>,
    /// The reference solution's class, fitted the same way.
    pub reference_estimate: Option<Estimate>,
    /// Why the profile stopped before the largest size, if it did.
    pub stopped: Option<ProfileStop>,
    /// The machine's time scale the reference times were multiplied by (ADR 0017).
    pub time_scale: f64,
    /// Time the profile waited for other sandbox users, in ms (not counted in its budget).
    pub queued_ms: u32,
    /// A program problem's process startup, taken off every point (microseconds).
    pub startup_us: u32,
    /// A program problem whose estimate would not be trustworthy (startup dominates, or
    /// too few sizes): shown without an estimate, saying why.
    pub unreliable: bool,
    /// An unreliable program profile where every size ran, yet the work stayed below the
    /// startup at all of them: the program is too fast to classify here (a slow class,
    /// such as O(n²), shows clear growth at these sizes).
    pub fast: bool,
    /// The same for the reference in this language (judged against this run's startup):
    /// its line is drawn, but no class is fitted to it.
    pub reference_fast: bool,
}

pub use kb_judge::profile::{
    PROGRAM_MIN_POINTS, PROGRAM_STARTUP_SHARE, program_fit_points, program_unreliable,
};

/// Whether an unreliable program profile is "fast" rather than cut short: every size
/// ran (no stop), with at least the minimum number of points.
pub(crate) fn program_fast(points: &[ProfilePoint], sizes: usize, stopped: bool) -> bool {
    !stopped && points.len() >= PROGRAM_MIN_POINTS && points.len() == sizes
}

/// A program reference's line as profile points, to judge it by the same rule.
fn reference_points(reference: &[ReferencePoint]) -> Vec<ProfilePoint> {
    reference
        .iter()
        .map(|r| ProfilePoint {
            n: r.n,
            median_us: r.us,
            samples_us: vec![r.us],
        })
        .collect()
}

/// The reference times (factor 1.0) at the machine's time scale.
pub(crate) fn scale_reference(sizes: &[u32], times: &[u32], scale: f64) -> Vec<ReferencePoint> {
    sizes
        .iter()
        .zip(times)
        .map(|(&n, &us)| ReferencePoint {
            n,
            us: (f64::from(us) * scale)
                .round()
                .clamp(0.0, f64::from(u32::MAX)) as u32,
        })
        .collect()
}

impl App {
    /// Profile `code` on `problem`'s profile inputs in the sandbox (at most about 12 s).
    /// Takes the judge slot like Run and Submit. Allowed in mock interviews: it reveals
    /// nothing locked.
    pub async fn profile_solution(
        &self,
        problem: &str,
        language: Language,
        code: &str,
    ) -> Result<ProfileView> {
        let _one = self.judge_slot()?;
        self.ensure_interview_language(problem, language).await?;
        self.ensure_calibrated().await;
        let data = self
            .store
            .content
            .profile_data(problem)
            .await?
            .ok_or_else(|| {
                AppError::new(
                    AppErrorCode::ProfileUnavailable,
                    format!("{problem} has no complexity profile"),
                )
            })?;
        let p = self.problem(problem).await?;
        let env = self.env();
        let scale = env.time_scale;
        let sizes: Vec<u32> = data.inputs.iter().map(|(n, _)| *n).collect();
        let reference = data
            .reference
            .iter()
            .find(|(l, _)| *l == language)
            .map(|(_, t)| scale_reference(&sizes, t, scale))
            .unwrap_or_default();
        let code = code.to_string();
        let inputs = data.inputs;
        let is_program = p.mode == kb_core::ProblemMode::Program;
        let run = tokio::task::spawn_blocking(move || {
            kb_judge::profile::profile(
                &p,
                language,
                &code,
                &inputs,
                &env,
                kb_judge::profile::BUDGET,
            )
        })
        .await??;
        let fit = |pts: Vec<(u32, u32)>| kb_judge::complexity::estimate(&pts);
        let unreliable = is_program && program_unreliable(&run.points, run.startup_us);
        let fast = unreliable && program_fast(&run.points, sizes.len(), run.stopped.is_some());
        let reference_fast = is_program
            && !reference.is_empty()
            && program_unreliable(&reference_points(&reference), run.startup_us);
        Ok(ProfileView {
            estimate: if unreliable {
                None
            } else if is_program {
                fit(program_fit_points(&run.points, run.startup_us))
            } else {
                fit(run.points.iter().map(|p| (p.n, p.median_us)).collect())
            },
            startup_us: run.startup_us,
            unreliable,
            fast,
            reference_fast,
            reference_estimate: if reference_fast {
                None
            } else if is_program {
                // Its times were recorded without startup; this run's startup stands in.
                fit(program_fit_points(
                    &reference_points(&reference),
                    run.startup_us,
                ))
            } else {
                fit(reference.iter().map(|r| (r.n, r.us)).collect())
            },
            points: run.points,
            reference,
            stopped: run.stopped,
            time_scale: scale,
            queued_ms: run.queued_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pts(us: &[u32]) -> Vec<ProfilePoint> {
        us.iter()
            .enumerate()
            .map(|(k, u)| ProfilePoint {
                n: 100 * (k as u32 + 1),
                median_us: *u,
                samples_us: vec![*u],
            })
            .collect()
    }

    #[test]
    fn a_program_profile_is_unreliable_when_startup_dominates_or_points_are_few() {
        // Work grows to 400 ms; 40 ms of startup is fine.
        assert!(!program_unreliable(
            &pts(&[10_000, 40_000, 160_000, 400_000]),
            40_000
        ));
        // Startup is most of the largest point's time.
        assert!(program_unreliable(&pts(&[100, 200, 400, 800]), 40_000));
        // Too few sizes.
        assert!(program_unreliable(&pts(&[10_000, 40_000]), 1_000));
        assert!(!program_unreliable(&pts(&[10_000, 40_000, 160_000]), 1_000));
        assert!(program_unreliable(&[], 0));
    }

    #[test]
    fn a_fast_program_ran_every_size_and_a_cut_short_one_is_not_fast() {
        let p = pts(&[100, 200, 400, 800]);
        assert!(program_fast(&p, 4, false));
        // A size stopped it (time limit, budget): not "fast".
        assert!(!program_fast(&p, 4, true));
        assert!(!program_fast(&p, 6, false));
        // Too few sizes to say anything.
        assert!(!program_fast(&p[..2], 2, false));
        // The reference's line is judged the same way.
        let r = reference_points(&scale_reference(&[1, 2, 3], &[50, 60, 70], 1.0));
        assert!(program_unreliable(&r, 40_000));
        assert_eq!(r[2].median_us, 70);
    }

    #[test]
    fn reference_times_follow_the_machine_scale() {
        let r = scale_reference(&[10, 100], &[40, 400], 1.5);
        assert_eq!(
            r,
            [
                ReferencePoint { n: 10, us: 60 },
                ReferencePoint { n: 100, us: 600 }
            ]
        );
        assert_eq!(scale_reference(&[10], &[7], 0.5)[0].us, 4);
    }
}
