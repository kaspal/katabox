//! What to practise next (ADR 0019): the pattern map and "next problem", spaced-repetition
//! reviews, the daily problem with its streak, and personal lists.
//!
//! The rules are pure functions over the problem list, so they are unit tested without a
//! database; [`App`] methods load the rows (kb-store), apply the rules and save the result.
//! "Today" is always the user's local calendar date, sent by the UI.

use std::collections::{HashMap, HashSet};

use kb_core::{Day, Difficulty, Verdict};
use kb_store::{ActivityRow, ProblemFilter, ProblemStatus, ProblemSummary, ReviewRow, UiLang};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{App, AppError, AppErrorCode, Result};

/// The order patterns are learnt in (the pattern map and the "next problem" fallback):
/// the order of `pattern.*` in the UI's locale files. Patterns not listed sort after
/// these, by id.
pub const PATTERN_TRACK: [&str; 20] = [
    "arrays-hashing",
    "sorting",
    "two-pointers",
    "sliding-window",
    "stack",
    "binary-search",
    "linked-list",
    "trees",
    "tries",
    "heap",
    "backtracking",
    "graphs",
    "dynamic-programming",
    "greedy",
    "intervals",
    "math",
    "bit-manipulation",
    "strings",
    "union-find",
    "design",
];

/// Review intervals in days: after the first Accepted, then after each passed review.
pub const REVIEW_INTERVALS: [u32; 3] = [3, 7, 21];

/// Weight of a daily candidate in the weakest pattern (others weigh 1).
pub const WEAKEST_WEIGHT: u64 = 3;

/// Settings toggles (Settings → Practice); both default to on.
pub const REVIEW_SETTING: &str = "practice.review";
pub const DAILY_SETTING: &str = "practice.daily";

fn track_index(pattern: &str) -> usize {
    PATTERN_TRACK
        .iter()
        .position(|p| *p == pattern)
        .unwrap_or(PATTERN_TRACK.len())
}

fn pattern_order(a: &str, b: &str) -> std::cmp::Ordering {
    track_index(a).cmp(&track_index(b)).then_with(|| a.cmp(b))
}

fn solved(p: &ProblemSummary) -> bool {
    p.status == Some(ProblemStatus::Solved)
}

// --- Progress page ------------------------------------------------------------------

/// Solved and total problems of one difficulty or tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DifficultyCount {
    pub difficulty: Difficulty,
    pub solved: u32,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TagCount {
    pub tag: String,
    pub solved: u32,
    pub total: u32,
}

/// Progress → solved by difficulty and by topic (tag).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SolvedBreakdown {
    /// Easy, Medium, Hard, always all three.
    pub by_difficulty: Vec<DifficultyCount>,
    /// Every tag in the pack: most problems first, then by id.
    pub by_tag: Vec<TagCount>,
}

pub fn solved_breakdown(all: &[ProblemSummary]) -> SolvedBreakdown {
    let count = |of: &mut dyn Iterator<Item = &ProblemSummary>| {
        of.fold((0u32, 0u32), |(s, t), p| (s + u32::from(solved(p)), t + 1))
    };
    let by_difficulty = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard]
        .into_iter()
        .map(|d| {
            let (solved, total) = count(&mut all.iter().filter(|p| p.difficulty == d));
            DifficultyCount {
                difficulty: d,
                solved,
                total,
            }
        })
        .collect();
    let mut tags: Vec<&str> = all
        .iter()
        .flat_map(|p| p.tags.iter().map(String::as_str))
        .collect();
    tags.sort_unstable();
    tags.dedup();
    let mut by_tag: Vec<TagCount> = tags
        .into_iter()
        .map(|tag| {
            let (solved, total) =
                count(&mut all.iter().filter(|p| p.tags.iter().any(|t| t == tag)));
            TagCount {
                tag: tag.to_string(),
                solved,
                total,
            }
        })
        .collect();
    by_tag.sort_by(|a, b| b.total.cmp(&a.total).then_with(|| a.tag.cmp(&b.tag)));
    SolvedBreakdown {
        by_difficulty,
        by_tag,
    }
}

/// Progress → the review queue at a glance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReviewSummary {
    /// Due before today.
    pub overdue: u32,
    pub due_today: u32,
    /// Due tomorrow through 7 days from today.
    pub next_7_days: u32,
    /// Due after that.
    pub later: u32,
    /// The earliest due date after today (`YYYY-MM-DD`).
    pub next_due: Option<String>,
}

pub fn review_summary(rows: &[ReviewRow], today: Day) -> Result<ReviewSummary> {
    let mut s = ReviewSummary {
        overdue: 0,
        due_today: 0,
        next_7_days: 0,
        later: 0,
        next_due: None,
    };
    let mut next: Option<Day> = None;
    for r in rows {
        let due = r.due()?;
        match due.days() - today.days() {
            d if d < 0 => s.overdue += 1,
            0 => s.due_today += 1,
            d => {
                if d <= 7 {
                    s.next_7_days += 1;
                } else {
                    s.later += 1;
                }
                next = Some(next.map_or(due, |n| n.min(due)));
            }
        }
    }
    s.next_due = next.map(|d| d.to_string());
    Ok(s)
}

// --- Pattern map and next problem ---------------------------------------------------

/// One card of the pattern map.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PatternProgress {
    pub pattern: String,
    pub solved: u32,
    pub total: u32,
    /// The lowest-ranked unsolved problem of the pattern.
    pub next: Option<ProblemSummary>,
}

fn lowest_unsolved<'a>(
    all: &'a [ProblemSummary],
    pattern: &str,
    min_rank: u8,
    skip: &str,
) -> Option<&'a ProblemSummary> {
    all.iter()
        .filter(|p| p.pattern == pattern && p.rank >= min_rank && !solved(p) && p.id != skip)
        .min_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.id.cmp(&b.id)))
}

/// Patterns present in `all`, in track order, with progress.
pub fn pattern_progress(all: &[ProblemSummary]) -> Vec<PatternProgress> {
    let mut patterns: Vec<&str> = all.iter().map(|p| p.pattern.as_str()).collect();
    patterns.sort_by(|a, b| pattern_order(a, b));
    patterns.dedup();
    patterns
        .into_iter()
        .map(|pat| {
            let of: Vec<&ProblemSummary> = all.iter().filter(|p| p.pattern == pat).collect();
            PatternProgress {
                pattern: pat.to_string(),
                solved: of.iter().filter(|p| solved(p)).count() as u32,
                total: of.len() as u32,
                next: lowest_unsolved(all, pat, 0, "").cloned(),
            }
        })
        .collect()
}

/// What to solve after `current`: the lowest-ranked unsolved problem of the same pattern
/// at or above its rank; otherwise the lowest-ranked unsolved problem of the following
/// patterns in track order (wrapping around), then of the same pattern below its rank.
/// Never a solved problem, never `current`.
pub fn next_problem<'a>(all: &'a [ProblemSummary], current: &str) -> Option<&'a ProblemSummary> {
    let here = all.iter().find(|p| p.id == current)?;
    if let Some(p) = lowest_unsolved(all, &here.pattern, here.rank, current) {
        return Some(p);
    }
    let mut patterns: Vec<&str> = all.iter().map(|p| p.pattern.as_str()).collect();
    patterns.sort_by(|a, b| pattern_order(a, b));
    patterns.dedup();
    let at = patterns
        .iter()
        .position(|p| *p == here.pattern)
        .unwrap_or(0);
    for k in 1..=patterns.len() {
        let pat = patterns[(at + k) % patterns.len()];
        if let Some(p) = lowest_unsolved(all, pat, 0, current) {
            return Some(p);
        }
    }
    None
}

// --- Daily problem ------------------------------------------------------------------

/// The pattern with the lowest solved share among patterns that still have unsolved
/// problems (ties: track order).
fn weakest_pattern(all: &[ProblemSummary]) -> Option<String> {
    let mut counts: HashMap<&str, (u64, u64)> = HashMap::new();
    for p in all {
        let c = counts.entry(p.pattern.as_str()).or_default();
        c.1 += 1;
        if solved(p) {
            c.0 += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, (s, t))| s < t)
        .min_by(|(pa, (sa, ta)), (pb, (sb, tb))| {
            // sa/ta < sb/tb  <=>  sa*tb < sb*ta
            (sa * tb)
                .cmp(&(sb * ta))
                .then_with(|| pattern_order(pa, pb))
        })
        .map(|(p, _)| p.to_string())
}

/// The daily problem for `day`: a deterministic, weighted choice over the unsolved
/// problems (every problem once all are solved), seeded by the day and the install id.
/// Problems in the weakest pattern weigh [`WEAKEST_WEIGHT`].
pub fn daily_pick<'a>(
    all: &'a [ProblemSummary],
    day: Day,
    install_id: &str,
) -> Option<&'a ProblemSummary> {
    let mut candidates: Vec<&ProblemSummary> = all.iter().filter(|p| !solved(p)).collect();
    if candidates.is_empty() {
        candidates = all.iter().collect();
    }
    if candidates.is_empty() {
        return None;
    }
    candidates.sort_by(|a, b| a.id.cmp(&b.id));
    let weakest = weakest_pattern(all);
    let weight = |p: &ProblemSummary| {
        if weakest.as_deref() == Some(p.pattern.as_str()) {
            WEAKEST_WEIGHT
        } else {
            1
        }
    };
    let total: u64 = candidates.iter().map(|p| weight(p)).sum();
    let digest = Sha256::digest(format!("katabox-daily|{day}|{install_id}").as_bytes());
    let seed = u64::from_le_bytes(digest[..8].try_into().unwrap_or([0; 8]));
    let mut r = seed % total;
    for p in candidates {
        let w = weight(p);
        if r < w {
            return Some(p);
        }
        r -= w;
    }
    None
}

/// Consecutive days with a solved daily problem, ending today, or yesterday when today's
/// is not solved yet. `solved_days` is newest first.
pub fn streak(solved_days: &[Day], today: Day) -> u32 {
    let set: HashSet<Day> = solved_days.iter().copied().collect();
    let mut day = if set.contains(&today) {
        today
    } else {
        today.add_days(-1)
    };
    let mut n = 0;
    while set.contains(&day) {
        n += 1;
        day = day.add_days(-1);
    }
    n
}

// --- Reviews ------------------------------------------------------------------------

/// The review row for a problem solved for the first time today.
pub fn first_review(problem: &str, today: Day) -> ReviewRow {
    ReviewRow {
        problem_id: problem.to_string(),
        due_on: today.add_days(i64::from(REVIEW_INTERVALS[0])).to_string(),
        interval_days: REVIEW_INTERVALS[0],
        passes: 0,
        lapsed: false,
    }
}

/// What a review-mode Submit did to the schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ReviewOutcome {
    /// Passed: due again on `due_on`.
    Scheduled { due_on: String, interval_days: u32 },
    /// Passed the last interval: no more reviews.
    Graduated,
    /// Not Accepted: the interval is back to 3 days; still due today, try again.
    Lapsed,
}

/// Apply a review-mode Submit to `row`: `None` means the problem graduates.
pub fn review_after(
    row: &ReviewRow,
    today: Day,
    accepted: bool,
) -> (Option<ReviewRow>, ReviewOutcome) {
    if !accepted {
        let mut r = row.clone();
        r.interval_days = REVIEW_INTERVALS[0];
        r.lapsed = true;
        if r.due().map(|d| d > today).unwrap_or(true) {
            r.due_on = today.to_string();
        }
        return (Some(r), ReviewOutcome::Lapsed);
    }
    let next = if row.lapsed {
        Some(REVIEW_INTERVALS[0])
    } else {
        let at = REVIEW_INTERVALS
            .iter()
            .position(|i| *i == row.interval_days)
            .unwrap_or(0);
        REVIEW_INTERVALS.get(at + 1).copied()
    };
    match next {
        Some(days) => {
            let r = ReviewRow {
                problem_id: row.problem_id.clone(),
                due_on: today.add_days(i64::from(days)).to_string(),
                interval_days: days,
                passes: row.passes + 1,
                lapsed: false,
            };
            let outcome = ReviewOutcome::Scheduled {
                due_on: r.due_on.clone(),
                interval_days: days,
            };
            (Some(r), outcome)
        }
        None => (None, ReviewOutcome::Graduated),
    }
}

/// A problem's review state, for the workspace and Home.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReviewState {
    pub due_on: String,
    pub interval_days: u32,
    pub passes: u32,
    pub lapsed: bool,
    /// Due today or earlier.
    pub due: bool,
}

/// A due review on Home.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReviewItem {
    pub problem: ProblemSummary,
    pub review: ReviewState,
}

/// The daily problem on Home.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DailyProblem {
    /// `YYYY-MM-DD`, the local date it is for.
    pub day: String,
    pub problem: ProblemSummary,
    pub solved: bool,
    pub streak: u32,
}

/// What a Submit did beyond the verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PracticeOutcome {
    /// The problem was solved for the first time: its first review is scheduled.
    pub first_solve: bool,
    /// The review-mode result, when this Submit was a review.
    pub review: Option<ReviewOutcome>,
    /// This Submit solved today's daily problem.
    pub daily_solved: bool,
    pub streak: u32,
    /// What to solve next (after Accepted only).
    pub next: Option<ProblemSummary>,
}

// --- Lists --------------------------------------------------------------------------

/// One entry of a list: the problem, or its id when content no longer has it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListEntry {
    pub problem_id: String,
    /// `None`: the problem was removed from the problem pack (shown as unavailable).
    pub problem: Option<ProblemSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListView {
    pub id: i64,
    pub name: String,
    pub solved: u32,
    /// Available problems only.
    pub total: u32,
    pub entries: Vec<ListEntry>,
}

/// The longest list name, in characters.
pub const LIST_NAME_MAX: usize = 80;

fn list_name(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > LIST_NAME_MAX {
        return Err(AppError::new(
            AppErrorCode::InvalidInput,
            format!("a list name has 1 to {LIST_NAME_MAX} characters"),
        ));
    }
    Ok(name.to_string())
}

pub(crate) fn parse_day(today: &str) -> Result<Day> {
    today
        .parse()
        .map_err(|e: String| AppError::new(AppErrorCode::InvalidInput, e))
}

// --- App ----------------------------------------------------------------------------

impl App {
    /// Every problem with its status, titles in `lang`.
    pub(crate) async fn all_problems(&self, lang: UiLang) -> Result<Vec<ProblemSummary>> {
        Ok(self
            .store
            .list_problems(&ProblemFilter {
                lang: Some(lang),
                ..Default::default()
            })
            .await?)
    }

    /// Submits since `since_ms` (UTC ms), for the heatmap. The UI buckets them into the
    /// user's local days: kb-app never guesses the time zone.
    pub async fn activity(&self, since_ms: i64) -> Result<Vec<ActivityRow>> {
        Ok(self.store.user.activity_since(since_ms).await?)
    }

    pub async fn solved_breakdown(&self, lang: UiLang) -> Result<SolvedBreakdown> {
        Ok(solved_breakdown(&self.all_problems(lang).await?))
    }

    pub async fn review_summary(&self, today: &str) -> Result<ReviewSummary> {
        let day = parse_day(today)?;
        review_summary(&self.store.user.review_rows().await?, day)
    }

    pub async fn patterns(&self, lang: UiLang) -> Result<Vec<PatternProgress>> {
        Ok(pattern_progress(&self.all_problems(lang).await?))
    }

    pub async fn next_problem(
        &self,
        current: &str,
        lang: UiLang,
    ) -> Result<Option<ProblemSummary>> {
        let all = self.all_problems(lang).await?;
        Ok(next_problem(&all, current).cloned())
    }

    /// Today's daily problem (picked on the first call of the day, then kept all day).
    pub async fn daily_problem(&self, today: &str, lang: UiLang) -> Result<Option<DailyProblem>> {
        let day = parse_day(today)?;
        let all = self.all_problems(lang).await?;
        let pick = match self.store.user.daily_pick(day).await? {
            Some(p) if all.iter().any(|x| x.id == p.problem_id) => p,
            // No pick yet, or the picked problem left the pack: pick (again).
            existing => {
                let install = self.store.user.install_id().await?;
                let Some(choice) = daily_pick(&all, day, &install) else {
                    return Ok(None);
                };
                if existing.is_some() {
                    self.store.user.replace_daily_pick(day, &choice.id).await?
                } else {
                    self.store.user.put_daily_pick(day, &choice.id).await?
                }
            }
        };
        let problem = all
            .into_iter()
            .find(|p| p.id == pick.problem_id)
            .ok_or_else(|| AppError::new(AppErrorCode::Internal, "daily problem vanished"))?;
        Ok(Some(DailyProblem {
            day: day.to_string(),
            problem,
            solved: pick.solved,
            streak: self.streak(today).await?,
        }))
    }

    pub async fn streak(&self, today: &str) -> Result<u32> {
        let day = parse_day(today)?;
        Ok(streak(&self.store.user.solved_daily_days(day).await?, day))
    }

    pub async fn reviews_due(&self, today: &str, lang: UiLang) -> Result<Vec<ReviewItem>> {
        let day = parse_day(today)?;
        let all = self.all_problems(lang).await?;
        let mut out = Vec::new();
        for row in self.store.user.reviews_due(day).await? {
            // Problems no longer in the pack are left out (their row stays harmless).
            if let Some(p) = all.iter().find(|p| p.id == row.problem_id) {
                out.push(ReviewItem {
                    problem: p.clone(),
                    review: review_state(&row, day)?,
                });
            }
        }
        Ok(out)
    }

    pub async fn review_state(&self, problem: &str, today: &str) -> Result<Option<ReviewState>> {
        let day = parse_day(today)?;
        self.store
            .user
            .review(problem)
            .await?
            .map(|r| review_state(&r, day))
            .transpose()
    }

    /// Put off a review by one day.
    pub async fn review_snooze(&self, problem: &str, today: &str) -> Result<()> {
        let day = parse_day(today)?;
        let mut row = self.store.user.review(problem).await?.ok_or_else(|| {
            AppError::new(
                AppErrorCode::NotFound,
                format!("{problem} is not in review"),
            )
        })?;
        row.due_on = day.add_days(1).to_string();
        Ok(self.store.user.put_review(&row).await?)
    }

    pub async fn review_remove(&self, problem: &str) -> Result<()> {
        self.store.user.delete_review(problem).await?;
        Ok(())
    }

    /// Put a problem (back) in the review queue, due in 3 days.
    pub async fn review_add(&self, problem: &str, today: &str) -> Result<()> {
        let day = parse_day(today)?;
        Ok(self
            .store
            .user
            .put_review(&first_review(problem, day))
            .await?)
    }

    /// Practice bookkeeping for a Submit, after it was recorded. `was_solved`: the
    /// problem's status before this Submit.
    pub(crate) async fn after_submit(
        &self,
        problem: &str,
        verdict: Verdict,
        was_solved: bool,
        today: Day,
        review_mode: bool,
        lang: UiLang,
    ) -> Result<PracticeOutcome> {
        let accepted = verdict.is_accepted();
        let mut out = PracticeOutcome::default();
        let row = self.store.user.review(problem).await?;
        if review_mode && let Some(row) = &row {
            let (next, outcome) = review_after(row, today, accepted);
            match next {
                Some(r) => self.store.user.put_review(&r).await?,
                None => {
                    self.store.user.delete_review(problem).await?;
                }
            }
            out.review = Some(outcome);
        }
        if accepted && !was_solved {
            out.first_solve = true;
            if row.is_none() {
                self.store
                    .user
                    .put_review(&first_review(problem, today))
                    .await?;
            }
        }
        if accepted {
            out.daily_solved = self.store.user.solve_daily_pick(today, problem).await?;
            out.next = self.next_problem(problem, lang).await?;
        }
        out.streak = streak(&self.store.user.solved_daily_days(today).await?, today);
        Ok(out)
    }

    // --- Lists ----------------------------------------------------------------------

    pub async fn lists(&self, lang: UiLang) -> Result<Vec<ListView>> {
        let all = self.all_problems(lang).await?;
        let mut out = Vec::new();
        for l in self.store.user.lists().await? {
            let entries: Vec<ListEntry> = self
                .store
                .user
                .list_items(l.id)
                .await?
                .into_iter()
                .map(|id| ListEntry {
                    problem: all.iter().find(|p| p.id == id).cloned(),
                    problem_id: id,
                })
                .collect();
            let available: Vec<&ProblemSummary> =
                entries.iter().filter_map(|e| e.problem.as_ref()).collect();
            out.push(ListView {
                id: l.id,
                name: l.name,
                solved: available.iter().filter(|p| solved(p)).count() as u32,
                total: available.len() as u32,
                entries,
            });
        }
        Ok(out)
    }

    pub async fn create_list(&self, name: &str) -> Result<i64> {
        Ok(self.store.user.create_list(&list_name(name)?).await?.id)
    }

    pub async fn rename_list(&self, id: i64, name: &str) -> Result<()> {
        Ok(self.store.user.rename_list(id, &list_name(name)?).await?)
    }

    pub async fn delete_list(&self, id: i64) -> Result<()> {
        Ok(self.store.user.delete_list(id).await?)
    }

    pub async fn add_to_list(&self, id: i64, problem: &str) -> Result<()> {
        Ok(self.store.user.add_to_list(id, problem).await?)
    }

    pub async fn remove_from_list(&self, id: i64, problem: &str) -> Result<()> {
        Ok(self.store.user.remove_from_list(id, problem).await?)
    }

    pub async fn reorder_list(&self, id: i64, problems: &[String]) -> Result<()> {
        Ok(self.store.user.reorder_list(id, problems).await?)
    }

    /// Ids of the lists that hold `problem`.
    pub async fn lists_with(&self, problem: &str) -> Result<Vec<i64>> {
        Ok(self.store.user.lists_with(problem).await?)
    }
}

fn review_state(row: &ReviewRow, today: Day) -> Result<ReviewState> {
    Ok(ReviewState {
        due_on: row.due_on.clone(),
        interval_days: row.interval_days,
        passes: row.passes,
        lapsed: row.lapsed,
        due: row.due()? <= today,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::Difficulty;

    fn p(id: &str, pattern: &str, rank: u8, solved_: bool) -> ProblemSummary {
        ProblemSummary {
            id: id.into(),
            title: id.into(),
            difficulty: Difficulty::Easy,
            pattern: pattern.into(),
            rank,
            tags: vec![],
            status: solved_.then_some(ProblemStatus::Solved),
        }
    }

    fn day(s: &str) -> Day {
        s.parse().unwrap()
    }

    fn pack() -> Vec<ProblemSummary> {
        vec![
            p("a1", "arrays-hashing", 1, true),
            p("a3", "arrays-hashing", 3, false),
            p("a5", "arrays-hashing", 5, false),
            p("a2", "arrays-hashing", 2, false),
            p("t2", "two-pointers", 2, false),
            p("t1", "two-pointers", 1, true),
            p("g9", "graphs", 9, false),
            p("z1", "zzz-unknown", 1, false),
        ]
    }

    #[test]
    fn track_order_puts_unknown_patterns_last() {
        let map = pattern_progress(&pack());
        let order: Vec<&str> = map.iter().map(|m| m.pattern.as_str()).collect();
        assert_eq!(
            order,
            ["arrays-hashing", "two-pointers", "graphs", "zzz-unknown"]
        );
        assert_eq!((map[0].solved, map[0].total), (1, 4));
        assert_eq!(map[0].next.as_ref().unwrap().id, "a2");
        assert_eq!(map[1].next.as_ref().unwrap().id, "t2");
    }

    #[test]
    fn track_matches_the_pattern_order_of_the_ui_locales() {
        // The doc comment on PATTERN_TRACK promises this; the UI lists pattern.* in
        // the same order, so the two must not drift apart.
        for lang in ["en", "es"] {
            let path = format!(
                "{}/../../apps/desktop/src/i18n/locales/{lang}.json",
                env!("CARGO_MANIFEST_DIR")
            );
            let text = std::fs::read_to_string(&path).unwrap();
            // serde_json maps sort their keys, so read the file order from the
            // top-level "pattern" block (the locale files are formatted with indent 2).
            let start = text.find("\n  \"pattern\": {\n").expect("a pattern block") + 1;
            let keys: Vec<&str> = text[start..]
                .lines()
                .skip(1)
                .take_while(|l| !l.starts_with("  }"))
                .filter_map(|l| l.trim().strip_prefix('"')?.split('"').next())
                .filter(|k| !["label", "any"].contains(k))
                .collect();
            assert_eq!(keys, PATTERN_TRACK, "pattern.* order in {lang}.json");
        }
    }

    #[test]
    fn sorting_comes_right_after_arrays_and_hashing() {
        let all = vec![
            p("a1", "arrays-hashing", 1, false),
            p("s2", "sorting", 2, false),
            p("t1", "two-pointers", 1, false),
            p("d3", "design", 3, false),
        ];
        let order: Vec<String> = pattern_progress(&all)
            .into_iter()
            .map(|m| m.pattern)
            .collect();
        assert_eq!(
            order,
            ["arrays-hashing", "sorting", "two-pointers", "design"]
        );
        assert_eq!(next_problem(&all, "a1").unwrap().id, "s2");
        assert_eq!(next_problem(&all, "s2").unwrap().id, "t1");
    }

    #[test]
    fn next_problem_stays_in_the_pattern_at_or_above_the_rank() {
        let all = pack();
        // From a3 (rank 3): a5 is the only other unsolved one at rank 3 or more.
        assert_eq!(next_problem(&all, "a3").unwrap().id, "a5");
        // From a1 (rank 1): a2 (rank 2) before a3.
        assert_eq!(next_problem(&all, "a1").unwrap().id, "a2");
        // From a5 (the highest): the next pattern in track order.
        assert_eq!(next_problem(&all, "a5").unwrap().id, "t2");
        // From the last pattern: wraps around to the first.
        assert_eq!(next_problem(&all, "z1").unwrap().id, "a2");
        assert_eq!(next_problem(&all, "missing"), None);
    }

    #[test]
    fn next_problem_never_suggests_a_solved_one_or_itself() {
        let mut all = pack();
        for x in &mut all {
            if x.id != "g9" {
                x.status = Some(ProblemStatus::Solved);
            }
        }
        for x in all.clone() {
            let next = next_problem(&all, &x.id);
            assert!(next.is_none_or(|n| !solved(n) && n.id != x.id), "{next:?}");
        }
        assert_eq!(next_problem(&all, "a1").unwrap().id, "g9");
        assert_eq!(next_problem(&all, "g9"), None); // everything else is solved
    }

    #[test]
    fn daily_pick_is_stable_and_changes_with_the_day() {
        let all = pack();
        let d = day("2026-11-14");
        let first = daily_pick(&all, d, "install-a").unwrap().id.clone();
        for _ in 0..5 {
            assert_eq!(daily_pick(&all, d, "install-a").unwrap().id, first);
        }
        let seq = |install: &str| -> Vec<String> {
            (0..30)
                .map(|k| daily_pick(&all, d.add_days(k), install).unwrap().id.clone())
                .collect()
        };
        let mine = seq("install-a");
        assert!(mine.iter().collect::<HashSet<_>>().len() > 1, "{mine:?}");
        // Another install gets its own sequence.
        assert_ne!(mine, seq("install-b"));
    }

    #[test]
    fn daily_pick_never_picks_a_solved_problem_while_unsolved_remain() {
        let mut all = pack();
        for k in 0..365 {
            let pick = daily_pick(&all, day("2026-01-01").add_days(k), "i").unwrap();
            assert!(!solved(pick), "{}", pick.id);
        }
        for x in &mut all {
            x.status = Some(ProblemStatus::Solved);
        }
        assert!(daily_pick(&all, day("2026-01-01"), "i").is_some());
        assert_eq!(daily_pick(&[], day("2026-01-01"), "i"), None);
    }

    #[test]
    fn daily_pick_leans_toward_the_weakest_pattern() {
        // graphs: 0/2 solved (the weakest); stack: 3/5 solved, 2 unsolved.
        let all = vec![
            p("w1", "graphs", 5, false),
            p("w2", "graphs", 6, false),
            p("s1", "stack", 1, true),
            p("s2", "stack", 2, true),
            p("s3", "stack", 3, true),
            p("s4", "stack", 4, false),
            p("s5", "stack", 5, false),
        ];
        assert_eq!(weakest_pattern(&all).as_deref(), Some("graphs"));
        let start = day("2026-01-01");
        let weak = (0..3000)
            .filter(|k| daily_pick(&all, start.add_days(*k), "i").unwrap().pattern == "graphs")
            .count();
        // Expected share 6/8 = 75% (weights 3 + 3 against 1 + 1).
        assert!((2000..2500).contains(&weak), "{weak}");
    }

    #[test]
    fn streak_counts_consecutive_days() {
        let t = day("2026-11-14");
        assert_eq!(streak(&[], t), 0);
        assert_eq!(streak(&[t], t), 1);
        assert_eq!(streak(&[t, t.add_days(-1), t.add_days(-2)], t), 3);
        // Today not solved yet: the streak up to yesterday still counts.
        assert_eq!(streak(&[t.add_days(-1), t.add_days(-2)], t), 2);
        // A gap breaks it.
        assert_eq!(streak(&[t, t.add_days(-2)], t), 1);
        assert_eq!(streak(&[t.add_days(-2)], t), 0);
    }

    #[test]
    fn reviews_go_3_7_21_then_graduate() {
        let r = first_review("x", day("2026-11-14"));
        assert_eq!((r.due_on.as_str(), r.interval_days), ("2026-11-17", 3));
        let (r, o) = review_after(&r, day("2026-11-17"), true);
        let r = r.unwrap();
        assert_eq!(
            o,
            ReviewOutcome::Scheduled {
                due_on: "2026-11-24".into(),
                interval_days: 7
            }
        );
        assert_eq!(r.passes, 1);
        let (r, o) = review_after(&r, day("2026-11-24"), true);
        let r = r.unwrap();
        assert_eq!(
            o,
            ReviewOutcome::Scheduled {
                due_on: "2026-12-15".into(),
                interval_days: 21
            }
        );
        let (gone, o) = review_after(&r, day("2026-12-15"), true);
        assert_eq!((gone, o), (None, ReviewOutcome::Graduated));
    }

    #[test]
    fn a_failed_review_resets_to_3_days_and_stays_due() {
        let t = day("2026-11-24");
        let r = ReviewRow {
            problem_id: "x".into(),
            due_on: "2026-11-24".into(),
            interval_days: 7,
            passes: 1,
            lapsed: false,
        };
        let (r, o) = review_after(&r, t, false);
        let r = r.unwrap();
        assert_eq!(o, ReviewOutcome::Lapsed);
        assert_eq!(
            (r.interval_days, r.lapsed, r.due_on.as_str()),
            (3, true, "2026-11-24")
        );
        // Failing again changes nothing more.
        let (r, _) = review_after(&r, t, false);
        // Passing after a lapse schedules 3 days, not the next interval.
        let (r, o) = review_after(&r.unwrap(), t, true);
        assert_eq!(
            o,
            ReviewOutcome::Scheduled {
                due_on: "2026-11-27".into(),
                interval_days: 3
            }
        );
        assert!(!r.unwrap().lapsed);
    }

    #[test]
    fn list_names_are_trimmed_and_bounded() {
        assert_eq!(list_name("  Warm-up  ").unwrap(), "Warm-up");
        assert!(list_name("   ").is_err());
        assert!(list_name(&"x".repeat(81)).is_err());
        assert!(list_name(&"ñ".repeat(80)).is_ok());
    }

    #[test]
    fn solved_breakdown_counts_difficulties_and_tags() {
        let mut all = pack();
        all[1].difficulty = Difficulty::Medium; // a3, unsolved
        all[6].difficulty = Difficulty::Hard; // g9, unsolved
        all[0].tags = vec!["array".into(), "hash-map".into()]; // a1, solved
        all[1].tags = vec!["array".into()];
        all[5].tags = vec!["two-pointers".into()]; // t1, solved
        let b = solved_breakdown(&all);
        let d: Vec<(Difficulty, u32, u32)> = b
            .by_difficulty
            .iter()
            .map(|c| (c.difficulty, c.solved, c.total))
            .collect();
        assert_eq!(
            d,
            [
                (Difficulty::Easy, 2, 6),
                (Difficulty::Medium, 0, 1),
                (Difficulty::Hard, 0, 1)
            ]
        );
        let t: Vec<(&str, u32, u32)> = b
            .by_tag
            .iter()
            .map(|c| (c.tag.as_str(), c.solved, c.total))
            .collect();
        assert_eq!(
            t,
            [("array", 1, 2), ("hash-map", 1, 1), ("two-pointers", 1, 1)]
        );
        let empty = solved_breakdown(&[]);
        assert_eq!(empty.by_difficulty.len(), 3);
        assert!(empty.by_tag.is_empty());
    }

    #[test]
    fn review_summary_buckets_by_due_date() {
        let r = |id: &str, due: &str| ReviewRow {
            problem_id: id.into(),
            due_on: due.into(),
            interval_days: 3,
            passes: 0,
            lapsed: false,
        };
        let today = day("2026-03-10");
        let rows = [
            r("old", "2026-03-01"),
            r("today", "2026-03-10"),
            r("tomorrow", "2026-03-11"),
            r("week", "2026-03-17"),
            r("later", "2026-03-18"),
        ];
        let s = review_summary(&rows, today).unwrap();
        assert_eq!(
            (s.overdue, s.due_today, s.next_7_days, s.later),
            (1, 1, 2, 1)
        );
        assert_eq!(s.next_due.as_deref(), Some("2026-03-11"));
        let none = review_summary(&[], today).unwrap();
        assert_eq!(
            (none.overdue + none.due_today + none.later, none.next_due),
            (0, None)
        );
        assert!(review_summary(&[r("bad", "2026-13-01")], today).is_err());
    }
}
