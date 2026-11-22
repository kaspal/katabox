//! The startup splash (ADR 0041): whether to show it and how long it may hold the main
//! window back. Pure rules; the desktop shell (`src-tauri/src/splash.rs`) reads the two
//! settings, applies the plan and records the version.
//!
//! - `app.splash` (machine-only, not exported): `"firstRunAfterUpdate"` (the default),
//!   `"always"` or `"never"`.
//! - `app.splash_version`: the app version of the last start, written by the shell.
//!
//! The splash never delays start-up beyond its animation, and only waits for the whole
//! animation on the first start after an install or update, or in `always` mode;
//! otherwise it closes as soon as the main window is ready.

use serde_json::Value;

/// The setting that picks the mode.
pub const SPLASH_MODE_KEY: &str = "app.splash";
/// The version of the last start, to tell an install or update from an ordinary start.
pub const SPLASH_VERSION_KEY: &str = "app.splash_version";
/// The length of the full animation (the bar ends at 0.3 s + 2.6 s; "Ready" at 2.9 s).
pub const ANIMATION_MS: u64 = 2_600;
/// Without `app_ready` by then, the main window is shown anyway.
pub const FAILSAFE_MS: u64 = 15_000;
/// The fade-out when the splash closes.
pub const FADE_MS: u64 = 180;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplashMode {
    FirstRunAfterUpdate,
    Always,
    Never,
}

/// What the shell does at start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplashPlan {
    /// Create the splash window at all.
    pub show: bool,
    /// Keep it until the animation has played out, even when the app is ready sooner.
    pub wait_full: bool,
}

/// The stored mode; anything unknown (or missing) is the default.
pub fn parse_mode(value: Option<&Value>) -> SplashMode {
    match value.and_then(Value::as_str) {
        Some("always") => SplashMode::Always,
        Some("never") => SplashMode::Never,
        _ => SplashMode::FirstRunAfterUpdate,
    }
}

/// The plan for this start. `last` is the stored version of the previous start (none on
/// a fresh install), `current` this build's version.
pub fn splash_plan(mode: SplashMode, last: Option<&str>, current: &str) -> SplashPlan {
    match mode {
        SplashMode::Never => SplashPlan {
            show: false,
            wait_full: false,
        },
        SplashMode::Always => SplashPlan {
            show: true,
            wait_full: true,
        },
        SplashMode::FirstRunAfterUpdate => SplashPlan {
            show: true,
            wait_full: last != Some(current),
        },
    }
}

/// How long to wait before handing over, `elapsed_ms` after the splash appeared.
pub fn remaining_ms(plan: SplashPlan, elapsed_ms: u64) -> u64 {
    if plan.show && plan.wait_full {
        ANIMATION_MS.saturating_sub(elapsed_ms)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_the_mode_with_the_default_for_anything_else() {
        assert_eq!(parse_mode(None), SplashMode::FirstRunAfterUpdate);
        assert_eq!(parse_mode(Some(&json!("always"))), SplashMode::Always);
        assert_eq!(parse_mode(Some(&json!("never"))), SplashMode::Never);
        assert_eq!(
            parse_mode(Some(&json!("firstRunAfterUpdate"))),
            SplashMode::FirstRunAfterUpdate
        );
        assert_eq!(
            parse_mode(Some(&json!("ALWAYS"))),
            SplashMode::FirstRunAfterUpdate
        );
        assert_eq!(
            parse_mode(Some(&json!(true))),
            SplashMode::FirstRunAfterUpdate
        );
        assert_eq!(
            parse_mode(Some(&Value::Null)),
            SplashMode::FirstRunAfterUpdate
        );
    }

    #[test]
    fn plays_in_full_on_a_new_install_or_an_update_only() {
        let m = SplashMode::FirstRunAfterUpdate;
        let full = SplashPlan {
            show: true,
            wait_full: true,
        };
        let quick = SplashPlan {
            show: true,
            wait_full: false,
        };
        assert_eq!(splash_plan(m, None, "1.0.0-beta.1"), full, "fresh install");
        assert_eq!(
            splash_plan(m, Some("1.0.0-beta.0"), "1.0.0-beta.1"),
            full,
            "update"
        );
        assert_eq!(
            splash_plan(m, Some("1.0.1"), "1.0.0"),
            full,
            "downgrade counts as a change"
        );
        assert_eq!(
            splash_plan(m, Some("1.0.0-beta.1"), "1.0.0-beta.1"),
            quick,
            "same version"
        );
    }

    #[test]
    fn always_and_never() {
        for last in [None, Some("1.0.0")] {
            assert_eq!(
                splash_plan(SplashMode::Always, last, "1.0.0"),
                SplashPlan {
                    show: true,
                    wait_full: true
                }
            );
            assert_eq!(
                splash_plan(SplashMode::Never, last, "1.0.0"),
                SplashPlan {
                    show: false,
                    wait_full: false
                }
            );
        }
    }

    #[test]
    fn never_waits_past_the_animation() {
        let full = SplashPlan {
            show: true,
            wait_full: true,
        };
        assert_eq!(remaining_ms(full, 0), ANIMATION_MS);
        assert_eq!(remaining_ms(full, 1_000), ANIMATION_MS - 1_000);
        assert_eq!(remaining_ms(full, 9_000), 0);
        assert_eq!(
            remaining_ms(
                SplashPlan {
                    show: true,
                    wait_full: false
                },
                0
            ),
            0
        );
        assert_eq!(
            remaining_ms(
                SplashPlan {
                    show: false,
                    wait_full: false
                },
                0
            ),
            0
        );
        const { assert!(FAILSAFE_MS > ANIMATION_MS) };
    }
}
