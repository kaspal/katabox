//! Kata drills (expansion M1, ADR 0033): re-solve solved problems from memory against
//! the clock. The editor starts from the starter code; attempts are recorded apart from
//! ordinary submissions (no submissions row, status, review, daily or draft change).
//!
//! The rules here are pure and unit tested: the rusty score, the daily kata pick,
//! personal bests and the kata streak.

use std::collections::{HashMap, HashSet};

use kb_core::{Day, Difficulty, Language, Verdict};
use kb_store::kata::{KataAttemptRow, KataKind, KataSessionRow};
use kb_store::{ProblemStatus, ProblemSummary, UiLang};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::practice::parse_day;
use crate::{App, AppError, AppErrorCode, Result};

pub use kb_store::kata::KATA_MAX_MS;
/// Problems in the daily kata.
pub const DAILY_KATA_SIZE: usize = 3;
/// The daily kata's soft time limit.
pub const DAILY_KATA_S: i64 = 15 * 60;
/// A pattern, list or custom kata: this many problems at most, this long each.
pub const SET_KATA_MAX: usize = 5;
pub const SET_KATA_PER_PROBLEM_S: i64 = 10 * 60;
/// Days for the rusty score to reach 1 − 1/e (about 0.63) without practice.
pub const RUSTY_DAYS: f64 = 30.0;
/// Added to the rusty score for each of the last three kata attempts that failed.
pub const RUSTY_FAILURE: f64 = 0.15;

const DAY_MS: f64 = 86_400_000.0;

/// How rusty a solved problem is, in [0, 1]: `1 − exp(−days / 30)` since its last
/// success (an Accepted submission or a solved kata attempt), plus 0.15 for each failed
/// kata attempt among its last three, capped at 1. Never practised: 1.
pub fn rusty_score(now_ms: i64, last_success_ms: Option<i64>, recent_failures: u32) -> f64 {
    let decay = match last_success_ms {
        Some(t) => {
            let days = ((now_ms - t).max(0) as f64) / DAY_MS;
            1.0 - (-days / RUSTY_DAYS).exp()
        }
        None => 1.0,
    };
    (decay + RUSTY_FAILURE * f64::from(recent_failures.min(3))).min(1.0)
}

/// The rusty score of each problem in `ids`. `running` is the running session (its
/// unfinished attempts are not failures yet).
pub fn rusty_scores(
    ids: &[&str],
    last_accepts: &HashMap<String, i64>,
    attempts: &[KataAttemptRow],
    running: Option<i64>,
    now_ms: i64,
) -> HashMap<String, f64> {
    let mut by_problem: HashMap<&str, Vec<&KataAttemptRow>> = HashMap::new();
    for a in attempts {
        by_problem.entry(a.problem_id.as_str()).or_default().push(a);
    }
    ids.iter()
        .map(|id| {
            let mine = by_problem.get(id).map(Vec::as_slice).unwrap_or(&[]);
            let kata_success = mine.iter().filter_map(|a| a.solved_at).max();
            let last = [last_accepts.get(*id).copied(), kata_success]
                .into_iter()
                .flatten()
                .max();
            let mut finished: Vec<&&KataAttemptRow> = mine
                .iter()
                .filter(|a| Some(a.session_started_at) != running)
                .collect();
            finished.sort_by_key(|a| std::cmp::Reverse(a.started_at));
            let failures = finished
                .iter()
                .take(3)
                .filter(|a| a.solved_at.is_none())
                .count();
            (
                (*id).to_string(),
                rusty_score(now_ms, last, u32::try_from(failures).unwrap_or(3)),
            )
        })
        .collect()
}

/// The daily kata: up to [`DAILY_KATA_SIZE`] distinct solved problems, a weighted
/// choice without replacement (weight = rusty score + 0.05), seeded by the day and the
/// install id, so it is the same all day on this machine.
pub fn daily_kata_pick(
    solved: &[&ProblemSummary],
    rusty: &HashMap<String, f64>,
    day: Day,
    install_id: &str,
) -> Vec<String> {
    let mut pool: Vec<(&str, u64)> = solved
        .iter()
        .map(|p| {
            let score = rusty.get(&p.id).copied().unwrap_or(1.0);
            // Integer weights keep the pick exact across platforms.
            (p.id.as_str(), ((score + 0.05) * 1000.0).round() as u64)
        })
        .collect();
    pool.sort_by(|a, b| a.0.cmp(b.0));
    let mut out = Vec::new();
    for k in 0..DAILY_KATA_SIZE {
        let total: u64 = pool.iter().map(|(_, w)| w).sum();
        if total == 0 {
            break;
        }
        let digest = Sha256::digest(format!("katabox-kata|{day}|{install_id}|{k}").as_bytes());
        let mut r = u64::from_le_bytes(digest[..8].try_into().unwrap_or([0; 8])) % total;
        let mut chosen = pool.len() - 1;
        for (i, (_, w)) in pool.iter().enumerate() {
            if r < *w {
                chosen = i;
                break;
            }
            r -= w;
        }
        out.push(pool.remove(chosen).0.to_string());
    }
    out
}

/// `ids` ordered rustiest first (ties by id), at most `n`.
pub fn rustiest(ids: &[&str], rusty: &HashMap<String, f64>, n: usize) -> Vec<String> {
    let mut v: Vec<(&str, f64)> = ids
        .iter()
        .map(|id| (*id, rusty.get(*id).copied().unwrap_or(1.0)))
        .collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    v.into_iter()
        .take(n)
        .map(|(id, _)| id.to_string())
        .collect()
}

/// A problem's personal bests over all kata attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataBest {
    pub problem_id: String,
    pub title: String,
    pub difficulty: Difficulty,
    /// The fastest time from opening to Accepted.
    pub best_time_ms: Option<i64>,
    /// The fewest submits needed to reach Accepted.
    pub fewest_submits: Option<u32>,
    /// Kata attempts on this problem (solved or not).
    pub attempts: u32,
    pub solved: u32,
}

/// Bests per problem from every attempt (titles filled by the caller).
pub fn bests(attempts: &[KataAttemptRow]) -> HashMap<String, (Option<i64>, Option<u32>, u32, u32)> {
    let mut out: HashMap<String, (Option<i64>, Option<u32>, u32, u32)> = HashMap::new();
    for a in attempts {
        let e = out
            .entry(a.problem_id.clone())
            .or_insert((None, None, 0, 0));
        e.2 += 1;
        if a.solved_at.is_some() {
            e.3 += 1;
            if let Some(t) = a.time_ms {
                e.0 = Some(e.0.map_or(t, |b: i64| b.min(t)));
            }
            e.1 = Some(e.1.map_or(a.submits, |b: u32| b.min(a.submits)));
        }
    }
    out
}

/// Consecutive local days with at least one solved kata attempt, ending today (or
/// yesterday when today has none yet).
pub fn kata_streak(sessions: &[KataSessionRow], attempts: &[KataAttemptRow], today: Day) -> u32 {
    let solved: HashSet<i64> = attempts
        .iter()
        .filter(|a| a.solved_at.is_some())
        .map(|a| a.session_started_at)
        .collect();
    let days: Vec<Day> = sessions
        .iter()
        .filter(|s| solved.contains(&s.started_at))
        .filter_map(|s| s.day.parse().ok())
        .collect();
    crate::practice::streak(&days, today)
}

// --- Views --------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataProblem {
    pub id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub pattern: String,
    pub attempt: Option<KataAttemptRow>,
    pub best_time_ms: Option<i64>,
}

/// A session as the UI shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataView {
    pub started_at: i64,
    pub day: String,
    pub kind: KataKind,
    pub source: Option<String>,
    pub duration_s: i64,
    pub ended_at: Option<i64>,
    /// When the session stops counting as running (UTC ms).
    pub expires_at: i64,
    pub problems: Vec<KataProblem>,
    pub solved: u32,
}

/// A pattern or list a kata can be started from, with its solved problems.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataSource {
    /// The pattern id, or the list id as text.
    pub id: String,
    /// The list name (patterns are named by the UI).
    pub name: Option<String>,
    pub solved: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataOptions {
    /// Today's daily kata (empty before the first solve).
    pub daily: Vec<ProblemSummary>,
    /// Today's daily kata session, once started.
    pub daily_session: Option<KataView>,
    pub patterns: Vec<KataSource>,
    pub lists: Vec<KataSource>,
    /// The running session, if any.
    pub running: Option<KataView>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RustyProblem {
    pub problem_id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub pattern: String,
    /// 0-100 (the rusty score as a percentage).
    pub rusty: u32,
    /// Days since the last success (`None`: never practised since solving).
    pub days: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataSessionSummary {
    pub started_at: i64,
    pub day: String,
    pub kind: KataKind,
    pub source: Option<String>,
    pub total: u32,
    pub solved: u32,
    pub ended: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataOverview {
    pub bests: Vec<KataBest>,
    /// The rustiest solved problems (score ≥ 50), rustiest first, at most 10.
    pub rusty: Vec<RustyProblem>,
    pub streak: u32,
    /// The latest sessions, newest first (at most 30).
    pub sessions: Vec<KataSessionSummary>,
}

/// What a kata Submit did.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataOutcome {
    pub attempt: KataAttemptRow,
    /// This Submit solved the problem faster than every earlier kata.
    pub new_best: bool,
    /// The best time before this Submit, if any.
    pub previous_best_ms: Option<i64>,
}

/// What to start a kata from.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct KataStart {
    pub kind: KataKind,
    /// The pattern id or the list id (as text), for those kinds.
    pub source: Option<String>,
    /// The problems of a custom kata.
    pub problems: Vec<String>,
    /// The user's local date, `YYYY-MM-DD`.
    pub today: String,
}

/// Hints, the editorial, submissions, History and notes wait until the kata ends.
pub(crate) fn locked() -> AppError {
    AppError::new(AppErrorCode::LockedInKata, "locked during the kata")
}

fn bad(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, m.into())
}

fn is_solved(p: &ProblemSummary) -> bool {
    p.status == Some(ProblemStatus::Solved)
}

impl App {
    /// The running kata session, ending it first once it expired.
    pub(crate) async fn running_kata(&self) -> Result<Option<KataSessionRow>> {
        let Some(s) = self.store.user.open_kata().await? else {
            return Ok(None);
        };
        let expires = s.started_at + KATA_MAX_MS;
        if self.now_ms() >= expires {
            self.store.user.end_kata(s.started_at, expires).await?;
            return Ok(None);
        }
        Ok(Some(s))
    }

    /// Whether `problem` is part of the running kata.
    pub(crate) async fn in_kata(&self, problem: &str) -> Result<bool> {
        Ok(self
            .running_kata()
            .await?
            .is_some_and(|s| s.problem_ids.iter().any(|p| p == problem)))
    }

    async fn rusty_of(&self, ids: &[&str], running: Option<i64>) -> Result<HashMap<String, f64>> {
        let last = self.store.user.last_accepts().await?;
        let attempts = self.store.user.kata_attempts().await?;
        Ok(rusty_scores(ids, &last, &attempts, running, self.now_ms()))
    }

    async fn kata_view(&self, s: &KataSessionRow, lang: UiLang) -> Result<KataView> {
        let all = self.all_problems(lang).await?;
        let attempts = self.store.user.kata_attempts().await?;
        let best = bests(&attempts);
        let problems: Vec<KataProblem> = s
            .problem_ids
            .iter()
            .filter_map(|id| all.iter().find(|p| &p.id == id))
            .map(|p| KataProblem {
                id: p.id.clone(),
                title: p.title.clone(),
                difficulty: p.difficulty,
                pattern: p.pattern.clone(),
                attempt: attempts
                    .iter()
                    .find(|a| a.session_started_at == s.started_at && a.problem_id == p.id)
                    .cloned(),
                best_time_ms: best.get(&p.id).and_then(|b| b.0),
            })
            .collect();
        let solved = problems
            .iter()
            .filter(|p| p.attempt.as_ref().is_some_and(|a| a.solved_at.is_some()))
            .count();
        Ok(KataView {
            started_at: s.started_at,
            day: s.day.clone(),
            kind: s.kind.clone(),
            source: s.source.clone(),
            duration_s: s.duration_s,
            ended_at: s.ended_at,
            expires_at: s.started_at + KATA_MAX_MS,
            problems,
            solved: u32::try_from(solved).unwrap_or(u32::MAX),
        })
    }

    /// Today's daily kata, the patterns and lists a kata can start from, and the
    /// running session.
    pub async fn kata_options(&self, today: &str, lang: UiLang) -> Result<KataOptions> {
        let day = parse_day(today)?;
        let all = self.all_problems(lang).await?;
        let solved: Vec<&ProblemSummary> = all.iter().filter(|p| is_solved(p)).collect();
        let running = self.running_kata().await?;
        let ids: Vec<&str> = solved.iter().map(|p| p.id.as_str()).collect();
        let rusty = self
            .rusty_of(&ids, running.as_ref().map(|s| s.started_at))
            .await?;
        let install = self.store.user.install_id().await?;
        let daily_ids = daily_kata_pick(&solved, &rusty, day, &install);
        let daily = daily_ids
            .iter()
            .filter_map(|id| all.iter().find(|p| &p.id == id).cloned())
            .collect();
        let sessions = self.store.user.katas().await?;
        let daily_session = match sessions
            .iter()
            .find(|s| s.kind == KataKind::Daily && s.day == day.to_string())
        {
            Some(s) => Some(self.kata_view(s, lang).await?),
            None => None,
        };
        let mut by_pattern: Vec<KataSource> = Vec::new();
        for p in &solved {
            match by_pattern.iter_mut().find(|s| s.id == p.pattern) {
                Some(s) => s.solved += 1,
                None => by_pattern.push(KataSource {
                    id: p.pattern.clone(),
                    name: None,
                    solved: 1,
                }),
            }
        }
        let order = |id: &str| {
            crate::practice::PATTERN_TRACK
                .iter()
                .position(|p| *p == id)
                .unwrap_or(usize::MAX)
        };
        by_pattern.sort_by_key(|s| order(&s.id));
        let mut lists = Vec::new();
        for l in self.store.user.lists().await? {
            let items = self.store.user.list_items(l.id).await?;
            let n = items
                .iter()
                .filter(|id| solved.iter().any(|p| &p.id == *id))
                .count();
            if n > 0 {
                lists.push(KataSource {
                    id: l.id.to_string(),
                    name: Some(l.name),
                    solved: u32::try_from(n).unwrap_or(u32::MAX),
                });
            }
        }
        let running = match running {
            Some(s) => Some(self.kata_view(&s, lang).await?),
            None => None,
        };
        Ok(KataOptions {
            daily,
            daily_session,
            patterns: by_pattern,
            lists,
            running,
        })
    }

    /// Start a kata. Refused while another kata or a mock interview runs. Only solved
    /// problems take part.
    pub async fn start_kata(&self, start: &KataStart, lang: UiLang) -> Result<KataView> {
        let day = parse_day(&start.today)?;
        if self.running_kata().await?.is_some() {
            return Err(AppError::new(
                AppErrorCode::KataActive,
                "a kata is already running",
            ));
        }
        if self.running_session().await?.is_some() {
            return Err(AppError::new(
                AppErrorCode::InterviewActive,
                "a mock interview is running",
            ));
        }
        let all = self.all_problems(lang).await?;
        let solved: Vec<&ProblemSummary> = all.iter().filter(|p| is_solved(p)).collect();
        let ids: Vec<&str> = solved.iter().map(|p| p.id.as_str()).collect();
        let rusty = self.rusty_of(&ids, None).await?;
        let (problems, duration_s, source) = match start.kind {
            KataKind::Daily => {
                if self
                    .store
                    .user
                    .katas()
                    .await?
                    .iter()
                    .any(|s| s.kind == KataKind::Daily && s.day == day.to_string())
                {
                    return Err(bad("today's daily kata was already started"));
                }
                let install = self.store.user.install_id().await?;
                (
                    daily_kata_pick(&solved, &rusty, day, &install),
                    DAILY_KATA_S,
                    None,
                )
            }
            KataKind::Pattern => {
                let pattern = start.source.clone().ok_or_else(|| bad("pick a pattern"))?;
                let in_pattern: Vec<&str> = solved
                    .iter()
                    .filter(|p| p.pattern == pattern)
                    .map(|p| p.id.as_str())
                    .collect();
                let picked = rustiest(&in_pattern, &rusty, SET_KATA_MAX);
                let d = SET_KATA_PER_PROBLEM_S * picked.len() as i64;
                (picked, d, Some(pattern))
            }
            KataKind::List => {
                let id: i64 = start
                    .source
                    .as_deref()
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(|| bad("pick a list"))?;
                let name = self
                    .store
                    .user
                    .lists()
                    .await?
                    .into_iter()
                    .find(|l| l.id == id)
                    .ok_or_else(|| AppError::new(AppErrorCode::NotFound, "no such list"))?
                    .name;
                let items = self.store.user.list_items(id).await?;
                let in_list: Vec<&str> = items
                    .iter()
                    .filter(|i| ids.contains(&i.as_str()))
                    .map(String::as_str)
                    .collect();
                let picked = rustiest(&in_list, &rusty, SET_KATA_MAX);
                let d = SET_KATA_PER_PROBLEM_S * picked.len() as i64;
                (picked, d, Some(name))
            }
            KataKind::Custom => {
                let mut picked: Vec<String> = Vec::new();
                for p in &start.problems {
                    if !ids.contains(&p.as_str()) {
                        return Err(bad(format!("{p} is not solved yet")));
                    }
                    if !picked.contains(p) {
                        picked.push(p.clone());
                    }
                }
                picked.truncate(SET_KATA_MAX);
                let d = SET_KATA_PER_PROBLEM_S * picked.len() as i64;
                (picked, d, None)
            }
        };
        if problems.is_empty() {
            return Err(bad(
                "solve a problem first: a kata re-solves solved problems",
            ));
        }
        // Started-at is the session key: never reuse one.
        let mut started_at = self.now_ms();
        while self.store.user.kata(started_at).await.is_ok() {
            started_at += 1;
        }
        let row = KataSessionRow {
            started_at,
            day: day.to_string(),
            kind: start.kind.clone(),
            source,
            problem_ids: problems,
            duration_s,
            ended_at: None,
        };
        self.store.user.create_kata(&row).await?;
        self.kata_view(&row, lang).await
    }

    /// The running kata, if any.
    pub async fn kata_state(&self, lang: UiLang) -> Result<Option<KataView>> {
        match self.running_kata().await? {
            Some(s) => Ok(Some(self.kata_view(&s, lang).await?)),
            None => Ok(None),
        }
    }

    /// The workspace opened `problem` in the running kata: its clock starts now (once).
    pub async fn kata_open(&self, problem: &str) -> Result<KataAttemptRow> {
        let s = self
            .running_kata()
            .await?
            .filter(|s| s.problem_ids.iter().any(|p| p == problem))
            .ok_or_else(|| bad(format!("{problem} is not in the running kata")))?;
        Ok(self
            .store
            .user
            .open_kata_attempt(s.started_at, problem, self.now_ms())
            .await?)
    }

    /// End the running kata.
    pub async fn end_kata(&self, lang: UiLang) -> Result<Option<KataView>> {
        let Some(s) = self.running_kata().await? else {
            return Ok(None);
        };
        self.store
            .user
            .end_kata(s.started_at, self.now_ms())
            .await?;
        let s = self.store.user.kata(s.started_at).await?;
        Ok(Some(self.kata_view(&s, lang).await?))
    }

    /// Personal bests, the rusty list, the kata streak and the latest sessions.
    pub async fn kata_overview(&self, today: &str, lang: UiLang) -> Result<KataOverview> {
        let day = parse_day(today)?;
        let all = self.all_problems(lang).await?;
        let running = self.running_kata().await?.map(|s| s.started_at);
        let attempts = self.store.user.kata_attempts().await?;
        let sessions = self.store.user.katas().await?;
        let best = bests(&attempts);
        let mut bests_v: Vec<KataBest> = best
            .iter()
            .filter_map(|(id, b)| {
                let p = all.iter().find(|p| &p.id == id)?;
                Some(KataBest {
                    problem_id: id.clone(),
                    title: p.title.clone(),
                    difficulty: p.difficulty,
                    best_time_ms: b.0,
                    fewest_submits: b.1,
                    attempts: b.2,
                    solved: b.3,
                })
            })
            .collect();
        bests_v.sort_by(|a, b| a.title.cmp(&b.title));
        let solved: Vec<&ProblemSummary> = all.iter().filter(|p| is_solved(p)).collect();
        let ids: Vec<&str> = solved.iter().map(|p| p.id.as_str()).collect();
        let last = self.store.user.last_accepts().await?;
        let rusty = rusty_scores(&ids, &last, &attempts, running, self.now_ms());
        let now = self.now_ms();
        let mut rusty_v: Vec<RustyProblem> = solved
            .iter()
            .filter_map(|p| {
                let score = *rusty.get(&p.id)?;
                (score >= 0.5).then(|| {
                    let kata_last = attempts
                        .iter()
                        .filter(|a| a.problem_id == p.id)
                        .filter_map(|a| a.solved_at)
                        .max();
                    let last_success = [last.get(&p.id).copied(), kata_last]
                        .into_iter()
                        .flatten()
                        .max();
                    RustyProblem {
                        problem_id: p.id.clone(),
                        title: p.title.clone(),
                        difficulty: p.difficulty,
                        pattern: p.pattern.clone(),
                        rusty: (score * 100.0).round() as u32,
                        days: last_success.map(|t| {
                            u32::try_from((now - t).max(0) / 86_400_000).unwrap_or(u32::MAX)
                        }),
                    }
                })
            })
            .collect();
        rusty_v.sort_by(|a, b| {
            b.rusty
                .cmp(&a.rusty)
                .then_with(|| a.problem_id.cmp(&b.problem_id))
        });
        rusty_v.truncate(10);
        let sessions_v = sessions
            .iter()
            .take(30)
            .map(|s| {
                let mine: Vec<&KataAttemptRow> = attempts
                    .iter()
                    .filter(|a| a.session_started_at == s.started_at)
                    .collect();
                KataSessionSummary {
                    started_at: s.started_at,
                    day: s.day.clone(),
                    kind: s.kind.clone(),
                    source: s.source.clone(),
                    total: u32::try_from(s.problem_ids.len()).unwrap_or(u32::MAX),
                    solved: u32::try_from(mine.iter().filter(|a| a.solved_at.is_some()).count())
                        .unwrap_or(u32::MAX),
                    ended: s.ended_at.is_some() || Some(s.started_at) != running,
                }
            })
            .collect();
        Ok(KataOverview {
            bests: bests_v,
            rusty: rusty_v,
            streak: kata_streak(&sessions, &attempts, day),
            sessions: sessions_v,
        })
    }

    /// Record a kata Submit (the judging is done by the caller): the attempt, and
    /// whether it beat the earlier best time.
    pub(crate) async fn record_kata(
        &self,
        session: i64,
        problem: &str,
        language: Language,
        code: &str,
        verdict: Verdict,
    ) -> Result<KataOutcome> {
        let before = self.store.user.kata_attempts().await?;
        let previous_best_ms = before
            .iter()
            .filter(|a| a.problem_id == problem && a.session_started_at != session)
            .filter_map(|a| a.time_ms)
            .min();
        let was_solved = before.iter().any(|a| {
            a.problem_id == problem && a.session_started_at == session && a.solved_at.is_some()
        });
        let attempt = self
            .store
            .user
            .record_kata_submit(session, problem, language, code, verdict, self.now_ms())
            .await?;
        self.forget_ratings();
        let new_best = !was_solved
            && attempt
                .time_ms
                .is_some_and(|t| previous_best_ms.is_none_or(|b| t < b));
        Ok(KataOutcome {
            attempt,
            new_best,
            previous_best_ms,
        })
    }

    /// The running kata `session`, which must hold `problem`.
    pub(crate) async fn kata_for(&self, session: i64, problem: &str) -> Result<KataSessionRow> {
        self.running_kata()
            .await?
            .filter(|s| s.started_at == session && s.problem_ids.iter().any(|p| p == problem))
            .ok_or_else(|| bad(format!("{problem} is not in the running kata")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::ProblemMode;

    fn prob(id: &str, pattern: &str) -> ProblemSummary {
        ProblemSummary {
            id: id.into(),
            title: id.into(),
            difficulty: Difficulty::Easy,
            pattern: pattern.into(),
            rank: 1,
            mode: ProblemMode::Function,
            tags: vec![],
            status: Some(ProblemStatus::Solved),
            fix: false,
            pack: None,
        }
    }

    fn attempt(session: i64, problem: &str, started: i64, solved: Option<i64>) -> KataAttemptRow {
        KataAttemptRow {
            session_started_at: session,
            problem_id: problem.into(),
            started_at: started,
            submits: 1,
            verdict: None,
            solved_at: solved,
            time_ms: solved.map(|s| s - started),
            language: None,
            code: None,
        }
    }

    const DAY: i64 = 86_400_000;

    #[test]
    fn rusty_score_decays_and_counts_failures() {
        let now = 100 * DAY;
        assert_eq!(rusty_score(now, Some(now), 0), 0.0);
        let month = rusty_score(now, Some(now - 30 * DAY), 0);
        assert!((month - (1.0 - (-1.0f64).exp())).abs() < 1e-9, "{month}");
        assert!(rusty_score(now, Some(now - 365 * DAY), 0) > 0.99);
        assert_eq!(rusty_score(now, None, 0), 1.0);
        assert!((rusty_score(now, Some(now), 2) - 0.30).abs() < 1e-9);
        // At most three failures count, and the score is capped at 1.
        assert!((rusty_score(now, Some(now), 9) - 0.45).abs() < 1e-9);
        assert_eq!(rusty_score(now, Some(now - 365 * DAY), 3), 1.0);
        // A clock behind the last success does not go negative.
        assert_eq!(rusty_score(now, Some(now + DAY), 0), 0.0);
    }

    #[test]
    fn rusty_scores_use_the_latest_success_and_finished_failures() {
        let now = 100 * DAY;
        let last: HashMap<String, i64> = [
            ("a".to_string(), now - 60 * DAY),
            ("b".to_string(), now - 15 * DAY),
        ]
        .into();
        let attempts = vec![
            // a: a kata solve 1 day ago is the latest success.
            attempt(1, "a", now - DAY - 1000, Some(now - DAY)),
            // b: two failed katas, one in the running session (not counted).
            attempt(2, "b", now - 2 * DAY, None),
            attempt(9, "b", now - 1000, None),
        ];
        let s = rusty_scores(&["a", "b", "c"], &last, &attempts, Some(9), now);
        assert!(s["a"] < 0.04, "{}", s["a"]);
        let b = 1.0 - (-0.5f64).exp() + 0.15;
        assert!((s["b"] - b).abs() < 1e-9, "{}", s["b"]);
        assert_eq!(s["c"], 1.0, "never solved in a submission or kata");
    }

    #[test]
    fn daily_kata_is_deterministic_distinct_and_leans_rusty() {
        let ps: Vec<ProblemSummary> = (0..10).map(|i| prob(&format!("p{i}"), "x")).collect();
        let refs: Vec<&ProblemSummary> = ps.iter().collect();
        let mut rusty: HashMap<String, f64> = ps.iter().map(|p| (p.id.clone(), 0.0)).collect();
        rusty.insert("p7".into(), 1.0);
        let day: Day = "2026-11-21".parse().unwrap();
        let a = daily_kata_pick(&refs, &rusty, day, "install");
        assert_eq!(a, daily_kata_pick(&refs, &rusty, day, "install"));
        assert_eq!(a.len(), 3);
        assert_eq!(a.iter().collect::<HashSet<_>>().len(), 3);
        // p7 weighs 1050 against 50 for each other problem: it is picked on most days.
        let hits = (0..60)
            .filter(|d| {
                daily_kata_pick(&refs, &rusty, day.add_days(*d), "install")
                    .contains(&"p7".to_string())
            })
            .count();
        assert!(hits > 50, "{hits}");
        // Fewer solved problems than the kata size: all of them.
        assert_eq!(daily_kata_pick(&refs[..2], &rusty, day, "i").len(), 2);
        assert!(daily_kata_pick(&[], &rusty, day, "i").is_empty());
    }

    #[test]
    fn rustiest_orders_by_score_then_id() {
        let rusty: HashMap<String, f64> =
            [("a".into(), 0.2), ("b".into(), 0.9), ("c".into(), 0.9)].into();
        assert_eq!(rustiest(&["a", "b", "c", "d"], &rusty, 3), ["d", "b", "c"]);
    }

    #[test]
    fn bests_and_streak() {
        let attempts = vec![
            attempt(1, "a", 0, Some(5_000)),
            attempt(2, "a", 10_000, Some(13_000)),
            attempt(3, "a", 20_000, None),
            attempt(3, "b", 20_000, None),
        ];
        let b = bests(&attempts);
        assert_eq!(b["a"], (Some(3_000), Some(1), 3, 2));
        assert_eq!(b["b"], (None, None, 1, 0));

        let session = |at: i64, day: &str| KataSessionRow {
            started_at: at,
            day: day.into(),
            kind: KataKind::Daily,
            source: None,
            problem_ids: vec![],
            duration_s: 900,
            ended_at: Some(at + 1),
        };
        let sessions = vec![
            session(1, "2026-11-19"),
            session(2, "2026-11-20"),
            session(3, "2026-11-21"),
        ];
        let today: Day = "2026-11-21".parse().unwrap();
        // Session 3 solved nothing: the streak ends yesterday.
        assert_eq!(kata_streak(&sessions, &attempts, today), 2);
        assert_eq!(kata_streak(&sessions, &attempts, today.add_days(2)), 0);
    }
}
