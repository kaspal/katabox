//! Window memory (ui-polish): where the main window reopens.
//!
//! The desktop shell saves the window's bounds in the `window.bounds` setting and restores
//! them before showing the window. Everything that decides *what* to save and *where* to
//! put the window is here, pure and unit tested; `src-tauri` only reads the window and the
//! monitors and applies the result.
//!
//! **Units:** positions and sizes are **physical pixels** (what Tauri's `outer_position`,
//! `inner_size` and monitor work areas report), together with the scale factor of the
//! monitor the window was on. When the window comes back on a monitor with another scale
//! factor, its size is rescaled so it keeps the same logical size. The minimum and default
//! sizes are logical (as in `tauri.conf.json`) and are converted with the target
//! monitor's scale.
//!
//! The bounds are per machine: the setting is never exported (see `transfer.rs`).

use serde::{Deserialize, Serialize};

/// The setting holding the saved [`WindowBounds`] (JSON). `null` or anything unreadable
/// means "nothing saved".
pub const WINDOW_BOUNDS_KEY: &str = "window.bounds";

/// The share of the window's area that must lie on one monitor for the saved position to
/// be used as is (otherwise the window goes to the primary monitor, centred).
const MIN_VISIBLE_SHARE: f64 = 0.5;

/// What is saved: the last normal (not maximised, not minimised) bounds, and whether the
/// window was maximised on top of them.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowBounds {
    /// Outer position, physical pixels (may be negative on a monitor left of or above the
    /// primary one).
    pub x: i32,
    pub y: i32,
    /// Inner size, physical pixels.
    pub width: u32,
    pub height: u32,
    /// The scale factor of the monitor the window was on when saved.
    pub scale: f64,
    pub maximized: bool,
}

/// A monitor's work area (without the taskbar), physical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Monitor {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub primary: bool,
}

/// What the window looks like right now, read by the shell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowNow {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub maximized: bool,
    pub minimized: bool,
}

/// Where to put the window on start: physical position and inner size, then maximise or
/// not.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

/// Logical sizes from the window config.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeRules {
    pub default: (u32, u32),
    pub min: (u32, u32),
}

/// Read the setting's value (`None` for nothing saved, `null` or a value that doesn't fit).
pub fn parse_bounds(value: Option<serde_json::Value>) -> Option<WindowBounds> {
    let b: WindowBounds = serde_json::from_value(value?).ok()?;
    (b.width > 0 && b.height > 0 && b.scale.is_finite() && b.scale > 0.0).then_some(b)
}

/// The bounds to save after the window changed, or `None` to keep what is saved.
///
/// - Minimised: never saved (Windows reports the window at -32000, -32000).
/// - Maximised: the window's size is the monitor's, so the last normal bounds are kept and
///   only the flag changes. With nothing saved yet, nothing is saved.
/// - Otherwise: the current bounds, not maximised.
pub fn next_bounds(saved: Option<WindowBounds>, now: WindowNow) -> Option<WindowBounds> {
    if now.minimized
        || now.width == 0
        || now.height == 0
        || !now.scale.is_finite()
        || now.scale <= 0.0
    {
        return None;
    }
    if now.maximized {
        let prev = saved?;
        return (!prev.maximized).then_some(WindowBounds {
            maximized: true,
            ..prev
        });
    }
    let next = WindowBounds {
        x: now.x,
        y: now.y,
        width: now.width,
        height: now.height,
        scale: now.scale,
        maximized: false,
    };
    (saved != Some(next)).then_some(next)
}

/// Where the window opens: the saved bounds when they are mostly visible on a current
/// monitor (moved inside it if they stick out), otherwise the primary monitor, centred,
/// with the size clamped to fit. Never smaller than the minimum size, never larger than
/// the monitor. `None` only when there is no monitor at all (leave the window as created).
pub fn placement(
    saved: Option<WindowBounds>,
    monitors: &[Monitor],
    rules: SizeRules,
) -> Option<Placement> {
    let primary = monitors
        .iter()
        .find(|m| m.primary)
        .or_else(|| monitors.first())?;
    if let Some(b) = saved
        && let Some(m) = best_monitor(&b, monitors)
    {
        let (width, height) = fit_size(rescale(&b, m), m, rules);
        let x = clamp_axis(b.x, width, m.x, m.width);
        let y = clamp_axis(b.y, height, m.y, m.height);
        return Some(Placement {
            x,
            y,
            width,
            height,
            maximized: b.maximized,
        });
    }
    let wanted = match saved {
        Some(b) => rescale(&b, primary),
        None => logical(rules.default, primary.scale),
    };
    let (width, height) = fit_size(wanted, primary, rules);
    Some(Placement {
        x: centre(primary.x, primary.width, width),
        y: centre(primary.y, primary.height, height),
        width,
        height,
        maximized: saved.is_some_and(|b| b.maximized),
    })
}

/// The default size, centred on the monitor (Settings → "Reset window size and position").
pub fn reset_placement(monitor: &Monitor, rules: SizeRules) -> Placement {
    let (width, height) = fit_size(logical(rules.default, monitor.scale), monitor, rules);
    Placement {
        x: centre(monitor.x, monitor.width, width),
        y: centre(monitor.y, monitor.height, height),
        width,
        height,
        maximized: false,
    }
}

/// The monitor holding the largest part of the window, if that part is at least
/// [`MIN_VISIBLE_SHARE`] of the window.
fn best_monitor<'a>(b: &WindowBounds, monitors: &'a [Monitor]) -> Option<&'a Monitor> {
    let area = f64::from(b.width) * f64::from(b.height);
    monitors
        .iter()
        .map(|m| (m, overlap(b, m)))
        .filter(|&(_, o)| o > 0.0 && o >= area * MIN_VISIBLE_SHARE)
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(m, _)| m)
}

fn overlap(b: &WindowBounds, m: &Monitor) -> f64 {
    let span = |a0: i32, alen: u32, b0: i32, blen: u32| {
        let (a0, a1) = (i64::from(a0), i64::from(a0) + i64::from(alen));
        let (b0, b1) = (i64::from(b0), i64::from(b0) + i64::from(blen));
        (a1.min(b1) - a0.max(b0)).max(0) as f64
    };
    span(b.x, b.width, m.x, m.width) * span(b.y, b.height, m.y, m.height)
}

/// The saved size for a monitor with another scale factor (same logical size).
fn rescale(b: &WindowBounds, m: &Monitor) -> (u32, u32) {
    if (b.scale - m.scale).abs() < 1e-6 {
        return (b.width, b.height);
    }
    let f = m.scale / b.scale;
    (scaled(b.width, f), scaled(b.height, f))
}

fn logical((w, h): (u32, u32), scale: f64) -> (u32, u32) {
    (scaled(w, scale), scaled(h, scale))
}

fn scaled(v: u32, f: f64) -> u32 {
    let s = (f64::from(v) * f).round();
    if s >= f64::from(u32::MAX) {
        u32::MAX
    } else {
        s.max(0.0) as u32
    }
}

/// Clamp a size to [minimum, monitor]. When the monitor is smaller than the minimum, the
/// monitor wins (the window must fit on screen).
fn fit_size((w, h): (u32, u32), m: &Monitor, rules: SizeRules) -> (u32, u32) {
    let (min_w, min_h) = logical(rules.min, m.scale);
    let fit = |v: u32, min: u32, max: u32| v.max(min).min(max).max(1);
    (fit(w, min_w, m.width), fit(h, min_h, m.height))
}

/// Move a span so it lies inside the monitor's span (the size already fits).
fn clamp_axis(pos: i32, len: u32, m0: i32, mlen: u32) -> i32 {
    let (pos, len, m0, mlen) = (
        i64::from(pos),
        i64::from(len),
        i64::from(m0),
        i64::from(mlen),
    );
    let max = (m0 + mlen - len).max(m0);
    i32::try_from(pos.clamp(m0, max)).unwrap_or(i32::MAX)
}

fn centre(m0: i32, mlen: u32, len: u32) -> i32 {
    let offset = (i64::from(mlen) - i64::from(len)).max(0) / 2;
    i32::try_from(i64::from(m0) + offset).unwrap_or(i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const RULES: SizeRules = SizeRules {
        default: (1280, 800),
        min: (960, 600),
    };

    fn mon(x: i32, y: i32, width: u32, height: u32, scale: f64, primary: bool) -> Monitor {
        Monitor {
            x,
            y,
            width,
            height,
            scale,
            primary,
        }
    }

    /// A 1920 × 1040 work area at 100 %, primary.
    fn primary() -> Monitor {
        mon(0, 0, 1920, 1040, 1.0, true)
    }

    fn bounds(x: i32, y: i32, width: u32, height: u32) -> WindowBounds {
        WindowBounds {
            x,
            y,
            width,
            height,
            scale: 1.0,
            maximized: false,
        }
    }

    fn now(x: i32, y: i32, width: u32, height: u32) -> WindowNow {
        WindowNow {
            x,
            y,
            width,
            height,
            scale: 1.0,
            maximized: false,
            minimized: false,
        }
    }

    #[test]
    fn first_start_is_the_default_size_centred_on_the_primary_monitor() {
        let second = mon(-1920, 0, 1920, 1040, 1.0, false);
        let p = placement(None, &[second, primary()], RULES).unwrap();
        assert_eq!(
            p,
            Placement {
                x: 320,
                y: 120,
                width: 1280,
                height: 800,
                maximized: false
            }
        );
        // At 150 % the default logical size is 1920 × 1200 physical: clamped to the work area.
        let hi = mon(0, 0, 2560, 1400, 1.5, true);
        let p = placement(None, &[hi], RULES).unwrap();
        assert_eq!((p.width, p.height), (1920, 1200));
        assert_eq!((p.x, p.y), (320, 100));
    }

    #[test]
    fn a_visible_saved_position_is_kept() {
        let b = bounds(100, 50, 1400, 900);
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.x, p.y, p.width, p.height), (100, 50, 1400, 900));
        assert!(!p.maximized);
    }

    #[test]
    fn an_off_screen_saved_position_falls_back_to_the_primary_monitor_centred() {
        // Saved on a monitor that is gone (far right).
        let b = bounds(4000, 200, 1200, 800);
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.width, p.height), (1200, 800), "the size is kept");
        assert_eq!((p.x, p.y), (360, 120), "centred on the primary monitor");
        // Mostly off screen (only a strip of 100 px visible): also moved.
        let b = bounds(1820, 100, 1200, 800);
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.x, p.y), (360, 120));
    }

    #[test]
    fn a_mostly_visible_window_that_sticks_out_is_moved_inside() {
        let b = bounds(1000, -50, 1200, 800); // 920 of 1200 px across are visible.
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.x, p.y), (720, 0));
        assert_eq!((p.width, p.height), (1200, 800));
    }

    #[test]
    fn a_smaller_monitor_clamps_the_size_and_respects_the_minimum() {
        // Saved at 1900 × 1000 on a big screen; now only a 1366 × 728 work area.
        let small = mon(0, 0, 1366, 728, 1.0, true);
        let p = placement(Some(bounds(10, 10, 1900, 1000)), &[small], RULES).unwrap();
        assert_eq!((p.width, p.height), (1366, 728));
        assert_eq!((p.x, p.y), (0, 0));
        // A saved size below the minimum grows to the minimum.
        let p = placement(Some(bounds(10, 10, 500, 300)), &[primary()], RULES).unwrap();
        assert_eq!((p.width, p.height), (960, 600));
        // A monitor smaller than the minimum: the window still fits on it.
        let tiny = mon(0, 0, 800, 560, 1.0, true);
        let p = placement(None, &[tiny], RULES).unwrap();
        assert_eq!((p.x, p.y, p.width, p.height), (0, 0, 800, 560));
    }

    #[test]
    fn a_negative_position_on_a_secondary_monitor_to_the_left_is_kept() {
        let left = mon(-2560, -200, 2560, 1400, 1.0, false);
        let b = bounds(-2000, -100, 1300, 850);
        let p = placement(Some(b), &[primary(), left], RULES).unwrap();
        assert_eq!((p.x, p.y, p.width, p.height), (-2000, -100, 1300, 850));
        // The same bounds after that monitor was unplugged: back on the primary one.
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.x, p.y), (310, 95));
    }

    #[test]
    fn maximised_restores_the_normal_bounds_and_the_flag() {
        let b = WindowBounds {
            maximized: true,
            ..bounds(200, 100, 1300, 850)
        };
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert_eq!((p.x, p.y, p.width, p.height), (200, 100, 1300, 850));
        assert!(p.maximized);
        // Off screen and maximised: maximised on the primary monitor.
        let b = WindowBounds { x: 5000, ..b };
        let p = placement(Some(b), &[primary()], RULES).unwrap();
        assert!(p.maximized);
        assert_eq!((p.x, p.y), (310, 95));
    }

    #[test]
    fn another_scale_factor_keeps_the_logical_size() {
        // Saved at 100 % as 1200 × 800; the monitor is now at 150 %.
        let hi = mon(0, 0, 3840, 2100, 1.5, true);
        let p = placement(Some(bounds(100, 100, 1200, 800)), &[hi], RULES).unwrap();
        assert_eq!((p.width, p.height), (1800, 1200));
        // The minimum is logical too: 960 × 600 at 150 % is 1440 × 900.
        let p = placement(Some(bounds(100, 100, 700, 500)), &[hi], RULES).unwrap();
        assert_eq!((p.width, p.height), (1440, 900));
    }

    #[test]
    fn no_monitor_means_no_placement() {
        assert_eq!(placement(None, &[], RULES), None);
        // No monitor marked primary: the first one is used.
        let m = mon(0, 0, 1920, 1040, 1.0, false);
        assert!(placement(None, &[m], RULES).is_some());
    }

    #[test]
    fn saving_skips_minimised_and_keeps_normal_bounds_when_maximised() {
        let saved = bounds(100, 100, 1200, 800);
        let min = WindowNow {
            minimized: true,
            ..now(-32000, -32000, 160, 28)
        };
        assert_eq!(next_bounds(Some(saved), min), None);

        let max = WindowNow {
            maximized: true,
            ..now(-8, -8, 1936, 1056)
        };
        assert_eq!(
            next_bounds(Some(saved), max),
            Some(WindowBounds {
                maximized: true,
                ..saved
            })
        );
        let saved_max = WindowBounds {
            maximized: true,
            ..saved
        };
        assert_eq!(next_bounds(Some(saved_max), max), None, "unchanged");
        assert_eq!(next_bounds(None, max), None, "no normal bounds known yet");

        // Restored from maximised, then moved: the new normal bounds, not maximised.
        assert_eq!(
            next_bounds(Some(saved_max), now(300, 200, 1200, 800)),
            Some(bounds(300, 200, 1200, 800))
        );
        assert_eq!(
            next_bounds(Some(saved), now(100, 100, 1200, 800)),
            None,
            "unchanged"
        );
        assert_eq!(next_bounds(None, now(0, 0, 0, 0)), None, "no size");
    }

    #[test]
    fn the_setting_round_trips_and_bad_values_mean_nothing_saved() {
        let b = WindowBounds {
            maximized: true,
            scale: 1.25,
            ..bounds(-10, 20, 1300, 850)
        };
        let v = serde_json::to_value(b).unwrap();
        assert_eq!(parse_bounds(Some(v)), Some(b));
        for bad in [
            json!(null),
            json!("x"),
            json!({"x": 1}),
            json!({"x": 0, "y": 0, "width": 0, "height": 10, "scale": 1.0, "maximized": false}),
            json!({"x": 0, "y": 0, "width": 10, "height": 10, "scale": 0.0, "maximized": false}),
            json!({"x": 0, "y": 0, "width": 10, "height": 10, "scale": 1.0, "maximized": false, "z": 1}),
        ] {
            assert_eq!(parse_bounds(Some(bad.clone())), None, "{bad}");
        }
        assert_eq!(parse_bounds(None), None);
    }

    #[test]
    fn reset_is_the_default_size_centred() {
        let p = reset_placement(&primary(), RULES);
        assert_eq!(
            p,
            Placement {
                x: 320,
                y: 120,
                width: 1280,
                height: 800,
                maximized: false
            }
        );
    }
}
