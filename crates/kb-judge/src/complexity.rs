//! The complexity fit (ADR 0024): which growth class best explains the timings.
//!
//! For each class `f` the model is `t = c + a·f(n)` with `a, c ≥ 0`: `c` absorbs a fixed
//! per-call overhead, `a` the cost per unit of work. The fit is least squares on
//! **relative** errors (weights 1/t²), so a 50 µs point counts as much as a 5 s one.
//! Occam's razor picks the class: the **simplest** class whose error is within
//! [`PARSIMONY`] times the best error wins, so a more complex class has to fit clearly
//! better (caches and allocation make large inputs a little slower per element, which
//! otherwise reads as an extra log factor); the best-fitting class is then shown as the
//! alternative. `f` is evaluated
//! relative to the largest size (`f(n) / f(n_max)`, in the log domain for 2ⁿ), so huge
//! values never overflow and `a` keeps a sane scale.

use kb_core::profile::{ComplexityClass, Estimate};

/// At least this many sizes before anything is estimated.
pub const MIN_POINTS: usize = 3;
/// The runner-up is shown too when its error is within this factor of the best one.
pub const CLOSE_RATIO: f64 = 1.5;
/// A simpler class wins while its error is within this factor of the best one. Tuned on
/// the calibration set (ADR 0024: 2.5-3.0 give the same hits; 1.0 confuses O(n) with
/// O(n log n), 4.0 starts calling O(n²) O(n log n)).
pub const PARSIMONY: f64 = 2.5;
/// Points faster than this are dominated by noise.
pub const NOISY_US: f64 = 1000.0;

/// `f(n) / f(n_max)` for a class.
fn relative(class: ComplexityClass, n: f64, n_max: f64) -> f64 {
    let ln = |x: f64| x.max(2.0).ln();
    match class {
        ComplexityClass::Constant => 1.0,
        ComplexityClass::Logarithmic => ln(n) / ln(n_max),
        ComplexityClass::Linear => n / n_max,
        ComplexityClass::Linearithmic => (n * ln(n)) / (n_max * ln(n_max)),
        ComplexityClass::Quadratic => (n / n_max) * (n / n_max),
        ComplexityClass::Exponential => ((n - n_max) * std::f64::consts::LN_2).exp(),
    }
}

/// The weighted relative error of the best `c + a·f` for one class (mean squared).
fn class_error(class: ComplexityClass, points: &[(f64, f64)]) -> f64 {
    let n_max = points.iter().map(|p| p.0).fold(1.0, f64::max);
    let rows: Vec<(f64, f64, f64)> = points
        .iter()
        .map(|&(n, t)| (relative(class, n, n_max), t, 1.0 / (t * t)))
        .collect();
    // Weighted sums for the normal equations of t ≈ c + a·f.
    let (mut sw, mut sf, mut sff, mut st, mut sft) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for &(f, t, w) in &rows {
        sw += w;
        sf += w * f;
        sff += w * f * f;
        st += w * t;
        sft += w * f * t;
    }
    let error = |a: f64, c: f64| {
        rows.iter()
            .map(|&(f, t, w)| {
                let r = t - (c + a * f);
                w * r * r
            })
            .sum::<f64>()
            / rows.len() as f64
    };
    let det = sw * sff - sf * sf;
    let mut best = f64::INFINITY;
    if det.abs() > 1e-300 {
        let a = (sw * sft - sf * st) / det;
        let c = (sff * st - sf * sft) / det;
        if a >= 0.0 && c >= 0.0 {
            best = error(a, c);
        }
    }
    // The boundaries: only overhead (a = 0), or no overhead (c = 0).
    best = best.min(error(0.0, st / sw));
    if sff > 0.0 {
        best = best.min(error((sft / sff).max(0.0), 0.0));
    }
    best
}

/// The estimated class of timings `(n, microseconds)`, or `None` with fewer than
/// [`MIN_POINTS`] distinct sizes.
pub fn estimate(points: &[(u32, u32)]) -> Option<Estimate> {
    let mut pts: Vec<(f64, f64)> = points
        .iter()
        .map(|&(n, us)| (f64::from(n), f64::from(us.max(1))))
        .collect();
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    pts.dedup_by(|a, b| a.0 == b.0);
    if pts.len() < MIN_POINTS {
        return None;
    }
    let errors: Vec<(ComplexityClass, f64)> = ComplexityClass::ALL
        .iter()
        .map(|&c| (c, class_error(c, &pts)))
        .collect();
    let mut ranked = errors.clone();
    // Stable sort: equal errors keep the simpler class first.
    ranked.sort_by(|a, b| a.1.total_cmp(&b.1));
    let (best, e1) = ranked[0];
    let (second, e2) = ranked[1];
    // The simplest class that fits almost as well as the best (ALL is simplest first).
    let chosen = errors
        .iter()
        .find(|(_, e)| *e <= e1 * PARSIMONY + 1e-12)
        .map_or(best, |(c, _)| *c);
    let confidence = if e2 <= 0.0 || !e2.is_finite() {
        if e2.is_finite() { 0.0 } else { 1.0 }
    } else {
        (1.0 - e1 / e2).clamp(0.0, 1.0)
    };
    let close_alternative = if chosen != best {
        Some(best)
    } else {
        (e2.is_finite() && e2 <= e1 * CLOSE_RATIO + 1e-12).then_some(second)
    };
    let noisy = pts.iter().filter(|p| p.1 < NOISY_US).count() * 2 > pts.len();
    Some(Estimate {
        class: chosen,
        confidence,
        close_alternative,
        noisy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ComplexityClass::*;

    /// Timings for `f` at the sizes, with a fixed overhead and ± `jitter` (deterministic).
    fn timings(
        sizes: &[u32],
        cost: impl Fn(f64) -> f64,
        overhead: f64,
        jitter: f64,
    ) -> Vec<(u32, u32)> {
        sizes
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                let wobble = if i % 2 == 0 {
                    1.0 + jitter
                } else {
                    1.0 - jitter
                };
                (n, ((overhead + cost(f64::from(n))) * wobble).round() as u32)
            })
            .collect()
    }

    type Cost = Box<dyn Fn(f64) -> f64>;

    const SIZES: [u32; 6] = [300, 1_000, 3_000, 10_000, 30_000, 100_000];

    #[test]
    fn names_each_polynomial_class() {
        let cases: [(ComplexityClass, Cost); 4] = [
            (Linear, Box::new(|n| 0.05 * n)),
            (Linearithmic, Box::new(|n| 0.02 * n * n.ln())),
            (Quadratic, Box::new(|n| 0.0005 * n * n)),
            (Logarithmic, Box::new(|n| 400.0 * n.ln())),
        ];
        for (want, cost) in cases {
            for jitter in [0.0, 0.05] {
                let e = estimate(&timings(&SIZES, &cost, 30.0, jitter)).unwrap();
                assert_eq!(e.class, want, "jitter {jitter}: {e:?}");
            }
        }
    }

    #[test]
    fn constant_time_and_noise() {
        let flat = timings(&SIZES, |_| 0.0, 2_000.0, 0.05);
        assert_eq!(estimate(&flat).unwrap().class, Constant);
        let tiny = timings(&SIZES, |n| 0.001 * n, 5.0, 0.0);
        assert!(estimate(&tiny).unwrap().noisy);
    }

    #[test]
    fn names_exponential_growth_on_small_sizes() {
        let sizes = [10, 12, 14, 16, 18, 20];
        let e = estimate(&timings(&sizes, |n| 0.01 * 1.84f64.powf(n), 20.0, 0.05)).unwrap();
        assert_eq!(e.class, Exponential, "{e:?}");
        let e = estimate(&timings(&sizes, |n| 3.0 * n * n, 20.0, 0.05)).unwrap();
        assert_ne!(e.class, Exponential, "{e:?}");
    }

    #[test]
    fn prefers_the_simpler_class_unless_the_complex_one_fits_clearly_better() {
        // Linear work that gets slower per element at large n (caches): a little
        // super-linear. Still O(n), with O(n log n) shown as the alternative.
        let e = estimate(&timings(
            &SIZES,
            |n| 0.05 * n * (1.0 + n / 400_000.0),
            20.0,
            0.0,
        ))
        .unwrap();
        assert_eq!(e.class, Linear, "{e:?}");
        assert_eq!(e.close_alternative, Some(Linearithmic), "{e:?}");
        // A real n log n is not argued away.
        let e = estimate(&timings(&SIZES, |n| 0.02 * n * n.ln(), 20.0, 0.03)).unwrap();
        assert_eq!(e.class, Linearithmic, "{e:?}");
    }

    #[test]
    fn needs_three_sizes_and_reports_close_calls() {
        assert!(estimate(&[(100, 5), (1000, 50)]).is_none());
        assert!(estimate(&[(100, 5), (100, 6), (1000, 50)]).is_none());
        // Exactly linear: n log n is clearly worse, so no close alternative.
        let e = estimate(&timings(&SIZES, |n| 0.1 * n, 0.0, 0.0)).unwrap();
        assert_eq!(e.class, Linear);
        assert!(e.confidence > 0.5, "{e:?}");
    }
}
