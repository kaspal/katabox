//! Focus sessions (expansion M10, ADR 0037): work and break periods on the wall clock.
//! The running session is the `focus.current` setting, so it survives a restart; Stop
//! writes a `focus_sessions` row with what happened during it. The phase rules are pure.

use kb_store::focus::{FocusSessionRow, FocusSummary};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{App, AppError, AppErrorCode, Result};

/// The running session's setting (stays on this machine).
pub const CURRENT: &str = "focus.current";
pub const WORK_MIN: std::ops::RangeInclusive<u32> = 5..=120;
pub const BREAK_MIN: std::ops::RangeInclusive<u32> = 1..=60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct FocusCurrent {
    /// UTC ms.
    pub started_at: i64,
    pub work_min: u32,
    pub break_min: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum FocusPhase {
    Work,
    Break,
}

/// Where a session is after `elapsed_ms`: the phase, the seconds left in it, and the work
/// periods completed so far.
pub fn phase_at(c: &FocusCurrent, elapsed_ms: i64) -> (FocusPhase, i64, u32) {
    let work = i64::from(c.work_min) * 60_000;
    let cycle = work + i64::from(c.break_min) * 60_000;
    let elapsed = elapsed_ms.max(0);
    let done = u32::try_from(elapsed / cycle).unwrap_or(u32::MAX);
    let into = elapsed % cycle;
    if into < work {
        (FocusPhase::Work, (work - into + 999) / 1000, done)
    } else {
        (
            FocusPhase::Break,
            (cycle - into + 999) / 1000,
            done.saturating_add(1),
        )
    }
}

impl App {
    pub async fn focus_current(&self) -> Result<Option<FocusCurrent>> {
        Ok(match self.store.user.get_setting(CURRENT).await? {
            Some(v) if !v.is_null() => serde_json::from_value(v).ok(),
            _ => None,
        })
    }

    /// Start a session now (a running one is stopped and kept first).
    pub async fn focus_start(&self, work_min: u32, break_min: u32) -> Result<FocusCurrent> {
        if !WORK_MIN.contains(&work_min) || !BREAK_MIN.contains(&break_min) {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "work is 5-120 minutes and breaks 1-60",
            ));
        }
        self.focus_stop().await?;
        let c = FocusCurrent {
            started_at: self.now_ms(),
            work_min,
            break_min,
        };
        self.store
            .user
            .set_setting(CURRENT, &serde_json::to_value(c).unwrap_or(Value::Null))
            .await?;
        Ok(c)
    }

    /// Stop the running session and keep it with its summary (`None`: nothing ran).
    pub async fn focus_stop(&self) -> Result<Option<FocusSessionRow>> {
        let Some(c) = self.focus_current().await? else {
            return Ok(None);
        };
        let now = self.now_ms().max(c.started_at);
        let (_, _, cycles) = phase_at(&c, now - c.started_at);
        let summary: FocusSummary = self.store.user.focus_summary(c.started_at, now).await?;
        let row = FocusSessionRow {
            started_at: c.started_at,
            ended_at: now,
            work_min: c.work_min,
            break_min: c.break_min,
            cycles,
            summary,
        };
        self.store.user.put_focus_session(&row).await?;
        self.store.user.set_setting(CURRENT, &Value::Null).await?;
        Ok(Some(row))
    }

    /// Finished sessions, newest first.
    pub async fn focus_sessions(&self) -> Result<Vec<FocusSessionRow>> {
        Ok(self.store.user.focus_sessions().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phases_follow_the_wall_clock() {
        let c = FocusCurrent {
            started_at: 0,
            work_min: 25,
            break_min: 5,
        };
        let min = 60_000;
        assert_eq!(phase_at(&c, 0), (FocusPhase::Work, 25 * 60, 0));
        assert_eq!(phase_at(&c, 24 * min + 59_500), (FocusPhase::Work, 1, 0));
        assert_eq!(phase_at(&c, 25 * min), (FocusPhase::Break, 5 * 60, 1));
        assert_eq!(phase_at(&c, 30 * min), (FocusPhase::Work, 25 * 60, 1));
        assert_eq!(phase_at(&c, 62 * min), (FocusPhase::Work, 23 * 60, 2));
        // A clock that went backwards counts as the start.
        assert_eq!(phase_at(&c, -5), (FocusPhase::Work, 25 * 60, 0));
    }
}
