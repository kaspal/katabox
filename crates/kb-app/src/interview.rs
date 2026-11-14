//! Mock interviews (ADR 0020): two never-opened problems against a wall-clock timer. While
//! a session runs, help for its problems is locked here, in kb-app (the UI only reflects
//! it): hints, editorial, earlier submissions and history, and accepted solutions.

use std::collections::HashSet;

use kb_core::{Difficulty, Language, Verdict};
use kb_store::{ProblemStatus, ProblemSummary, SessionRow, SubmitFact, UiLang};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{App, AppError, AppErrorCode, Result};

/// Allowed durations, in minutes.
pub const DURATIONS_MIN: [u32; 3] = [30, 45, 60];

/// Allowed difficulty mixes (one difficulty per problem).
pub const MIXES: [&str; 5] = [
    "easy+easy",
    "easy+medium",
    "medium+medium",
    "medium+hard",
    "hard+hard",
];

/// What the setup dialog sends.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct InterviewSetup {
    pub duration_min: u32,
    pub mix: String,
    /// The languages allowed during the session (at least one).
    pub languages: Vec<Language>,
}

/// The running session.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct InterviewView {
    pub id: i64,
    /// UTC milliseconds.
    pub started_at: i64,
    pub ends_at: i64,
    pub duration_s: i64,
    pub remaining_s: i64,
    pub mix: String,
    pub languages: Vec<Language>,
    pub problems: Vec<ProblemSummary>,
}

/// One problem of a scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProblemScore {
    pub problem_id: String,
    pub solved: bool,
    /// Seconds from the start to the first Accepted submit.
    pub time_to_solve_s: Option<i64>,
    pub submits: u32,
    /// The first submit was Accepted.
    pub first_try: bool,
}

/// A finished session's scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct InterviewResult {
    pub id: i64,
    pub started_at: i64,
    pub ended_at: i64,
    pub duration_s: i64,
    /// Seconds actually used (ended early or at the deadline).
    pub used_s: i64,
    pub mix: String,
    pub languages: Vec<Language>,
    pub problems: Vec<ProblemScore>,
    pub solved: u32,
    pub first_try: u32,
    /// Problem titles in the requested UI language (`problem_id` → title; missing when the
    /// problem left the pack).
    pub titles: Vec<(String, String)>,
}

fn mix_difficulties(mix: &str) -> Option<[Difficulty; 2]> {
    let d = |s: &str| match s {
        "easy" => Some(Difficulty::Easy),
        "medium" => Some(Difficulty::Medium),
        "hard" => Some(Difficulty::Hard),
        _ => None,
    };
    let (a, b) = mix.split_once('+')?;
    MIXES.contains(&mix).then_some([d(a)?, d(b)?])
}

/// Difficulties to try for a slot, nearest first.
fn nearest(d: Difficulty) -> [Difficulty; 3] {
    match d {
        Difficulty::Easy => [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard],
        Difficulty::Medium => [Difficulty::Medium, Difficulty::Easy, Difficulty::Hard],
        Difficulty::Hard => [Difficulty::Hard, Difficulty::Medium, Difficulty::Easy],
    }
}

/// Two problems for a session: unsolved, never touched, one per slot of the mix (nearest
/// difficulty when a slot has none), chosen deterministically from `seed`.
pub fn pick_problems<'a>(
    all: &'a [ProblemSummary],
    touched: &HashSet<String>,
    mix: [Difficulty; 2],
    seed: i64,
) -> Option<[&'a ProblemSummary; 2]> {
    let mut fresh: Vec<&ProblemSummary> = all
        .iter()
        .filter(|p| p.status != Some(ProblemStatus::Solved) && !touched.contains(&p.id))
        .collect();
    fresh.sort_by(|a, b| a.id.cmp(&b.id));
    let mut chosen: Vec<&ProblemSummary> = Vec::new();
    for (slot, want) in mix.iter().enumerate() {
        let pick = nearest(*want).into_iter().find_map(|d| {
            let pool: Vec<&&ProblemSummary> = fresh
                .iter()
                .filter(|p| p.difficulty == d && !chosen.iter().any(|c| c.id == p.id))
                .collect();
            if pool.is_empty() {
                return None;
            }
            let h = Sha256::digest(format!("katabox-interview|{seed}|{slot}").as_bytes());
            let r = u64::from_le_bytes(h[..8].try_into().unwrap_or([0; 8]));
            Some(*pool[(r % pool.len() as u64) as usize])
        })?;
        chosen.push(pick);
    }
    Some([chosen[0], chosen[1]])
}

/// The scorecard from the session's submits (oldest first).
pub fn score(session: &SessionRow, ended_at: i64, facts: &[SubmitFact]) -> InterviewResult {
    let problems: Vec<ProblemScore> = session
        .problem_ids
        .iter()
        .map(|id| {
            let mine: Vec<&SubmitFact> = facts.iter().filter(|f| &f.problem_id == id).collect();
            let first_ok = mine.iter().find(|f| f.verdict == Verdict::Accepted);
            ProblemScore {
                problem_id: id.clone(),
                solved: first_ok.is_some(),
                time_to_solve_s: first_ok.map(|f| (f.created_at - session.started_at) / 1000),
                submits: mine.len() as u32,
                first_try: mine.first().is_some_and(|f| f.verdict == Verdict::Accepted),
            }
        })
        .collect();
    InterviewResult {
        id: session.id,
        started_at: session.started_at,
        ended_at,
        duration_s: session.duration_s,
        used_s: ((ended_at - session.started_at) / 1000).clamp(0, session.duration_s),
        mix: session.mix.clone(),
        languages: session.languages.clone(),
        solved: problems.iter().filter(|p| p.solved).count() as u32,
        first_try: problems.iter().filter(|p| p.first_try).count() as u32,
        problems,
        titles: Vec::new(),
    }
}

fn deadline(s: &SessionRow) -> i64 {
    s.started_at + s.duration_s * 1000
}

pub(crate) fn locked() -> AppError {
    AppError::new(
        AppErrorCode::LockedInInterview,
        "locked during the mock interview",
    )
}

impl App {
    /// The running session, ending it first if its time is up.
    pub(crate) async fn running_session(&self) -> Result<Option<SessionRow>> {
        let Some(s) = self.store.user.open_session().await? else {
            return Ok(None);
        };
        if self.now_ms() >= deadline(&s) {
            self.finish_session(&s, deadline(&s)).await?;
            return Ok(None);
        }
        Ok(Some(s))
    }

    async fn finish_session(&self, s: &SessionRow, ended_at: i64) -> Result<InterviewResult> {
        let facts = self
            .store
            .user
            .submit_facts(&s.problem_ids, s.started_at, ended_at)
            .await?;
        let result = score(s, ended_at, &facts);
        let value = serde_json::to_value(&result)
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        self.store.user.end_session(s.id, ended_at, &value).await?;
        Ok(result)
    }

    /// Refuse when `problem` is part of the running interview.
    pub(crate) async fn ensure_unlocked(&self, problem: &str) -> Result<()> {
        match self.running_session().await? {
            Some(s) if s.problem_ids.iter().any(|p| p == problem) => Err(locked()),
            _ => Ok(()),
        }
    }

    /// Whether `problem` is part of the running interview.
    pub(crate) async fn in_interview(&self, problem: &str) -> Result<bool> {
        Ok(self
            .running_session()
            .await?
            .is_some_and(|s| s.problem_ids.iter().any(|p| p == problem)))
    }

    /// Record that the workspace opened `problem` (interviews pick never-opened problems).
    pub async fn mark_opened(&self, problem: &str) -> Result<()> {
        Ok(self.store.user.mark_opened(problem).await?)
    }

    pub async fn start_interview(
        &self,
        setup: &InterviewSetup,
        lang: UiLang,
    ) -> Result<InterviewView> {
        let bad = |m: &str| AppError::new(AppErrorCode::InvalidInput, m.to_string());
        if !DURATIONS_MIN.contains(&setup.duration_min) {
            return Err(bad("duration must be 30, 45 or 60 minutes"));
        }
        let mix = mix_difficulties(&setup.mix).ok_or_else(|| bad("unknown difficulty mix"))?;
        let mut languages = setup.languages.clone();
        languages.dedup();
        if languages.is_empty()
            || languages
                .iter()
                .any(|l| !kb_judge::languages::SUPPORTED.contains(l))
        {
            return Err(bad("pick at least one judge language"));
        }
        if self.running_session().await?.is_some() {
            return Err(AppError::new(
                AppErrorCode::InterviewActive,
                "a mock interview is already running",
            ));
        }
        let all = self.all_problems(lang).await?;
        let touched = self.store.user.touched_problems().await?;
        let now = self.now_ms();
        let [a, b] = pick_problems(&all, &touched, mix, now).ok_or_else(|| {
            AppError::new(
                AppErrorCode::NotEnoughProblems,
                "fewer than two unsolved problems you have never opened",
            )
        })?;
        let ids = vec![a.id.clone(), b.id.clone()];
        let s = self
            .store
            .user
            .create_session(
                now,
                i64::from(setup.duration_min) * 60,
                &ids,
                &languages,
                &setup.mix,
            )
            .await?;
        self.view(&s, lang).await
    }

    async fn view(&self, s: &SessionRow, lang: UiLang) -> Result<InterviewView> {
        let all = self.all_problems(lang).await?;
        let now = self.now_ms();
        Ok(InterviewView {
            id: s.id,
            started_at: s.started_at,
            ends_at: deadline(s),
            duration_s: s.duration_s,
            remaining_s: ((deadline(s) - now) / 1000).max(0),
            mix: s.mix.clone(),
            languages: s.languages.clone(),
            problems: s
                .problem_ids
                .iter()
                .filter_map(|id| all.iter().find(|p| &p.id == id).cloned())
                .collect(),
        })
    }

    /// The running session, if any (ending it if its time is up).
    pub async fn interview_active(&self, lang: UiLang) -> Result<Option<InterviewView>> {
        match self.running_session().await? {
            Some(s) => Ok(Some(self.view(&s, lang).await?)),
            None => Ok(None),
        }
    }

    /// End the running session now and return its scorecard.
    pub async fn end_interview(&self, lang: UiLang) -> Result<InterviewResult> {
        let s = match self.running_session().await? {
            Some(s) => s,
            None => {
                return Err(AppError::new(
                    AppErrorCode::NotFound,
                    "no mock interview is running",
                ));
            }
        };
        let end = self.now_ms().min(deadline(&s));
        let mut r = self.finish_session(&s, end).await?;
        self.add_titles(&mut r, lang).await?;
        Ok(r)
    }

    /// Past interviews, newest first.
    pub async fn interviews(&self, lang: UiLang) -> Result<Vec<InterviewResult>> {
        self.running_session().await?; // finish one whose time is up
        let mut out = Vec::new();
        for s in self.store.user.sessions().await? {
            let Some(ended) = s.ended_at else { continue };
            let mut r = match s
                .results
                .clone()
                .and_then(|v| serde_json::from_value::<InterviewResult>(v).ok())
            {
                Some(r) => r,
                // Imported without a scorecard: compute it from the submissions.
                None => {
                    let facts = self
                        .store
                        .user
                        .submit_facts(&s.problem_ids, s.started_at, ended)
                        .await?;
                    score(&s, ended, &facts)
                }
            };
            r.id = s.id;
            self.add_titles(&mut r, lang).await?;
            out.push(r);
        }
        Ok(out)
    }

    async fn add_titles(&self, r: &mut InterviewResult, lang: UiLang) -> Result<()> {
        let all = self.all_problems(lang).await?;
        r.titles = r
            .problems
            .iter()
            .filter_map(|p| {
                all.iter()
                    .find(|x| x.id == p.problem_id)
                    .map(|x| (x.id.clone(), x.title.clone()))
            })
            .collect();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: &str, d: Difficulty, solved: bool) -> ProblemSummary {
        ProblemSummary {
            id: id.into(),
            title: id.into(),
            difficulty: d,
            pattern: "x".into(),
            rank: 1,
            mode: kb_core::ProblemMode::Function,
            tags: vec![],
            status: solved.then_some(ProblemStatus::Solved),
        }
    }

    #[test]
    fn mixes_parse_and_reject_unknowns() {
        assert_eq!(
            mix_difficulties("easy+medium"),
            Some([Difficulty::Easy, Difficulty::Medium])
        );
        assert_eq!(mix_difficulties("medium+easy"), None);
        assert_eq!(mix_difficulties("easy"), None);
    }

    #[test]
    fn picks_only_fresh_unsolved_problems_with_fallback() {
        let all = vec![
            p("e1", Difficulty::Easy, false),
            p("e2", Difficulty::Easy, true),
            p("e3", Difficulty::Easy, false),
            p("m1", Difficulty::Medium, false),
            p("h1", Difficulty::Hard, false),
        ];
        let touched: HashSet<String> = ["e1".to_string()].into();
        for seed in 0..50 {
            let [a, b] =
                pick_problems(&all, &touched, [Difficulty::Easy, Difficulty::Medium], seed)
                    .unwrap();
            assert_eq!((a.id.as_str(), b.id.as_str()), ("e3", "m1"));
        }
        // Two easy slots, one fresh easy problem: the second falls back to medium.
        let [a, b] =
            pick_problems(&all, &touched, [Difficulty::Easy, Difficulty::Easy], 7).unwrap();
        assert_eq!((a.id.as_str(), b.id.as_str()), ("e3", "m1"));
        // Hard + hard: h1, then the nearest (medium).
        let [a, b] =
            pick_problems(&all, &touched, [Difficulty::Hard, Difficulty::Hard], 7).unwrap();
        assert_eq!((a.id.as_str(), b.id.as_str()), ("h1", "m1"));
        // Fewer than two fresh problems.
        let touched: HashSet<String> = ["e1", "e3", "m1"].iter().map(|s| s.to_string()).collect();
        assert!(pick_problems(&all, &touched, [Difficulty::Easy, Difficulty::Easy], 1).is_none());
    }

    #[test]
    fn picks_vary_with_the_seed() {
        let all: Vec<ProblemSummary> = (0..20)
            .map(|i| p(&format!("m{i:02}"), Difficulty::Medium, false))
            .collect();
        let none = HashSet::new();
        let firsts: HashSet<String> = (0..40)
            .map(|s| {
                pick_problems(&all, &none, [Difficulty::Medium, Difficulty::Medium], s).unwrap()[0]
                    .id
                    .clone()
            })
            .collect();
        assert!(firsts.len() > 3, "{firsts:?}");
        for s in 0..40 {
            let [a, b] =
                pick_problems(&all, &none, [Difficulty::Medium, Difficulty::Medium], s).unwrap();
            assert_ne!(a.id, b.id);
        }
    }

    #[test]
    fn scorecard_counts_submits_time_and_first_try() {
        let s = SessionRow {
            id: 1,
            started_at: 10_000,
            duration_s: 1800,
            problem_ids: vec!["a".into(), "b".into()],
            languages: vec![Language::Python],
            mix: "easy+medium".into(),
            ended_at: None,
            results: None,
        };
        let f = |p: &str, v: Verdict, t: i64| SubmitFact {
            problem_id: p.into(),
            verdict: v,
            created_at: t,
        };
        let facts = vec![
            f("a", Verdict::Accepted, 70_000),
            f("b", Verdict::WrongAnswer, 100_000),
            f("b", Verdict::TimeLimitExceeded, 200_000),
            f("b", Verdict::Accepted, 610_000),
            f("a", Verdict::WrongAnswer, 700_000),
        ];
        let r = score(&s, 900_000, &facts);
        assert_eq!(r.solved, 2);
        assert_eq!(r.first_try, 1);
        assert_eq!(r.used_s, 890);
        assert_eq!(
            r.problems[0],
            ProblemScore {
                problem_id: "a".into(),
                solved: true,
                time_to_solve_s: Some(60),
                submits: 2,
                first_try: true
            }
        );
        assert_eq!(r.problems[1].time_to_solve_s, Some(600));
        assert_eq!(r.problems[1].submits, 3);
        assert!(!r.problems[1].first_try);
        // Ending past the deadline counts the whole duration only.
        assert_eq!(score(&s, 99_000_000, &[]).used_s, 1800);
    }
}
