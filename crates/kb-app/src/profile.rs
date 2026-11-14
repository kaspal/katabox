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
        Ok(ProfileView {
            estimate: fit(run.points.iter().map(|p| (p.n, p.median_us)).collect()),
            reference_estimate: fit(reference.iter().map(|r| (r.n, r.us)).collect()),
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
