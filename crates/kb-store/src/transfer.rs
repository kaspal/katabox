//! Progress export and import (ADR 0020): every user.db record a user would want on
//! another machine, as plain data, and a merge that keeps the newer record on conflicts.
//! The file envelope (`katabox-export/v2`, v1 still read), size cap and value checks live in kb-app;
//! this module reads all rows and merges rows inside one transaction.

use std::collections::HashMap;

use kb_core::{Language, Verdict};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Row, SqliteConnection};

use crate::journal::{MistakeRow, ReflectionRow};
use crate::kata::{KATA_MAX_MS, KataAttemptRow, KataSessionRow};
use crate::plans::{PlanItemRow, PlanRow};
use crate::reading_answers::ReadingAnswerRow;
use crate::snippets::SnippetRow;
use crate::stress::StressBruteRow;
use crate::user::{code_hash, parse_language, parse_status, parse_verdict, verdict_code};
use crate::{ProblemStatus, Result, SnapshotSource, UserDb};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusRow {
    pub problem_id: String,
    pub status: ProblemStatus,
    pub first_solved_at: Option<i64>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionRow {
    pub problem_id: String,
    pub language: Language,
    pub code: String,
    pub verdict: Verdict,
    pub time_us: i64,
    pub memory_kb: i64,
    /// The stored judge Report (JSON object).
    pub report: Value,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRow {
    pub problem_id: String,
    pub language: Language,
    pub code: String,
    pub source: SnapshotSource,
    pub verdict: Option<Verdict>,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoteRow {
    pub problem_id: String,
    pub body: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftRow {
    pub problem_id: String,
    pub language: Language,
    pub code: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HintUnlockRow {
    pub problem_id: String,
    pub tier: u32,
    pub unlocked_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenRow {
    pub problem_id: String,
    pub first_opened_at: i64,
    pub last_opened_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewExport {
    pub problem_id: String,
    /// `YYYY-MM-DD`.
    pub due_on: String,
    pub interval_days: u32,
    pub passes: u32,
    pub lapsed: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DailyPickRow {
    /// `YYYY-MM-DD`.
    pub day: String,
    pub problem_id: String,
    pub solved_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListExport {
    pub name: String,
    pub created_at: i64,
    /// Problem ids in list order.
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterviewExport {
    pub started_at: i64,
    pub duration_s: i64,
    pub problem_ids: Vec<String>,
    pub languages: Vec<Language>,
    pub mix: String,
    pub ended_at: Option<i64>,
    pub results: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingRow {
    pub key: String,
    pub value: Value,
    pub updated_at: i64,
}

/// Everything exported, by kind.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportData {
    pub statuses: Vec<StatusRow>,
    pub submissions: Vec<SubmissionRow>,
    pub snapshots: Vec<SnapshotRow>,
    pub notes: Vec<NoteRow>,
    pub drafts: Vec<DraftRow>,
    pub hint_unlocks: Vec<HintUnlockRow>,
    pub opens: Vec<OpenRow>,
    pub reviews: Vec<ReviewExport>,
    pub daily_picks: Vec<DailyPickRow>,
    pub lists: Vec<ListExport>,
    pub interviews: Vec<InterviewExport>,
    pub settings: Vec<SettingRow>,
    // Sections added by `katabox-export/v2` (expansion) go below, each
    // `#[serde(default)]` so v1 files still parse.
    /// Kata sessions (expansion M1).
    #[serde(default)]
    pub kata_sessions: Vec<KataSessionRow>,
    /// Kata attempts (expansion M1).
    #[serde(default)]
    pub kata_attempts: Vec<KataAttemptRow>,
    /// Study plans (expansion M2).
    #[serde(default)]
    pub study_plans: Vec<PlanRow>,
    /// Study plan items (expansion M2).
    #[serde(default)]
    pub plan_items: Vec<PlanItemRow>,
    /// The mistake journal (expansion M3).
    #[serde(default)]
    pub mistakes: Vec<MistakeRow>,
    /// Reflections after Accepted (expansion M3).
    #[serde(default)]
    pub reflections: Vec<ReflectionRow>,
    /// The snippet library (expansion M4).
    #[serde(default)]
    pub snippets: Vec<SnippetRow>,
    /// Answers to reading exercises (expansion M7).
    #[serde(default)]
    pub reading_answers: Vec<ReadingAnswerRow>,
    /// The stress lab's brute-force solutions (expansion M8).
    #[serde(default)]
    pub stress_brutes: Vec<StressBruteRow>,
}

impl ExportData {
    /// True when the data holds none of the sections that only v2 files may have.
    pub fn v2_sections_empty(&self) -> bool {
        self.kata_sessions.is_empty()
            && self.kata_attempts.is_empty()
            && self.study_plans.is_empty()
            && self.plan_items.is_empty()
            && self.mistakes.is_empty()
            && self.reflections.is_empty()
            && self.snippets.is_empty()
            && self.reading_answers.is_empty()
            && self.stress_brutes.is_empty()
    }
}

/// How many records an import adds, updates (newer wins) or keeps as they were.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct MergeStats {
    pub added: u64,
    pub updated: u64,
    pub unchanged: u64,
}

impl MergeStats {
    fn count(&mut self, changed_rows: u64, existed: bool) {
        match (changed_rows > 0, existed) {
            (true, false) => self.added += 1,
            (true, true) => self.updated += 1,
            (false, _) => self.unchanged += 1,
        }
    }
}

impl UserDb {
    /// Every exportable record. `skip_setting` filters settings that must not leave the
    /// machine (decided by kb-app).
    pub async fn export_all(&self, skip_setting: impl Fn(&str) -> bool) -> Result<ExportData> {
        let p = &self.pool;
        let mut d = ExportData::default();
        for r in sqlx::query("SELECT * FROM problem_status ORDER BY problem_id")
            .fetch_all(p)
            .await?
        {
            d.statuses.push(StatusRow {
                problem_id: r.try_get("problem_id")?,
                status: parse_status(&r.try_get::<String, _>("status")?)?,
                first_solved_at: r.try_get("first_solved_at")?,
                updated_at: r.try_get("updated_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM submissions ORDER BY id")
            .fetch_all(p)
            .await?
        {
            d.submissions.push(SubmissionRow {
                problem_id: r.try_get("problem_id")?,
                language: parse_language(&r.try_get::<String, _>("language")?)?,
                code: r.try_get("code")?,
                verdict: parse_verdict(&r.try_get::<String, _>("verdict")?)?,
                time_us: r.try_get("time_us")?,
                memory_kb: r.try_get("memory_kb")?,
                report: serde_json::from_str(&r.try_get::<String, _>("report")?)?,
                created_at: r.try_get("created_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM snapshots ORDER BY id")
            .fetch_all(p)
            .await?
        {
            d.snapshots.push(SnapshotRow {
                problem_id: r.try_get("problem_id")?,
                language: parse_language(&r.try_get::<String, _>("language")?)?,
                code: r.try_get("code")?,
                source: SnapshotSource::parse(&r.try_get::<String, _>("source")?)?,
                verdict: r
                    .try_get::<Option<String>, _>("verdict")?
                    .as_deref()
                    .map(parse_verdict)
                    .transpose()?,
                created_at: r.try_get("created_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM notes ORDER BY problem_id")
            .fetch_all(p)
            .await?
        {
            d.notes.push(NoteRow {
                problem_id: r.try_get("problem_id")?,
                body: r.try_get("body")?,
                updated_at: r.try_get("updated_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM drafts ORDER BY problem_id, language")
            .fetch_all(p)
            .await?
        {
            d.drafts.push(DraftRow {
                problem_id: r.try_get("problem_id")?,
                language: parse_language(&r.try_get::<String, _>("language")?)?,
                code: r.try_get("code")?,
                updated_at: r.try_get("updated_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM hint_unlocks ORDER BY problem_id, tier")
            .fetch_all(p)
            .await?
        {
            d.hint_unlocks.push(HintUnlockRow {
                problem_id: r.try_get("problem_id")?,
                tier: u32::try_from(r.try_get::<i64, _>("tier")?).unwrap_or(0),
                unlocked_at: r.try_get("unlocked_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM problem_opens ORDER BY problem_id")
            .fetch_all(p)
            .await?
        {
            d.opens.push(OpenRow {
                problem_id: r.try_get("problem_id")?,
                first_opened_at: r.try_get("first_opened_at")?,
                last_opened_at: r.try_get("last_opened_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM reviews ORDER BY problem_id")
            .fetch_all(p)
            .await?
        {
            d.reviews.push(ReviewExport {
                problem_id: r.try_get("problem_id")?,
                due_on: r.try_get("due_on")?,
                interval_days: u32::try_from(r.try_get::<i64, _>("interval_days")?).unwrap_or(3),
                passes: u32::try_from(r.try_get::<i64, _>("passes")?).unwrap_or(0),
                lapsed: r.try_get::<i64, _>("lapsed")? != 0,
                created_at: r.try_get("created_at")?,
            });
        }
        for r in sqlx::query("SELECT * FROM daily_picks ORDER BY day")
            .fetch_all(p)
            .await?
        {
            d.daily_picks.push(DailyPickRow {
                day: r.try_get("day")?,
                problem_id: r.try_get("problem_id")?,
                solved_at: r.try_get("solved_at")?,
            });
        }
        for l in self.lists().await? {
            d.lists.push(ListExport {
                items: self.list_items(l.id).await?,
                name: l.name,
                created_at: l.created_at,
            });
        }
        for s in self.sessions().await?.into_iter().rev() {
            d.interviews.push(InterviewExport {
                started_at: s.started_at,
                duration_s: s.duration_s,
                problem_ids: s.problem_ids,
                languages: s.languages,
                mix: s.mix,
                ended_at: s.ended_at,
                results: s.results,
            });
        }
        d.kata_sessions = self.katas().await?.into_iter().rev().collect();
        d.kata_attempts = self.kata_attempts().await?;
        d.study_plans = self.plans().await?.into_iter().rev().collect();
        d.plan_items = self.all_plan_items().await?;
        d.mistakes = self.mistakes().await?;
        d.reflections = self.reflections().await?;
        d.snippets = self.snippets().await?;
        d.reading_answers = self.reading_answers().await?;
        d.stress_brutes = self.stress_brutes().await?;
        for r in sqlx::query("SELECT * FROM settings ORDER BY key")
            .fetch_all(p)
            .await?
        {
            let key: String = r.try_get("key")?;
            if skip_setting(&key) {
                continue;
            }
            d.settings.push(SettingRow {
                value: serde_json::from_str(&r.try_get::<String, _>("value")?)?,
                key,
                updated_at: r.try_get("updated_at")?,
            });
        }
        Ok(d)
    }

    /// How many imported records already exist here (the newer one will be kept).
    pub async fn import_conflicts(&self, d: &ExportData) -> Result<u64> {
        let p = &self.pool;
        let mut n = 0u64;
        for r in &d.statuses {
            n += u64::from(
                has(
                    p,
                    "SELECT 1 FROM problem_status WHERE problem_id = ?",
                    &[&r.problem_id],
                )
                .await?,
            );
        }
        for r in &d.notes {
            n += u64::from(
                has(
                    p,
                    "SELECT 1 FROM notes WHERE problem_id = ?",
                    &[&r.problem_id],
                )
                .await?,
            );
        }
        for r in &d.drafts {
            n += u64::from(
                has(
                    p,
                    "SELECT 1 FROM drafts WHERE problem_id = ? AND language = ?",
                    &[&r.problem_id, r.language.id()],
                )
                .await?,
            );
        }
        for r in &d.reviews {
            n += u64::from(
                has(
                    p,
                    "SELECT 1 FROM reviews WHERE problem_id = ?",
                    &[&r.problem_id],
                )
                .await?,
            );
        }
        for r in &d.daily_picks {
            n += u64::from(has(p, "SELECT 1 FROM daily_picks WHERE day = ?", &[&r.day]).await?);
        }
        for r in &d.lists {
            n += u64::from(has(p, "SELECT 1 FROM lists WHERE name = ?", &[&r.name]).await?);
        }
        for r in &d.settings {
            n += u64::from(has(p, "SELECT 1 FROM settings WHERE key = ?", &[&r.key]).await?);
        }
        for r in &d.kata_attempts {
            let session = r.session_started_at.to_string();
            n += u64::from(
                has(
                    p,
                    "SELECT 1 FROM kata_attempts WHERE session_started_at = ? AND problem_id = ?",
                    &[&session, &r.problem_id],
                )
                .await?,
            );
        }
        Ok(n)
    }

    /// Merge `d` into this database in one transaction; the newer record wins on
    /// conflicts (see ADR 0020 for each kind's rule).
    pub async fn import_merge(&self, d: &ExportData) -> Result<HashMap<&'static str, MergeStats>> {
        let mut tx = self.pool.begin().await?;
        let c: &mut SqliteConnection = &mut tx;
        let mut stats: HashMap<&'static str, MergeStats> = HashMap::new();

        for r in &d.statuses {
            let existed = row_exists(
                c,
                "SELECT 1 FROM problem_status WHERE problem_id = ?",
                &[&r.problem_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO problem_status (problem_id, status, first_solved_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (problem_id) DO UPDATE SET
                   status = CASE WHEN problem_status.status = 'solved' OR excluded.status = 'solved'
                                 THEN 'solved' ELSE 'attempted' END,
                   first_solved_at = CASE
                     WHEN problem_status.first_solved_at IS NULL THEN excluded.first_solved_at
                     WHEN excluded.first_solved_at IS NULL THEN problem_status.first_solved_at
                     ELSE MIN(problem_status.first_solved_at, excluded.first_solved_at) END,
                   updated_at = MAX(problem_status.updated_at, excluded.updated_at)
                 WHERE excluded.updated_at > problem_status.updated_at
                    OR (excluded.status = 'solved' AND problem_status.status <> 'solved')
                    OR (excluded.first_solved_at IS NOT NULL AND (problem_status.first_solved_at IS NULL
                        OR excluded.first_solved_at < problem_status.first_solved_at))",
            )
            .bind(&r.problem_id)
            .bind(match r.status {
                ProblemStatus::Solved => "solved",
                ProblemStatus::Attempted => "attempted",
            })
            .bind(r.first_solved_at)
            .bind(r.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("statuses").or_default().count(n, existed);
        }

        for r in &d.submissions {
            let existed = sqlx::query(
                "SELECT 1 FROM submissions WHERE problem_id = ? AND language = ? AND created_at = ? AND code = ?",
            )
            .bind(&r.problem_id)
            .bind(r.language.id())
            .bind(r.created_at)
            .bind(&r.code)
            .fetch_optional(&mut *c)
            .await?
            .is_some();
            if existed {
                stats.entry("submissions").or_default().count(0, true);
                continue;
            }
            sqlx::query(
                "INSERT INTO submissions (problem_id, language, code, verdict, time_us, memory_kb, report, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&r.problem_id)
            .bind(r.language.id())
            .bind(&r.code)
            .bind(verdict_code(r.verdict)?)
            .bind(r.time_us)
            .bind(r.memory_kb)
            .bind(serde_json::to_string(&r.report)?)
            .bind(r.created_at)
            .execute(&mut *c)
            .await?;
            stats.entry("submissions").or_default().count(1, false);
        }

        for r in &d.snapshots {
            let hash = code_hash(&r.code);
            let existed = row_exists(
                c,
                "SELECT 1 FROM snapshots WHERE problem_id = ? AND language = ? AND code_hash = ?",
                &[&r.problem_id, r.language.id(), &hash],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO snapshots (problem_id, language, code, code_hash, source, verdict, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT (problem_id, language, code_hash) DO UPDATE SET
                   source = excluded.source, verdict = excluded.verdict, created_at = excluded.created_at
                 WHERE excluded.created_at > snapshots.created_at",
            )
            .bind(&r.problem_id)
            .bind(r.language.id())
            .bind(&r.code)
            .bind(&hash)
            .bind(r.source.code())
            .bind(r.verdict.map(verdict_code).transpose()?)
            .bind(r.created_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("snapshots").or_default().count(n, existed);
        }

        for r in &d.notes {
            let existed = row_exists(
                c,
                "SELECT 1 FROM notes WHERE problem_id = ?",
                &[&r.problem_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO notes (problem_id, body, updated_at) VALUES (?, ?, ?)
                 ON CONFLICT (problem_id) DO UPDATE SET body = excluded.body, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > notes.updated_at",
            )
            .bind(&r.problem_id)
            .bind(&r.body)
            .bind(r.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("notes").or_default().count(n, existed);
        }

        for r in &d.drafts {
            let existed = row_exists(
                c,
                "SELECT 1 FROM drafts WHERE problem_id = ? AND language = ?",
                &[&r.problem_id, r.language.id()],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO drafts (problem_id, language, code, updated_at) VALUES (?, ?, ?, ?)
                 ON CONFLICT (problem_id, language) DO UPDATE SET code = excluded.code, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > drafts.updated_at",
            )
            .bind(&r.problem_id)
            .bind(r.language.id())
            .bind(&r.code)
            .bind(r.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("drafts").or_default().count(n, existed);
        }

        for r in &d.hint_unlocks {
            let n = sqlx::query("INSERT OR IGNORE INTO hint_unlocks (problem_id, tier, unlocked_at) VALUES (?, ?, ?)")
                .bind(&r.problem_id)
                .bind(i64::from(r.tier))
                .bind(r.unlocked_at)
                .execute(&mut *c)
                .await?
                .rows_affected();
            stats.entry("hint_unlocks").or_default().count(n, n == 0);
        }

        for r in &d.opens {
            let existed = row_exists(
                c,
                "SELECT 1 FROM problem_opens WHERE problem_id = ?",
                &[&r.problem_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO problem_opens (problem_id, first_opened_at, last_opened_at) VALUES (?, ?, ?)
                 ON CONFLICT (problem_id) DO UPDATE SET
                   first_opened_at = MIN(problem_opens.first_opened_at, excluded.first_opened_at),
                   last_opened_at = MAX(problem_opens.last_opened_at, excluded.last_opened_at)
                 WHERE excluded.first_opened_at < problem_opens.first_opened_at
                    OR excluded.last_opened_at > problem_opens.last_opened_at",
            )
            .bind(&r.problem_id)
            .bind(r.first_opened_at)
            .bind(r.last_opened_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("opens").or_default().count(n, existed);
        }

        for r in &d.reviews {
            let existed = row_exists(
                c,
                "SELECT 1 FROM reviews WHERE problem_id = ?",
                &[&r.problem_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO reviews (problem_id, due_on, interval_days, passes, lapsed, created_at)
                 VALUES (?, ?, ?, ?, ?, ?)
                 ON CONFLICT (problem_id) DO UPDATE SET due_on = excluded.due_on,
                   interval_days = excluded.interval_days, passes = excluded.passes,
                   lapsed = excluded.lapsed, created_at = excluded.created_at
                 WHERE excluded.passes > reviews.passes
                    OR (excluded.passes = reviews.passes AND excluded.due_on > reviews.due_on)",
            )
            .bind(&r.problem_id)
            .bind(&r.due_on)
            .bind(i64::from(r.interval_days))
            .bind(i64::from(r.passes))
            .bind(i64::from(r.lapsed))
            .bind(r.created_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("reviews").or_default().count(n, existed);
        }

        for r in &d.daily_picks {
            let existed =
                row_exists(c, "SELECT 1 FROM daily_picks WHERE day = ?", &[&r.day]).await?;
            let n = sqlx::query(
                "INSERT INTO daily_picks (day, problem_id, solved_at) VALUES (?, ?, ?)
                 ON CONFLICT (day) DO UPDATE SET problem_id = excluded.problem_id, solved_at = excluded.solved_at
                 WHERE daily_picks.solved_at IS NULL AND excluded.solved_at IS NOT NULL",
            )
            .bind(&r.day)
            .bind(&r.problem_id)
            .bind(r.solved_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("daily_picks").or_default().count(n, existed);
        }

        for l in &d.lists {
            let id: Option<i64> =
                sqlx::query_scalar("SELECT id FROM lists WHERE name = ? ORDER BY id LIMIT 1")
                    .bind(&l.name)
                    .fetch_optional(&mut *c)
                    .await?;
            let (id, existed) = match id {
                Some(id) => (id, true),
                None => {
                    let id: i64 = sqlx::query_scalar(
                        "INSERT INTO lists (name, created_at) VALUES (?, ?) RETURNING id",
                    )
                    .bind(&l.name)
                    .bind(l.created_at)
                    .fetch_one(&mut *c)
                    .await?;
                    (id, false)
                }
            };
            let mut added = 0u64;
            for (i, problem) in l.items.iter().enumerate() {
                added += sqlx::query(
                    "INSERT OR IGNORE INTO list_items (list_id, problem_id, position, added_at)
                     VALUES (?1, ?2, (SELECT COALESCE(MAX(position), -1) + 1 FROM list_items WHERE list_id = ?1), ?3)",
                )
                .bind(id)
                .bind(problem)
                .bind(l.created_at + i as i64)
                .execute(&mut *c)
                .await?
                .rows_affected();
            }
            stats
                .entry("lists")
                .or_default()
                .count(u64::from(!existed) + added, existed);
        }

        for s in &d.interviews {
            // An imported session that never ended is imported as ended at its deadline.
            let ended = s.ended_at.or(Some(s.started_at + s.duration_s * 1000));
            let n = sqlx::query(
                "INSERT OR IGNORE INTO interview_sessions
                   (started_at, duration_s, problem_ids, languages, mix, ended_at, results)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(s.started_at)
            .bind(s.duration_s)
            .bind(serde_json::to_string(&s.problem_ids)?)
            .bind(serde_json::to_string(&s.languages)?)
            .bind(&s.mix)
            .bind(ended)
            .bind(s.results.as_ref().map(serde_json::to_string).transpose()?)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("interviews").or_default().count(n, n == 0);
        }

        for s in &d.kata_sessions {
            // A session that never ended is imported as ended when it would have expired.
            let ended = s.ended_at.or(Some(s.started_at + KATA_MAX_MS));
            let n = sqlx::query(
                "INSERT OR IGNORE INTO kata_sessions
                   (started_at, day, kind, source, problem_ids, duration_s, ended_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(s.started_at)
            .bind(&s.day)
            .bind(s.kind.id())
            .bind(&s.source)
            .bind(serde_json::to_string(&s.problem_ids)?)
            .bind(s.duration_s)
            .bind(ended)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("kata_sessions").or_default().count(n, n == 0);
        }

        for a in &d.kata_attempts {
            let session = a.session_started_at.to_string();
            let existed = row_exists(
                c,
                "SELECT 1 FROM kata_attempts WHERE session_started_at = ? AND problem_id = ?",
                &[&session, &a.problem_id],
            )
            .await?;
            // The imported attempt wins when it solved what the local one did not, or
            // counts more submits with the same outcome.
            let n = sqlx::query(
                "INSERT INTO kata_attempts (session_started_at, problem_id, started_at, submits,
                   verdict, solved_at, time_ms, language, code)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT (session_started_at, problem_id) DO UPDATE SET
                   started_at = excluded.started_at, submits = excluded.submits,
                   verdict = excluded.verdict, solved_at = excluded.solved_at,
                   time_ms = excluded.time_ms, language = excluded.language, code = excluded.code
                 WHERE (kata_attempts.solved_at IS NULL AND excluded.solved_at IS NOT NULL)
                    OR (excluded.submits > kata_attempts.submits
                        AND (kata_attempts.solved_at IS NULL) = (excluded.solved_at IS NULL))",
            )
            .bind(a.session_started_at)
            .bind(&a.problem_id)
            .bind(a.started_at)
            .bind(i64::from(a.submits))
            .bind(a.verdict.map(verdict_code).transpose()?)
            .bind(a.solved_at)
            .bind(a.time_ms)
            .bind(a.language.map(|l| l.id()))
            .bind(&a.code)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("kata_attempts").or_default().count(n, existed);
        }

        for p in &d.study_plans {
            let n = sqlx::query(
                "INSERT OR IGNORE INTO study_plans
                   (created_at, goal, weeks, minutes_per_day, start_day, archived)
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(p.created_at)
            .bind(serde_json::to_string(&p.goal)?)
            .bind(i64::from(p.weeks))
            .bind(i64::from(p.minutes_per_day))
            .bind(&p.start_day)
            .bind(p.archived)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("study_plans").or_default().count(n, n == 0);
        }
        // One plan stays active: the newest of the active ones.
        sqlx::query(
            "UPDATE study_plans SET archived = 1 WHERE archived = 0 AND created_at <
               (SELECT max(created_at) FROM study_plans WHERE archived = 0)",
        )
        .execute(&mut *c)
        .await?;
        for i in &d.plan_items {
            let plan = i.plan_created_at.to_string();
            let position = i.position.to_string();
            let existed = row_exists(
                c,
                "SELECT 1 FROM plan_items WHERE plan_created_at = ? AND position = ?",
                &[&plan, &position],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO plan_items (plan_created_at, position, day, kind, ref, minutes, status, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT (plan_created_at, position) DO UPDATE SET
                   day = excluded.day, status = excluded.status, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > plan_items.updated_at",
            )
            .bind(i.plan_created_at)
            .bind(i64::from(i.position))
            .bind(&i.day)
            .bind(&i.kind)
            .bind(&i.reference)
            .bind(i64::from(i.minutes))
            .bind(&i.status)
            .bind(i.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("plan_items").or_default().count(n, existed);
        }

        for m in &d.mistakes {
            let at = m.submitted_at.to_string();
            let existed = row_exists(
                c,
                "SELECT 1 FROM mistakes WHERE problem_id = ? AND language = ? AND submitted_at = ?",
                &[&m.problem_id, &m.language, &at],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO mistakes (problem_id, language, submitted_at, auto_tags, tags, note, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT (problem_id, language, submitted_at) DO UPDATE SET
                   auto_tags = excluded.auto_tags, tags = excluded.tags, note = excluded.note,
                   updated_at = excluded.updated_at
                 WHERE excluded.updated_at > mistakes.updated_at",
            )
            .bind(&m.problem_id)
            .bind(&m.language)
            .bind(m.submitted_at)
            .bind(serde_json::to_string(&m.auto_tags)?)
            .bind(serde_json::to_string(&m.tags)?)
            .bind(&m.note)
            .bind(m.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("mistakes").or_default().count(n, existed);
        }
        for r in &d.reflections {
            let existed = row_exists(
                c,
                "SELECT 1 FROM reflections WHERE problem_id = ?",
                &[&r.problem_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO reflections (problem_id, why, time_class, space_class, updated_at)
                 VALUES (?, ?, ?, ?, ?)
                 ON CONFLICT (problem_id) DO UPDATE SET why = excluded.why,
                   time_class = excluded.time_class, space_class = excluded.space_class,
                   updated_at = excluded.updated_at
                 WHERE excluded.updated_at > reflections.updated_at",
            )
            .bind(&r.problem_id)
            .bind(&r.why)
            .bind(&r.time_class)
            .bind(&r.space_class)
            .bind(r.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("reflections").or_default().count(n, existed);
        }

        for s in &d.snippets {
            let existed = row_exists(c, "SELECT 1 FROM snippets WHERE uid = ?", &[&s.uid]).await?;
            let n = sqlx::query(
                "INSERT INTO snippets (uid, language, name, tags, body, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT (uid) DO UPDATE SET language = excluded.language, name = excluded.name,
                   tags = excluded.tags, body = excluded.body, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > snippets.updated_at",
            )
            .bind(&s.uid)
            .bind(&s.language)
            .bind(&s.name)
            .bind(serde_json::to_string(&s.tags)?)
            .bind(&s.body)
            .bind(s.created_at)
            .bind(s.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("snippets").or_default().count(n, existed);
        }

        for a in &d.reading_answers {
            let existed = row_exists(
                c,
                "SELECT 1 FROM reading_answers WHERE item_id = ?",
                &[&a.item_id],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO reading_answers (item_id, attempts, correct_at, updated_at)
                 VALUES (?, ?, ?, ?)
                 ON CONFLICT (item_id) DO UPDATE SET attempts = excluded.attempts,
                   correct_at = excluded.correct_at, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > reading_answers.updated_at",
            )
            .bind(&a.item_id)
            .bind(a.attempts)
            .bind(a.correct_at)
            .bind(a.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats
                .entry("reading_answers")
                .or_default()
                .count(n, existed);
        }

        for b in &d.stress_brutes {
            let existed = row_exists(
                c,
                "SELECT 1 FROM stress_brutes WHERE problem_id = ? AND language = ?",
                &[&b.problem_id, &b.language],
            )
            .await?;
            let n = sqlx::query(
                "INSERT INTO stress_brutes (problem_id, language, code, updated_at)
                 VALUES (?, ?, ?, ?)
                 ON CONFLICT (problem_id, language) DO UPDATE SET code = excluded.code,
                   updated_at = excluded.updated_at
                 WHERE excluded.updated_at > stress_brutes.updated_at",
            )
            .bind(&b.problem_id)
            .bind(&b.language)
            .bind(&b.code)
            .bind(b.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("stress_brutes").or_default().count(n, existed);
        }

        for r in &d.settings {
            let existed = row_exists(c, "SELECT 1 FROM settings WHERE key = ?", &[&r.key]).await?;
            let n = sqlx::query(
                "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, ?)
                 ON CONFLICT (key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at
                 WHERE excluded.updated_at > settings.updated_at",
            )
            .bind(&r.key)
            .bind(serde_json::to_string(&r.value)?)
            .bind(r.updated_at)
            .execute(&mut *c)
            .await?
            .rows_affected();
            stats.entry("settings").or_default().count(n, existed);
        }

        tx.commit().await?;
        Ok(stats)
    }
}

async fn row_exists(c: &mut SqliteConnection, sql: &str, binds: &[&str]) -> Result<bool> {
    let mut q = sqlx::query(sql);
    for b in binds {
        q = q.bind(*b);
    }
    Ok(q.fetch_optional(&mut *c).await?.is_some())
}

async fn has(p: &sqlx::SqlitePool, sql: &str, binds: &[&str]) -> Result<bool> {
    let mut q = sqlx::query(sql);
    for b in binds {
        q = q.bind(*b);
    }
    Ok(q.fetch_optional(p).await?.is_some())
}
