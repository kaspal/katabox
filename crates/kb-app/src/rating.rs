//! Skill rating (expansion M2, ADR 0033): an Elo-style rating per pattern and overall,
//! derived by replaying the user's history. It is not stored: the same history always
//! gives the same ratings, an import needs no merge rule, and the "back-fill" of earlier
//! solves is the replay itself.
//!
//! Every constant is here and in ADR 0033. The rating is an estimate to guide practice,
//! not a grade: the UI labels it "Estimated".

use std::collections::{BTreeMap, HashMap};

use kb_core::Difficulty;
use serde::Serialize;

/// Where every player starts.
pub const START: f64 = 1200.0;
/// A problem's rating: its difficulty's base plus `RANK_STEP` per rank above 1, for at
/// most `RANK_CAP` ranks.
pub const BASE_EASY: f64 = 1000.0;
pub const BASE_MEDIUM: f64 = 1400.0;
pub const BASE_HARD: f64 = 1800.0;
pub const RANK_STEP: f64 = 20.0;
pub const RANK_CAP: f64 = 10.0;
/// Minutes a solve may take before the time penalty starts.
pub const TARGET_EASY_MIN: f64 = 15.0;
pub const TARGET_MEDIUM_MIN: f64 = 30.0;
pub const TARGET_HARD_MIN: f64 = 45.0;
/// The score loses this much per extra submit and, at most, this much for time.
pub const SUBMIT_PENALTY: f64 = 0.1;
pub const TIME_PENALTY: f64 = 0.2;
/// The lowest score a solve can earn.
pub const SCORE_FLOOR: f64 = 0.3;
/// K factor: large while a scope has few events, then smaller.
pub const K_NEW: f64 = 40.0;
pub const K_MID: f64 = 24.0;
pub const K_SETTLED: f64 = 16.0;
pub const K_NEW_EVENTS: u32 = 10;
pub const K_MID_EVENTS: u32 = 30;
/// Practice weights rise for patterns rated below this (see `practice.rs`).
pub const PRACTICE_PIVOT: f64 = 1500.0;
/// Inside a pattern, Easy problems are skipped once the pattern's rating is this far
/// above theirs.
pub const SKIP_EASY_MARGIN: f64 = 400.0;

/// One solve that moves the rating.
#[derive(Debug, Clone, PartialEq)]
pub struct RatingEvent {
    /// UTC milliseconds.
    pub at: i64,
    pub pattern: String,
    pub difficulty: Difficulty,
    pub rank: u8,
    /// Submits up to and including the Accepted one (at least 1).
    pub submits: u32,
    /// Minutes from opening to Accepted, when known.
    pub minutes: Option<f64>,
}

pub fn problem_rating(difficulty: Difficulty, rank: u8) -> f64 {
    let base = match difficulty {
        Difficulty::Easy => BASE_EASY,
        Difficulty::Medium => BASE_MEDIUM,
        Difficulty::Hard => BASE_HARD,
    };
    base + RANK_STEP * (f64::from(rank.max(1)) - 1.0).min(RANK_CAP)
}

fn target_min(difficulty: Difficulty) -> f64 {
    match difficulty {
        Difficulty::Easy => TARGET_EASY_MIN,
        Difficulty::Medium => TARGET_MEDIUM_MIN,
        Difficulty::Hard => TARGET_HARD_MIN,
    }
}

/// How well a solve went, in [`SCORE_FLOOR`, 1]: one submit within the target time is 1.
pub fn score(difficulty: Difficulty, submits: u32, minutes: Option<f64>) -> f64 {
    let target = target_min(difficulty);
    let late = minutes.map_or(0.0, |m| ((m - target) / target).clamp(0.0, 1.0));
    let extra = f64::from(submits.max(1) - 1);
    (1.0 - SUBMIT_PENALTY * extra - TIME_PENALTY * late).clamp(SCORE_FLOOR, 1.0)
}

fn k_factor(events: u32) -> f64 {
    if events < K_NEW_EVENTS {
        K_NEW
    } else if events < K_MID_EVENTS {
        K_MID
    } else {
        K_SETTLED
    }
}

/// One rating: its value, how many events moved it, and its history.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub rating: f64,
    pub events: u32,
    /// (time, rating after the event), oldest first.
    pub history: Vec<(i64, f64)>,
}

impl Scope {
    /// A scope at the starting rating.
    fn starting() -> Scope {
        Scope {
            rating: START,
            events: 0,
            history: Vec::new(),
        }
    }

    fn apply(&mut self, at: i64, opponent: f64, s: f64) {
        let expected = 1.0 / (1.0 + 10f64.powf((opponent - self.rating) / 400.0));
        self.rating += k_factor(self.events) * (s - expected);
        self.events += 1;
        self.history.push((at, self.rating));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ratings {
    pub overall: Scope,
    pub patterns: BTreeMap<String, Scope>,
}

impl Ratings {
    /// Pattern → rating, for the practice weights (only patterns with events).
    pub fn by_pattern(&self) -> HashMap<String, f64> {
        self.patterns
            .iter()
            .map(|(p, s)| (p.clone(), s.rating))
            .collect()
    }
}

/// Replay `events` in time order (ties keep their order) into the ratings.
pub fn replay(events: &[RatingEvent]) -> Ratings {
    let mut order: Vec<&RatingEvent> = events.iter().collect();
    order.sort_by_key(|e| e.at);
    let mut overall = Scope::starting();
    let mut patterns: BTreeMap<String, Scope> = BTreeMap::new();
    for e in order {
        let opponent = problem_rating(e.difficulty, e.rank);
        let s = score(e.difficulty, e.submits, e.minutes);
        overall.apply(e.at, opponent, s);
        patterns
            .entry(e.pattern.clone())
            .or_insert_with(Scope::starting)
            .apply(e.at, opponent, s);
    }
    Ratings { overall, patterns }
}

/// The practice weight factor of a pattern: 1, up to 2 for patterns rated well below
/// [`PRACTICE_PIVOT`] (no rating: 1).
pub fn practice_factor(rating: Option<f64>) -> f64 {
    rating.map_or(1.0, |r| {
        1.0 + ((PRACTICE_PIVOT - r) / 400.0).clamp(0.0, 1.0)
    })
}

/// Whether `next problem` may skip an Easy problem of rating `problem` in a pattern
/// rated `pattern`.
pub fn skip_easy(pattern: Option<f64>, problem: f64) -> bool {
    pattern.is_some_and(|r| r >= problem + SKIP_EASY_MARGIN)
}

// --- Views --------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RatingPoint {
    pub at: i64,
    pub rating: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RatingView {
    /// Rounded to a whole number.
    pub rating: i32,
    pub events: u32,
    /// At most the last 30 points, oldest first.
    pub history: Vec<RatingPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PatternRating {
    pub pattern: String,
    pub rating: RatingView,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RatingsView {
    pub overall: RatingView,
    /// Patterns with at least one event.
    pub patterns: Vec<PatternRating>,
}

fn view(s: &Scope) -> RatingView {
    let skip = s.history.len().saturating_sub(30);
    RatingView {
        rating: s.rating.round() as i32,
        events: s.events,
        history: s.history[skip..]
            .iter()
            .map(|(at, r)| RatingPoint {
                at: *at,
                rating: r.round() as i32,
            })
            .collect(),
    }
}

pub fn ratings_view(r: &Ratings) -> RatingsView {
    RatingsView {
        overall: view(&r.overall),
        patterns: r
            .patterns
            .iter()
            .map(|(p, s)| PatternRating {
                pattern: p.clone(),
                rating: view(s),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(
        at: i64,
        pattern: &str,
        d: Difficulty,
        submits: u32,
        minutes: Option<f64>,
    ) -> RatingEvent {
        RatingEvent {
            at,
            pattern: pattern.into(),
            difficulty: d,
            rank: 1,
            submits,
            minutes,
        }
    }

    #[test]
    fn problem_ratings_and_scores() {
        assert_eq!(problem_rating(Difficulty::Easy, 1), 1000.0);
        assert_eq!(problem_rating(Difficulty::Medium, 3), 1440.0);
        assert_eq!(
            problem_rating(Difficulty::Hard, 40),
            2000.0,
            "rank is capped"
        );
        assert_eq!(problem_rating(Difficulty::Easy, 0), 1000.0);
        assert_eq!(score(Difficulty::Easy, 1, Some(10.0)), 1.0);
        assert_eq!(score(Difficulty::Easy, 1, None), 1.0);
        assert!((score(Difficulty::Easy, 3, None) - 0.8).abs() < 1e-12);
        // Twice the target time costs the whole time penalty.
        assert!((score(Difficulty::Medium, 1, Some(60.0)) - 0.8).abs() < 1e-12);
        assert!((score(Difficulty::Medium, 1, Some(45.0)) - 0.9).abs() < 1e-12);
        assert_eq!(score(Difficulty::Hard, 20, Some(500.0)), SCORE_FLOOR);
    }

    #[test]
    fn replay_is_deterministic_and_golden() {
        let events = vec![
            ev(3, "graphs", Difficulty::Medium, 2, Some(40.0)),
            ev(1, "arrays-hashing", Difficulty::Easy, 1, Some(5.0)),
            ev(2, "arrays-hashing", Difficulty::Easy, 1, None),
        ];
        let a = replay(&events);
        let b = replay(&events);
        assert_eq!(a, b);
        // Golden values: first event, E = 1 / (1 + 10^((1000 - 1200) / 400)).
        let e1 = 1.0 / (1.0 + 10f64.powf(-0.5));
        let r1 = START + K_NEW * (1.0 - e1);
        assert!((a.patterns["arrays-hashing"].history[0].1 - r1).abs() < 1e-9);
        assert_eq!(a.overall.events, 3);
        assert_eq!(a.patterns["arrays-hashing"].events, 2);
        assert_eq!(a.patterns["graphs"].events, 1);
        // Order is by time, not by the slice's order.
        assert_eq!(
            a.overall.history.iter().map(|h| h.0).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(a.overall.rating.round(), 1242.0);
        // A medium solved in 2 submits slightly late, from 1200: S = 1 - 0.1 - 0.2/3.
        let s = 1.0 - 0.1 - 0.2 * (10.0 / 30.0);
        let g = &a.patterns["graphs"];
        let e = 1.0 / (1.0 + 10f64.powf((1400.0 - START) / 400.0));
        assert!((g.rating - (START + K_NEW * (s - e))).abs() < 1e-9);
    }

    #[test]
    fn k_settles_with_experience() {
        let events: Vec<RatingEvent> = (0..40)
            .map(|i| ev(i, "stack", Difficulty::Medium, 1, None))
            .collect();
        let r = replay(&events);
        let steps: Vec<f64> = r.patterns["stack"]
            .history
            .windows(2)
            .map(|w| w[1].1 - w[0].1)
            .collect();
        // Winning steadily against 1400 rises, ever more slowly.
        assert!(steps.iter().all(|d| *d > 0.0));
        assert!(steps[35] < steps[15] && steps[15] < steps[2]);
        assert!(r.overall.rating > 1500.0);
    }

    #[test]
    fn practice_factor_and_easy_skip() {
        assert_eq!(practice_factor(None), 1.0);
        assert_eq!(practice_factor(Some(1600.0)), 1.0);
        assert_eq!(practice_factor(Some(1300.0)), 1.5);
        assert_eq!(practice_factor(Some(900.0)), 2.0);
        assert!(!skip_easy(None, 1000.0));
        assert!(!skip_easy(Some(1399.0), 1000.0));
        assert!(skip_easy(Some(1400.0), 1000.0));
    }

    #[test]
    fn views_round_and_keep_the_last_30_points() {
        let events: Vec<RatingEvent> = (0..45)
            .map(|i| ev(i, "heap", Difficulty::Easy, 1, None))
            .collect();
        let v = ratings_view(&replay(&events));
        assert_eq!(v.overall.history.len(), 30);
        assert_eq!(v.overall.history[0].at, 15);
        assert_eq!(v.patterns.len(), 1);
        assert_eq!(v.overall.events, 45);
    }
}

// --- App ----------------------------------------------------------------------------

/// The replay's events from the history: the first Accepted submission of each problem
/// (with the submits up to it and the minutes since it was first opened) and every
/// solved kata attempt. Problems no longer in the catalogue are left out.
pub fn events_from(
    problems: &[kb_store::ProblemSummary],
    submissions: &[(String, bool, i64)],
    first_opens: &HashMap<String, i64>,
    katas: &[kb_store::kata::KataAttemptRow],
) -> Vec<RatingEvent> {
    let meta: HashMap<&str, &kb_store::ProblemSummary> =
        problems.iter().map(|p| (p.id.as_str(), p)).collect();
    let mut tries: HashMap<&str, u32> = HashMap::new();
    let mut out = Vec::new();
    for (pid, ok, at) in submissions {
        let n = tries.entry(pid.as_str()).or_insert(0);
        if *n == u32::MAX {
            continue; // already solved
        }
        *n += 1;
        if *ok {
            if let Some(p) = meta.get(pid.as_str()) {
                let minutes = first_opens
                    .get(pid)
                    .filter(|o| **o <= *at)
                    .map(|o| (*at - *o) as f64 / 60_000.0);
                out.push(RatingEvent {
                    at: *at,
                    pattern: p.pattern.clone(),
                    difficulty: p.difficulty,
                    rank: p.rank,
                    submits: *n,
                    minutes,
                });
            }
            *n = u32::MAX;
        }
    }
    for a in katas {
        let (Some(at), Some(p)) = (a.solved_at, meta.get(a.problem_id.as_str())) else {
            continue;
        };
        out.push(RatingEvent {
            at,
            pattern: p.pattern.clone(),
            difficulty: p.difficulty,
            rank: p.rank,
            submits: a.submits.max(1),
            minutes: a.time_ms.map(|t| t as f64 / 60_000.0),
        });
    }
    out
}

impl crate::App {
    /// Drop the cached ratings (the history changed).
    pub(crate) fn forget_ratings(&self) {
        *self.ratings.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }

    /// The ratings, replayed once and cached until the history changes.
    pub(crate) async fn current_ratings(&self) -> crate::Result<std::sync::Arc<Ratings>> {
        if let Some(r) = self
            .ratings
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
        {
            return Ok(r);
        }
        let problems = self.all_problems(kb_store::UiLang::En).await?;
        let subs = self.store.user.submission_outcomes().await?;
        let opens = self.store.user.first_opens().await?;
        let katas = self.store.user.kata_attempts().await?;
        let r = std::sync::Arc::new(replay(&events_from(&problems, &subs, &opens, &katas)));
        *self.ratings.lock().unwrap_or_else(|e| e.into_inner()) = Some(r.clone());
        Ok(r)
    }

    /// Ratings per pattern and overall, for Progress.
    pub async fn ratings(&self) -> crate::Result<RatingsView> {
        Ok(ratings_view(&*self.current_ratings().await?))
    }
}

#[cfg(test)]
mod event_tests {
    use super::*;
    use kb_core::ProblemMode;

    fn prob(id: &str, d: Difficulty) -> kb_store::ProblemSummary {
        kb_store::ProblemSummary {
            id: id.into(),
            title: id.into(),
            difficulty: d,
            pattern: "stack".into(),
            rank: 2,
            mode: ProblemMode::Function,
            tags: vec![],
            status: None,
            fix: false,
            pack: None,
        }
    }

    #[test]
    fn first_accepted_counts_the_submits_and_the_time_since_opening() {
        let problems = [prob("a", Difficulty::Easy), prob("b", Difficulty::Hard)];
        let subs = vec![
            ("a".to_string(), false, 1_000),
            ("a".to_string(), false, 2_000),
            ("a".to_string(), true, 600_000),
            ("a".to_string(), true, 900_000), // a later Accepted moves nothing
            ("b".to_string(), false, 5_000),  // never solved
            ("gone".to_string(), true, 7_000), // not in the catalogue
        ];
        let opens: HashMap<String, i64> = [("a".to_string(), 0)].into();
        let ev = events_from(&problems, &subs, &opens, &[]);
        assert_eq!(ev.len(), 1);
        assert_eq!((ev[0].submits, ev[0].at, ev[0].rank), (3, 600_000, 2));
        assert!((ev[0].minutes.unwrap() - 10.0).abs() < 1e-9);
    }
}
