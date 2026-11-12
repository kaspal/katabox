//! This machine's calibration (ADR 0017): measured once with kb-judge's benchmark, kept in
//! user.db under [`CALIBRATION_SETTING`], and applied to every judge run as the time scale.

use std::time::{SystemTime, UNIX_EPOCH};

use kb_core::{Language, LanguageLimit};
use kb_judge::calibration::{BENCHMARK_LANGUAGES, Calibration};
use kb_judge::{JudgeError, languages, limits};
use serde::{Deserialize, Serialize};

use crate::{App, AppError, AppErrorCode, Result};

/// The user.db setting holding the last [`CalibrationInfo`].
pub const CALIBRATION_SETTING: &str = "judge.calibration";

/// What Settings shows about the calibration.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CalibrationInfo {
    /// Measured time ÷ the reference machine's: above 1 is a slower machine.
    pub factor: f64,
    /// The time scale the judge applies: `factor` within 0.5-3.0.
    pub applied: f64,
    /// The runtime that ran the benchmark (Python, or TypeScript for Node).
    pub runtime: Language,
    /// Fastest benchmark round here and on the reference machine, in microseconds.
    pub measured_us: u32,
    pub reference_us: u32,
    /// When it was measured (Unix seconds).
    pub measured_at: i64,
}

impl CalibrationInfo {
    fn from_calibration(c: Calibration) -> Self {
        let us = |v: u64| u32::try_from(v).unwrap_or(u32::MAX);
        CalibrationInfo {
            factor: c.factor,
            applied: c.applied,
            runtime: c.runtime,
            measured_us: us(c.measured_us),
            reference_us: us(c.reference_us),
            measured_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX)),
        }
    }
}

/// The time scale for a stored calibration (1.0 when none), always within bounds.
pub(crate) fn scale_of(c: Option<&CalibrationInfo>) -> f64 {
    limits::clamp_scale(c.map_or(1.0, |c| c.applied))
}

/// Each judge language's limit for a problem's `base_ms` at `scale`.
pub(crate) fn time_limits(base_ms: u32, scale: f64) -> Vec<LanguageLimit> {
    languages::SUPPORTED
        .into_iter()
        .filter_map(|language| {
            let def = languages::definition(language).ok()?;
            Some(LanguageLimit {
                language,
                time_limit_ms: limits::effective_limit_ms(base_ms, &def, scale),
            })
        })
        .collect()
}

impl App {
    /// The stored calibration, if this machine was measured.
    pub fn calibration(&self) -> Option<CalibrationInfo> {
        *self.calibration.read().unwrap_or_else(|e| e.into_inner())
    }

    /// Measure this machine again (Settings → Recalibrate). Refused with `judge_busy`
    /// during a Run or Submit, and with `calibration_unavailable` when neither Python
    /// nor Node is installed.
    pub async fn recalibrate(&self) -> Result<CalibrationInfo> {
        let _one = self.judge_slot()?;
        self.measure().await
    }

    /// Measure once per machine: when nothing is stored yet and a benchmark runtime is
    /// installed. A failure leaves the scale at 1.0 and is retried next time. The caller
    /// holds the judge slot.
    pub(crate) async fn ensure_calibrated(&self) {
        if self.calibration().is_some() || !self.can_calibrate() {
            return;
        }
        // Not fatal: the judge falls back to the reference machine's limits.
        let _ = self.measure().await;
    }

    fn can_calibrate(&self) -> bool {
        BENCHMARK_LANGUAGES
            .iter()
            .any(|l| self.toolchains.runtime_installed(*l))
    }

    async fn measure(&self) -> Result<CalibrationInfo> {
        if !self.can_calibrate() {
            return Err(unavailable());
        }
        let env = self.env();
        let measured = tokio::task::spawn_blocking(move || kb_judge::calibration::calibrate(&env))
            .await?
            .map_err(|e| match e {
                JudgeError::Toolchain(_) => unavailable(),
                e => AppError::from(e),
            })?;
        let info = CalibrationInfo::from_calibration(measured);
        let value = serde_json::to_value(info)
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        self.store
            .user
            .set_setting(CALIBRATION_SETTING, &value)
            .await?;
        *self.calibration.write().unwrap_or_else(|e| e.into_inner()) = Some(info);
        Ok(info)
    }
}

fn unavailable() -> AppError {
    AppError::new(
        AppErrorCode::CalibrationUnavailable,
        "measuring this machine needs Python or JavaScript/TypeScript installed",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_follow_the_scale() {
        let at = |scale| {
            time_limits(300, scale)
                .into_iter()
                .map(|l| (l.language, l.time_limit_ms))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            at(1.0),
            [
                (Language::Python, 3000),
                (Language::TypeScript, 600),
                (Language::Php, 1500),
                (Language::Rust, 300)
            ]
        );
        assert_eq!(at(2.0)[3], (Language::Rust, 600));
        assert_eq!(at(9.0)[3], (Language::Rust, 900)); // clamped to 3.0
    }

    #[test]
    fn the_scale_comes_from_the_applied_factor() {
        assert_eq!(scale_of(None), 1.0);
        let c = CalibrationInfo {
            factor: 4.2,
            applied: 3.0,
            runtime: Language::Python,
            measured_us: 900_000,
            reference_us: 216_000,
            measured_at: 0,
        };
        assert_eq!(scale_of(Some(&c)), 3.0);
        let round_trip: CalibrationInfo =
            serde_json::from_value(serde_json::to_value(c).unwrap()).unwrap();
        assert_eq!(round_trip, c);
    }
}
