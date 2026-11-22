//! Study plans in the app (expansion M2, ADR 0033): make, show, update and export the
//! active plan. The generator and the ICS writer are pure, in `plan.rs`.

use std::collections::HashSet;

use kb_core::{Day, Difficulty};
use kb_store::plans::{PlanItemRow, PlanRow};
use kb_store::{ProblemStatus, ProblemSummary, UiLang};
use serde::{Deserialize, Serialize};

use crate::plan::{
    IcsDay, MINUTES_MAX, MINUTES_MIN, PatternWork, PlanItemKind, WEEKS_MAX, generate, plan_ics,
};
use crate::practice::{PATTERN_TRACK, parse_day};
use crate::{App, AppError, AppErrorCode, Result};

/// What to plan: the patterns (empty: every pattern in track order), how long, how much.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanSetup {
    pub patterns: Vec<String>,
    pub weeks: u32,
    pub minutes_per_day: u32,
    /// The user's local date, `YYYY-MM-DD`: the plan starts today.
    pub today: String,
}

/// One item as the UI shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanItemView {
    pub position: u32,
    pub day: String,
    pub kind: PlanItemKind,
    pub reference: Option<String>,
    /// The problem or topic title in the UI language (none for reviews and katas).
    pub title: Option<String>,
    pub difficulty: Option<Difficulty>,
    /// For topics: the subject (the topic page's route needs it).
    pub subject: Option<String>,
    pub minutes: u32,
    /// `pending`, `done` or `skipped`.
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanView {
    pub created_at: i64,
    pub patterns: Vec<String>,
    pub weeks: u32,
    pub minutes_per_day: u32,
    pub start_day: String,
    pub end_day: String,
    pub items: Vec<PlanItemView>,
    pub done: u32,
    pub total: u32,
    /// Topics and problems that did not fit when the plan was made.
    pub left_over: u32,
}

/// The words the calendar file uses, in the UI language (the UI sends them).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PlanIcsLabels {
    pub title: String,
    pub review: String,
    pub kata: String,
    /// Put before a topic's title, e.g. "Learn: ".
    pub topic: String,
    /// After the minutes, e.g. "min".
    pub minutes: String,
}

fn bad(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, m.into())
}

fn store_err(m: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::Store, m.into())
}

fn goal_patterns(goal: &serde_json::Value) -> Vec<String> {
    goal.get("patterns")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|p| p.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_stored_day(s: &str) -> Result<Day> {
    s.parse().map_err(|e: String| store_err(e))
}

/// `YYYYMMDDTHHMMSSZ` of a UTC millisecond time.
pub(crate) fn ics_stamp(ms: i64) -> String {
    let day = Day::from_days(ms.div_euclid(86_400_000));
    let secs = ms.rem_euclid(86_400_000) / 1000;
    format!(
        "{}T{:02}{:02}{:02}Z",
        day.to_string().replace('-', ""),
        secs / 3600,
        (secs / 60) % 60,
        secs % 60
    )
}

impl App {
    async fn viewed_topics(&self) -> Result<HashSet<String>> {
        Ok(self
            .store
            .user
            .get_setting("learn.viewed")
            .await?
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default()
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect())
    }

    /// Make a plan from `setup`, starting today; it replaces the active plan.
    pub async fn create_plan(&self, setup: &PlanSetup, lang: UiLang) -> Result<PlanView> {
        let start = parse_day(&setup.today)?;
        if !(1..=WEEKS_MAX).contains(&setup.weeks) {
            return Err(bad(format!("a plan runs 1 to {WEEKS_MAX} weeks")));
        }
        if !(MINUTES_MIN..=MINUTES_MAX).contains(&setup.minutes_per_day) {
            return Err(bad(format!(
                "a plan takes {MINUTES_MIN} to {MINUTES_MAX} minutes a day"
            )));
        }
        let all = self.practice_problems(lang).await?;
        let wanted: Vec<&str> = PATTERN_TRACK
            .iter()
            .copied()
            .filter(|p| setup.patterns.is_empty() || setup.patterns.iter().any(|w| w == p))
            .collect();
        if wanted.is_empty() {
            return Err(bad("pick at least one known pattern"));
        }
        let viewed = self.viewed_topics().await?;
        let by_pattern = self.learn_by_pattern(lang).await?;
        let mut seen_topics: HashSet<String> = HashSet::new();
        let mut work = Vec::new();
        for pat in &wanted {
            let topics: Vec<String> = by_pattern
                .iter()
                .find(|t| t.pattern == *pat)
                .map(|t| {
                    t.topics
                        .iter()
                        .filter(|x| !viewed.contains(&x.id) && seen_topics.insert(x.id.clone()))
                        .map(|x| x.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let mut problems: Vec<&ProblemSummary> = all
                .iter()
                .filter(|p| p.pattern == *pat && p.status != Some(ProblemStatus::Solved))
                .collect();
            problems.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.id.cmp(&b.id)));
            work.push(PatternWork {
                pattern: (*pat).to_string(),
                topics,
                problems: problems
                    .iter()
                    .map(|p| (p.id.clone(), p.difficulty))
                    .collect(),
            });
        }
        let g = generate(&work, start, setup.weeks, setup.minutes_per_day);
        let created_at = self.now_ms();
        let plan = PlanRow {
            created_at,
            goal: serde_json::json!({ "patterns": wanted, "left_over": g.left_over }),
            weeks: setup.weeks,
            minutes_per_day: setup.minutes_per_day,
            start_day: start.to_string(),
            archived: false,
        };
        let items: Vec<PlanItemRow> = g
            .items
            .iter()
            .enumerate()
            .map(|(i, it)| PlanItemRow {
                plan_created_at: created_at,
                position: u32::try_from(i).unwrap_or(u32::MAX),
                day: it.day.to_string(),
                kind: it.kind.id().into(),
                reference: it.reference.clone(),
                minutes: it.minutes,
                status: "pending".into(),
                updated_at: created_at,
            })
            .collect();
        self.store.user.create_plan(&plan, &items).await?;
        self.plan_view(lang)
            .await?
            .ok_or_else(|| AppError::new(AppErrorCode::Internal, "the new plan vanished"))
    }

    /// The active plan with titles. Problems solved and topics viewed since are marked
    /// done (and stored so), as are kata items on a day with a solved kata.
    pub async fn plan_view(&self, lang: UiLang) -> Result<Option<PlanView>> {
        let Some(plan) = self.store.user.active_plan().await? else {
            return Ok(None);
        };
        let all = self.all_problems(lang).await?;
        let viewed = self.viewed_topics().await?;
        let topics: Vec<kb_store::LearnTopicSummary> = self
            .store
            .content
            .learn_catalog(lang)
            .await?
            .into_iter()
            .flat_map(|s| s.topics)
            .collect();
        let solved_sessions: HashSet<i64> = self
            .store
            .user
            .kata_attempts()
            .await?
            .iter()
            .filter(|a| a.solved_at.is_some())
            .map(|a| a.session_started_at)
            .collect();
        let kata_days: HashSet<String> = self
            .store
            .user
            .katas()
            .await?
            .into_iter()
            .filter(|s| solved_sessions.contains(&s.started_at))
            .map(|s| s.day)
            .collect();
        let mut items = Vec::new();
        for row in self.store.user.plan_items(plan.created_at).await? {
            let kind = PlanItemKind::parse(&row.kind)
                .ok_or_else(|| store_err(format!("plan item kind {}", row.kind)))?;
            let problem = row
                .reference
                .as_ref()
                .filter(|_| kind == PlanItemKind::Problem)
                .and_then(|id| all.iter().find(|p| &p.id == id));
            let topic = row
                .reference
                .as_ref()
                .filter(|_| kind == PlanItemKind::Topic)
                .and_then(|id| topics.iter().find(|t| &t.id == id));
            let auto_done = row.status == "pending"
                && match kind {
                    PlanItemKind::Problem => {
                        problem.is_some_and(|p| p.status == Some(ProblemStatus::Solved))
                    }
                    PlanItemKind::Topic => {
                        row.reference.as_ref().is_some_and(|t| viewed.contains(t))
                    }
                    PlanItemKind::Kata => kata_days.contains(&row.day),
                    PlanItemKind::Review => false,
                };
            let status = if auto_done {
                self.store
                    .user
                    .set_plan_item_status(plan.created_at, row.position, "done")
                    .await?;
                "done".to_string()
            } else {
                row.status.clone()
            };
            items.push(PlanItemView {
                position: row.position,
                day: row.day.clone(),
                kind,
                reference: row.reference.clone(),
                title: problem
                    .map(|p| p.title.clone())
                    .or_else(|| topic.map(|t| t.title.clone())),
                difficulty: problem.map(|p| p.difficulty),
                subject: topic.map(|t| t.subject.clone()),
                minutes: row.minutes,
                status,
            });
        }
        let start = parse_stored_day(&plan.start_day)?;
        let done = items.iter().filter(|i| i.status == "done").count();
        Ok(Some(PlanView {
            created_at: plan.created_at,
            patterns: goal_patterns(&plan.goal),
            weeks: plan.weeks,
            minutes_per_day: plan.minutes_per_day,
            start_day: plan.start_day.clone(),
            end_day: start.add_days(i64::from(plan.weeks) * 7 - 1).to_string(),
            total: u32::try_from(items.len()).unwrap_or(u32::MAX),
            done: u32::try_from(done).unwrap_or(u32::MAX),
            left_over: plan
                .goal
                .get("left_over")
                .and_then(serde_json::Value::as_u64)
                .and_then(|n| u32::try_from(n).ok())
                .unwrap_or(0),
            items,
        }))
    }

    /// Mark an item done, skipped or pending again.
    pub async fn set_plan_item(&self, plan: i64, position: u32, status: &str) -> Result<()> {
        Ok(self
            .store
            .user
            .set_plan_item_status(plan, position, status)
            .await?)
    }

    /// Move an item one day later, and every pending item after it (same day and a
    /// later position, or a later day) with it, so the order holds.
    pub async fn reschedule_plan_item(&self, plan: i64, position: u32) -> Result<()> {
        let items = self.store.user.plan_items(plan).await?;
        let it = items
            .iter()
            .find(|i| i.position == position)
            .ok_or_else(|| AppError::new(AppErrorCode::NotFound, "no such plan item"))?;
        let key = (it.day.clone(), it.position);
        let mut moves = Vec::new();
        for i in &items {
            let later = (i.day.clone(), i.position) >= key;
            if later && (i.status == "pending" || i.position == position) {
                let d = parse_stored_day(&i.day)?;
                moves.push((i.position, d.add_days(1).to_string()));
            }
        }
        Ok(self.store.user.move_plan_items(plan, &moves).await?)
    }

    pub async fn archive_plan(&self, plan: i64) -> Result<()> {
        Ok(self.store.user.archive_plan(plan).await?)
    }

    /// Write the active plan's calendar to `path` (a temp file, then a rename).
    pub async fn export_plan_ics(
        &self,
        path: &std::path::Path,
        labels: &PlanIcsLabels,
        lang: UiLang,
    ) -> Result<()> {
        let text = self.plan_ics_text(lang, labels).await?;
        let io = |e: std::io::Error| {
            AppError::new(
                AppErrorCode::Internal,
                format!("cannot write the calendar: {e}"),
            )
        };
        let tmp = path.with_extension("ics.tmp");
        std::fs::write(&tmp, text).map_err(io)?;
        std::fs::rename(&tmp, path).map_err(io)?;
        Ok(())
    }

    /// The active plan as an iCalendar file (skipped items left out).
    pub async fn plan_ics_text(&self, lang: UiLang, labels: &PlanIcsLabels) -> Result<String> {
        let view = self
            .plan_view(lang)
            .await?
            .ok_or_else(|| AppError::new(AppErrorCode::NotFound, "no active plan"))?;
        let mut days: Vec<IcsDay> = Vec::new();
        for it in view.items.iter().filter(|i| i.status != "skipped") {
            let day = parse_stored_day(&it.day)?;
            let text = match it.kind {
                PlanItemKind::Problem => it.title.clone().unwrap_or_default(),
                PlanItemKind::Topic => {
                    format!("{}{}", labels.topic, it.title.clone().unwrap_or_default())
                }
                PlanItemKind::Review => labels.review.clone(),
                PlanItemKind::Kata => labels.kata.clone(),
            };
            let line = format!("{text} ({} {})", it.minutes, labels.minutes);
            match days.iter_mut().find(|d| d.day == day) {
                Some(d) => {
                    d.lines.push(line);
                    d.minutes += it.minutes;
                }
                None => days.push(IcsDay {
                    day,
                    lines: vec![line],
                    minutes: it.minutes,
                }),
            }
        }
        Ok(plan_ics(
            &labels.title,
            &format!("katabox-plan-{}", view.created_at),
            &ics_stamp(self.now_ms()),
            &days,
        ))
    }
}
