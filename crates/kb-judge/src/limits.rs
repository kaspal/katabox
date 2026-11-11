//! Per-language time limits (ADR 0017).
//!
//! A problem has one base limit (`time_limit_ms` in problem.toml, set for Rust). Each
//! language multiplies it by its `[time] factor` (language.toml) and gets `startup_ms` of
//! CPU on top for starting its runtime. The machine's calibration factor (the time scale)
//! then stretches or shrinks the whole budget, within [`MIN_SCALE`, `MAX_SCALE`], so a
//! slower machine gets proportionally more time.
//!
//! All limits are CPU time for the whole run (every test in the batch), as in ADR 0008.

use crate::languages::LanguageDef;

/// The lowest time scale applied, however fast the machine measured.
pub const MIN_SCALE: f64 = 0.5;
/// The highest time scale applied, however slow the machine measured.
pub const MAX_SCALE: f64 = 3.0;

/// The scale actually applied: `scale` clamped to [`MIN_SCALE`, `MAX_SCALE`]; 1.0 for a
/// value that is not a positive number.
pub fn clamp_scale(scale: f64) -> f64 {
    if scale.is_finite() && scale > 0.0 {
        scale.clamp(MIN_SCALE, MAX_SCALE)
    } else {
        1.0
    }
}

fn ceil_ms(ms: f64) -> u32 {
    // Saturating: limits never get near u32::MAX ms, but a bad problem.toml must not wrap.
    ms.ceil().clamp(0.0, f64::from(u32::MAX)) as u32
}

/// The language's limit on the reference machine: `base × factor`.
pub fn language_limit_ms(base_ms: u32, def: &LanguageDef) -> u32 {
    ceil_ms(f64::from(base_ms) * def.time.factor)
}

/// The limit users see and the TLE message names: the language's limit at `scale`.
pub fn effective_limit_ms(base_ms: u32, def: &LanguageDef, scale: f64) -> u32 {
    ceil_ms(f64::from(base_ms) * def.time.factor * clamp_scale(scale))
}

/// The CPU cap for the sandboxed run: the language's limit plus its startup allowance,
/// both at `scale`.
pub fn cpu_cap_ms(base_ms: u32, def: &LanguageDef, scale: f64) -> u32 {
    let unscaled = f64::from(base_ms) * def.time.factor + f64::from(def.time.startup_ms);
    ceil_ms(unscaled * clamp_scale(scale))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::{SUPPORTED, TimeDef, definition};
    use kb_core::Language;

    fn def(factor: f64, startup_ms: u32) -> LanguageDef {
        let mut d = definition(Language::Python).unwrap();
        d.time = TimeDef { factor, startup_ms };
        d
    }

    #[test]
    fn limit_is_base_times_factor() {
        assert_eq!(language_limit_ms(300, &def(1.0, 300)), 300);
        assert_eq!(language_limit_ms(300, &def(10.0, 1000)), 3000);
        // Fractional factors round up, never down.
        assert_eq!(language_limit_ms(333, &def(2.5, 0)), 833);
        assert_eq!(language_limit_ms(u32::MAX, &def(10.0, 0)), u32::MAX);
    }

    #[test]
    fn cpu_cap_adds_the_startup_allowance_then_scales() {
        let d = def(5.0, 500);
        assert_eq!(cpu_cap_ms(200, &d, 1.0), 1500);
        assert_eq!(cpu_cap_ms(200, &d, 2.0), 3000);
        assert_eq!(cpu_cap_ms(200, &d, 0.5), 750);
        assert_eq!(effective_limit_ms(200, &d, 1.0), 1000);
        assert_eq!(effective_limit_ms(200, &d, 1.3), 1300);
        assert_eq!(effective_limit_ms(201, &d, 1.1), 1106); // 1105.5 rounds up
    }

    #[test]
    fn scale_is_clamped() {
        assert_eq!(clamp_scale(1.07), 1.07);
        assert_eq!(clamp_scale(0.3), MIN_SCALE);
        assert_eq!(clamp_scale(4.0), MAX_SCALE);
        for bad in [f64::NAN, f64::INFINITY, 0.0, -2.0] {
            assert_eq!(clamp_scale(bad), 1.0, "{bad}");
        }
        let d = def(1.0, 0);
        assert_eq!(effective_limit_ms(1000, &d, 10.0), 3000);
        assert_eq!(effective_limit_ms(1000, &d, 0.1), 500);
    }

    #[test]
    fn shipped_languages_are_ordered_by_speed() {
        // Rust is the base; every other language gets at least as much time.
        let rust = definition(Language::Rust).unwrap();
        for lang in SUPPORTED {
            let d = definition(lang).unwrap();
            assert!(
                language_limit_ms(100, &d) >= language_limit_ms(100, &rust),
                "{lang}"
            );
            assert!(
                cpu_cap_ms(100, &d, 1.0) > language_limit_ms(100, &d),
                "{lang}"
            );
        }
    }
}
