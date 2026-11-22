//! Classroom mode (expansion M12, ADR 0039). Not cheat-proof: the app is offline and has
//! no accounts, so the results file's hash chain only detects casual edits.
//!
//! Student side: an assignment comes with a pack (`assignment.toml`). Once started it runs
//! until the student finishes it or its time limit passes. While it runs, kb-app (never
//! only the UI) enforces its rules on its problems: only its languages (SQL problems
//! always take SQL) and, with `hints = false`, the same help locks as a mock interview.
//! The results file (`katabox-results/v1`) counts only submissions made while it ran.
//!
//! Teacher side: results files are imported (validated strictly, the chain recomputed;
//! a broken chain is kept but flagged), summarised per student and problem, and exported
//! as CSV with formula injection defused.

use std::collections::BTreeMap;
use std::path::Path;

use kb_core::{Language, Verdict};
use kb_store::classroom::{AssignmentMeta, AssignmentRow, WindowSubmission};
use kb_store::{ProblemSummary, UiLang};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{App, AppError, AppErrorCode, Result};

pub const RESULTS_FORMAT: &str = "katabox-results/v1";
/// The largest results file the teacher side reads.
pub const MAX_RESULTS_BYTES: u64 = 8 * 1024 * 1024;
/// The largest code kept per problem in a results file.
pub const MAX_CODE_BYTES: usize = 64 * 1024;
/// At most this many results files per import.
pub const MAX_RESULTS_FILES: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum AssignmentState {
    NotStarted,
    Running,
    Finished,
}

/// One problem of an assignment, with what happened while it ran.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AssignmentProblem {
    pub id: String,
    /// `None` when the problem is not in this app's content (yet: a pack applies after a
    /// restart).
    pub summary: Option<ProblemSummary>,
    pub best_verdict: Option<Verdict>,
    pub submits: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AssignmentView {
    pub id: String,
    pub meta: AssignmentMeta,
    pub pack_sha256: String,
    pub state: AssignmentState,
    pub imported_at: i64,
    pub started_at: Option<i64>,
    pub submitted_at: Option<i64>,
    /// When the time limit ends it (running with a limit only).
    pub ends_at: Option<i64>,
    pub problems: Vec<AssignmentProblem>,
}

/// One problem in a results file. Its canonical JSON (this field order) is what the hash
/// chain covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct ResultsEntry {
    pub problem_id: String,
    /// The language of the best submission.
    pub language: Option<Language>,
    /// Accepted if any submission was, else the last one's verdict.
    pub best_verdict: Option<Verdict>,
    pub submits: u32,
    /// Seconds from the start to the first Accepted.
    pub first_accepted_s: Option<i64>,
    /// The best submission's code, only when the student chose to include it.
    pub code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct ResultsAssignment {
    pub id: String,
    pub title: String,
    pub pack_sha256: String,
}

/// `katabox-results/v1`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(deny_unknown_fields)]
pub struct ResultsFile {
    pub format: String,
    pub app_version: String,
    pub student: String,
    pub assignment: ResultsAssignment,
    pub started_at: i64,
    pub ended_at: i64,
    pub include_code: bool,
    pub entries: Vec<ResultsEntry>,
    /// `hashes[i]` covers the header and entries `0..=i`.
    pub hashes: Vec<String>,
    /// The last hash (the header's own hash when there are no entries).
    pub final_hash: String,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The chain: `h0 = sha256("katabox-results/v1|" + pack_sha256 + "|" + student + "|" +
/// assignment id + "|" + title + "|" + started_at + "|" + ended_at + "|" + include_code)`,
/// then `h_i = sha256(h_{i-1} + canonical_json(entry_i))`, every hash as lowercase hex.
pub fn chain(file: &ResultsFile) -> (Vec<String>, String) {
    let head = format!(
        "{RESULTS_FORMAT}|{}|{}|{}|{}|{}|{}|{}",
        file.assignment.pack_sha256,
        file.student,
        file.assignment.id,
        file.assignment.title,
        file.started_at,
        file.ended_at,
        file.include_code
    );
    let mut h = hex(&Sha256::digest(head.as_bytes()));
    let mut out = Vec::with_capacity(file.entries.len());
    for e in &file.entries {
        let canonical = serde_json::to_string(e).unwrap_or_default();
        let mut d = Sha256::new();
        d.update(h.as_bytes());
        d.update(canonical.as_bytes());
        h = hex(&d.finalize());
        out.push(h.clone());
    }
    (out, h)
}

/// Whether the stored hashes match the content.
pub fn chain_ok(file: &ResultsFile) -> bool {
    let (hashes, last) = chain(file);
    hashes == file.hashes && last == file.final_hash
}

/// One problem's entry from its submissions inside the window (oldest first).
pub fn entry_for(
    problem: &str,
    subs: &[&WindowSubmission],
    started_at: i64,
    include_code: bool,
) -> ResultsEntry {
    let first_ok = subs.iter().find(|s| s.verdict.is_accepted());
    let best = first_ok.or(subs.last());
    ResultsEntry {
        problem_id: problem.to_string(),
        language: best.map(|s| s.language),
        best_verdict: best.map(|s| s.verdict),
        submits: u32::try_from(subs.len()).unwrap_or(u32::MAX),
        first_accepted_s: first_ok.map(|s| ((s.created_at - started_at) / 1000).max(0)),
        code: best
            .filter(|_| include_code)
            .map(|s| truncate_bytes(&s.code, MAX_CODE_BYTES)),
    }
}

/// `s` cut to at most `max` bytes, on a character boundary.
fn truncate_bytes(s: &str, max: usize) -> String {
    let mut end = s.len().min(max);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

fn plain_text(s: &str, max: usize) -> bool {
    !s.trim().is_empty() && s.chars().count() <= max && !s.chars().any(char::is_control)
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn valid_problem_id(s: &str) -> bool {
    (1..=64).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Read and check a results file's bytes (everything but the chain, which only flags).
pub fn parse_results(bytes: &[u8]) -> std::result::Result<ResultsFile, String> {
    if bytes.len() as u64 > MAX_RESULTS_BYTES {
        return Err("larger than 8 MB".into());
    }
    let f: ResultsFile =
        serde_json::from_slice(bytes).map_err(|e| format!("not a results file: {e}"))?;
    if f.format != RESULTS_FORMAT {
        return Err(format!("format must be {RESULTS_FORMAT:?}"));
    }
    if !plain_text(&f.student, 80) {
        return Err("student: 1-80 characters, no control characters".into());
    }
    if !kb_pack::kbpack::valid_pack_id(&f.assignment.id)
        || !plain_text(&f.assignment.title, 80)
        || !is_hex64(&f.assignment.pack_sha256)
    {
        return Err("assignment: bad id, title or pack hash".into());
    }
    if !plain_text(&f.app_version, 40) {
        return Err("app_version: bad value".into());
    }
    if f.started_at < 0 || f.ended_at < f.started_at {
        return Err("started_at / ended_at: bad times".into());
    }
    if f.entries.is_empty() || f.entries.len() > kb_pack::kbpack::MAX_ASSIGNMENT_PROBLEMS {
        return Err("entries: 1-50 problems".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    for e in &f.entries {
        if !valid_problem_id(&e.problem_id) || !seen.insert(e.problem_id.as_str()) {
            return Err(format!(
                "entries: bad or repeated problem id {:?}",
                e.problem_id
            ));
        }
        if e.first_accepted_s.is_some_and(|s| s < 0)
            || e.code.as_ref().is_some_and(|c| c.len() > MAX_CODE_BYTES)
            || (e.code.is_some() && !f.include_code)
            || (e.submits == 0 && (e.best_verdict.is_some() || e.language.is_some()))
        {
            return Err(format!("entries: bad values for {}", e.problem_id));
        }
    }
    if f.hashes.len() != f.entries.len()
        || !f.hashes.iter().all(|h| is_hex64(h))
        || !is_hex64(&f.final_hash)
    {
        return Err("hashes: one per entry, 64 hex characters each".into());
    }
    Ok(f)
}

/// A CSV cell (RFC 4180): quoted when needed, and a leading `=`, `+`, `-`, `@`, tab or
/// carriage return prefixed with `'` so spreadsheets never read it as a formula.
pub fn csv_cell(s: &str) -> String {
    let mut v = String::with_capacity(s.len() + 2);
    if s.starts_with(['=', '+', '-', '@', '\t', '\r']) {
        v.push('\'');
    }
    v.push_str(s);
    if v.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", v.replace('"', "\"\""))
    } else {
        v
    }
}

/// One student's row in the class summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClassStudent {
    pub student: String,
    /// `false` when the file's hash chain does not match: edited after export.
    pub chain_ok: bool,
    pub imported_at: i64,
    /// Per problem of [`ClassAssignment::problems`], in order.
    pub cells: Vec<Option<ResultsEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClassAssignment {
    pub id: String,
    pub title: String,
    pub problems: Vec<String>,
    pub students: Vec<ClassStudent>,
}

/// What an import of results files did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ClassImport {
    pub added: u32,
    /// Files imported before (same bytes).
    pub duplicates: u32,
    /// Files kept but whose chain does not match.
    pub edited: u32,
    /// `(file name, why)` for files refused.
    pub refused: Vec<(String, String)>,
}

/// The class summary from imported files (each file's latest import per student wins).
pub fn summarize(rows: &[kb_store::classroom::ClassResultRow]) -> Vec<ClassAssignment> {
    let mut by: BTreeMap<String, ClassAssignment> = BTreeMap::new();
    let mut files: BTreeMap<String, Vec<(ResultsFile, bool, i64)>> = BTreeMap::new();
    for r in rows {
        let Ok(f) = serde_json::from_value::<ResultsFile>(r.data.clone()) else {
            continue;
        };
        files
            .entry(r.assignment_id.clone())
            .or_default()
            .push((f, r.chain_ok, r.imported_at));
    }
    for (id, list) in files {
        let mut problems: Vec<String> = Vec::new();
        for (f, _, _) in &list {
            for e in &f.entries {
                if !problems.contains(&e.problem_id) {
                    problems.push(e.problem_id.clone());
                }
            }
        }
        // The newest file per student.
        let mut latest: BTreeMap<String, (ResultsFile, bool, i64)> = BTreeMap::new();
        for (f, ok, at) in list {
            match latest.get(&f.student) {
                Some((old, _, old_at)) if (old.ended_at, *old_at) >= (f.ended_at, at) => {}
                _ => {
                    latest.insert(f.student.clone(), (f, ok, at));
                }
            }
        }
        let title = latest
            .values()
            .next()
            .map(|(f, _, _)| f.assignment.title.clone())
            .unwrap_or_default();
        let students = latest
            .into_values()
            .map(|(f, chain_ok, imported_at)| ClassStudent {
                cells: problems
                    .iter()
                    .map(|p| f.entries.iter().find(|e| &e.problem_id == p).cloned())
                    .collect(),
                student: f.student,
                chain_ok,
                imported_at,
            })
            .collect();
        by.insert(
            id.clone(),
            ClassAssignment {
                id,
                title,
                problems,
                students,
            },
        );
    }
    by.into_values().collect()
}

/// The CSV for one assignment: one row per student and problem.
pub fn class_csv(a: &ClassAssignment) -> String {
    let mut out = String::from(
        "student,file_unchanged,problem,language,verdict,submits,first_accepted_s\r\n",
    );
    for s in &a.students {
        for (p, cell) in a.problems.iter().zip(&s.cells) {
            let lang = cell
                .as_ref()
                .and_then(|c| c.language)
                .map(|l| l.to_string())
                .unwrap_or_default();
            let verdict = cell
                .as_ref()
                .and_then(|c| c.best_verdict)
                .and_then(|v| serde_json::to_value(v).ok())
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_default();
            let submits = cell.as_ref().map_or(0, |c| c.submits);
            let first = cell
                .as_ref()
                .and_then(|c| c.first_accepted_s)
                .map(|s| s.to_string())
                .unwrap_or_default();
            out.push_str(
                &[
                    csv_cell(&s.student),
                    if s.chain_ok { "yes" } else { "no" }.into(),
                    csv_cell(p),
                    csv_cell(&lang),
                    csv_cell(&verdict),
                    submits.to_string(),
                    first,
                ]
                .join(","),
            );
            out.push_str("\r\n");
        }
    }
    out
}

pub(crate) fn locked() -> AppError {
    AppError::new(
        AppErrorCode::LockedInAssignment,
        "locked while the assignment runs (its hints are off)",
    )
}

fn deadline(a: &AssignmentRow) -> Option<i64> {
    match (a.started_at, a.meta.time_limit_min) {
        (Some(s), Some(m)) => Some(s + i64::from(m) * 60_000),
        _ => None,
    }
}

fn state_of(a: &AssignmentRow) -> AssignmentState {
    match (a.started_at, a.submitted_at) {
        (None, _) => AssignmentState::NotStarted,
        (Some(_), None) => AssignmentState::Running,
        (Some(_), Some(_)) => AssignmentState::Finished,
    }
}

fn not_found(id: &str) -> AppError {
    AppError::new(AppErrorCode::NotFound, format!("no assignment {id}"))
}

impl App {
    /// Assignments of installed packs, finishing any whose time is up.
    async fn current_assignments(&self) -> Result<Vec<AssignmentRow>> {
        let installed: Vec<String> = self.packs().await?.into_iter().map(|p| p.id).collect();
        let now = self.now_ms();
        let mut out = Vec::new();
        for mut a in self.store.user.assignments().await? {
            if !installed.contains(&a.id) {
                continue;
            }
            if a.submitted_at.is_none()
                && let Some(end) = deadline(&a)
                && now >= end
            {
                self.store.user.finish_assignment(&a.id, end).await?;
                a.submitted_at = Some(end);
            }
            out.push(a);
        }
        Ok(out)
    }

    /// The running assignment that holds `problem`, if any.
    pub(crate) async fn assignment_of(&self, problem: &str) -> Result<Option<AssignmentRow>> {
        Ok(self.current_assignments().await?.into_iter().find(|a| {
            state_of(a) == AssignmentState::Running && a.meta.problems.iter().any(|p| p == problem)
        }))
    }

    /// Whether help for `problem` is locked by a running assignment without hints.
    pub(crate) async fn assignment_locks_help(&self, problem: &str) -> Result<bool> {
        Ok(self
            .assignment_of(problem)
            .await?
            .is_some_and(|a| !a.meta.hints))
    }

    /// A running assignment allows only its languages on its problems (SQL problems
    /// always take SQL).
    pub(crate) async fn ensure_assignment_language(
        &self,
        problem: &str,
        language: Language,
    ) -> Result<()> {
        match self.assignment_of(problem).await? {
            Some(a) if language != Language::Sql && !a.meta.languages.contains(&language) => {
                Err(AppError::new(
                    AppErrorCode::InvalidInput,
                    format!("{language} is not one of this assignment's languages"),
                ))
            }
            _ => Ok(()),
        }
    }

    async fn assignment_row(&self, id: &str) -> Result<AssignmentRow> {
        self.current_assignments()
            .await?
            .into_iter()
            .find(|a| a.id == id)
            .ok_or_else(|| not_found(id))
    }

    async fn assignment_view(&self, a: AssignmentRow, lang: UiLang) -> Result<AssignmentView> {
        let all = self.all_problems(lang).await?;
        let subs = match a.started_at {
            Some(s) => {
                self.store
                    .user
                    .window_submissions(&a.meta.problems, s, a.submitted_at.unwrap_or(i64::MAX))
                    .await?
            }
            None => Vec::new(),
        };
        let problems = a
            .meta
            .problems
            .iter()
            .map(|id| {
                let mine: Vec<&WindowSubmission> =
                    subs.iter().filter(|s| &s.problem_id == id).collect();
                let e = entry_for(id, &mine, a.started_at.unwrap_or(0), false);
                AssignmentProblem {
                    id: id.clone(),
                    summary: all.iter().find(|p| &p.id == id).cloned(),
                    best_verdict: e.best_verdict,
                    submits: e.submits,
                }
            })
            .collect();
        Ok(AssignmentView {
            state: state_of(&a),
            ends_at: if a.submitted_at.is_none() {
                deadline(&a)
            } else {
                None
            },
            id: a.id,
            meta: a.meta,
            pack_sha256: a.pack_sha256,
            imported_at: a.imported_at,
            started_at: a.started_at,
            submitted_at: a.submitted_at,
            problems,
        })
    }

    /// Assignments of installed packs, by id.
    pub async fn assignments(&self, lang: UiLang) -> Result<Vec<AssignmentView>> {
        let mut out = Vec::new();
        for a in self.current_assignments().await? {
            out.push(self.assignment_view(a, lang).await?);
        }
        Ok(out)
    }

    pub async fn assignment(&self, id: &str, lang: UiLang) -> Result<AssignmentView> {
        let a = self.assignment_row(id).await?;
        self.assignment_view(a, lang).await
    }

    /// Start an assignment: its rules apply from now until it is finished.
    pub async fn start_assignment(&self, id: &str, lang: UiLang) -> Result<AssignmentView> {
        let a = self.assignment_row(id).await?;
        if a.started_at.is_none() {
            if self
                .current_assignments()
                .await?
                .iter()
                .any(|x| state_of(x) == AssignmentState::Running)
            {
                return Err(AppError::new(
                    AppErrorCode::AssignmentActive,
                    "another assignment is running: finish it first",
                ));
            }
            if self.running_session().await?.is_some() {
                return Err(AppError::new(
                    AppErrorCode::InterviewActive,
                    "a mock interview is running",
                ));
            }
            self.store.user.start_assignment(id, self.now_ms()).await?;
        }
        self.assignment(id, lang).await
    }

    /// Finish a running assignment now: its locks lift and its results are final.
    pub async fn finish_assignment(&self, id: &str, lang: UiLang) -> Result<AssignmentView> {
        let a = self.assignment_row(id).await?;
        if state_of(&a) == AssignmentState::Running {
            let end = deadline(&a).map_or(self.now_ms(), |d| d.min(self.now_ms()));
            self.store.user.finish_assignment(id, end).await?;
        }
        self.assignment(id, lang).await
    }

    /// The results file for a started assignment, exactly as it would be saved.
    pub async fn assignment_results(
        &self,
        id: &str,
        student: &str,
        include_code: bool,
    ) -> Result<ResultsFile> {
        let a = self.assignment_row(id).await?;
        let student = student.trim();
        if !plain_text(student, 80) {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "your name: 1-80 characters",
            ));
        }
        let Some(started) = a.started_at else {
            return Err(AppError::new(
                AppErrorCode::InvalidInput,
                "start the assignment first",
            ));
        };
        let ended = a.submitted_at.unwrap_or_else(|| self.now_ms()).max(started);
        let subs = self
            .store
            .user
            .window_submissions(&a.meta.problems, started, ended)
            .await?;
        let entries = a
            .meta
            .problems
            .iter()
            .map(|p| {
                let mine: Vec<&WindowSubmission> =
                    subs.iter().filter(|s| &s.problem_id == p).collect();
                entry_for(p, &mine, started, include_code)
            })
            .collect();
        let mut file = ResultsFile {
            format: RESULTS_FORMAT.into(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            student: student.to_string(),
            assignment: ResultsAssignment {
                id: a.id.clone(),
                title: a.meta.title.clone(),
                pack_sha256: a.pack_sha256.clone(),
            },
            started_at: started,
            ended_at: ended,
            include_code,
            entries,
            hashes: Vec::new(),
            final_hash: String::new(),
        };
        let (hashes, last) = chain(&file);
        file.hashes = hashes;
        file.final_hash = last;
        Ok(file)
    }

    /// Save the results file to `path` (a temp file, then rename).
    pub async fn save_assignment_results(
        &self,
        id: &str,
        student: &str,
        include_code: bool,
        path: &Path,
    ) -> Result<ResultsFile> {
        let file = self.assignment_results(id, student, include_code).await?;
        let json = serde_json::to_vec_pretty(&file)
            .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
        let tmp = path.with_extension("json.tmp");
        let io = |e: std::io::Error| {
            AppError::new(
                AppErrorCode::Internal,
                format!("cannot write the results: {e}"),
            )
        };
        std::fs::write(&tmp, json).map_err(io)?;
        std::fs::rename(&tmp, path).map_err(io)?;
        Ok(file)
    }

    // --- Teacher side ---------------------------------------------------------------

    /// Import results files. Each is read with a size cap and checked strictly; a file
    /// whose chain does not match is kept and flagged as edited.
    pub async fn import_class_results(&self, paths: &[std::path::PathBuf]) -> Result<ClassImport> {
        let mut out = ClassImport {
            added: 0,
            duplicates: 0,
            edited: 0,
            refused: Vec::new(),
        };
        for p in paths.iter().take(MAX_RESULTS_FILES) {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let bytes = match std::fs::metadata(p) {
                Ok(m) if m.len() > MAX_RESULTS_BYTES => {
                    out.refused.push((name, "larger than 8 MB".into()));
                    continue;
                }
                Ok(_) => match std::fs::read(p) {
                    Ok(b) => b,
                    Err(e) => {
                        out.refused.push((name, e.to_string()));
                        continue;
                    }
                },
                Err(e) => {
                    out.refused.push((name, e.to_string()));
                    continue;
                }
            };
            let file = match parse_results(&bytes) {
                Ok(f) => f,
                Err(why) => {
                    out.refused.push((name, why));
                    continue;
                }
            };
            let ok = chain_ok(&file);
            let data = serde_json::to_value(&file)
                .map_err(|e| AppError::new(AppErrorCode::Internal, e.to_string()))?;
            let sha = hex(&Sha256::digest(&bytes));
            if self
                .store
                .user
                .add_class_result(
                    &file.assignment.id,
                    &file.student,
                    &sha,
                    &data,
                    ok,
                    self.now_ms(),
                )
                .await?
            {
                out.added += 1;
                if !ok {
                    out.edited += 1;
                }
            } else {
                out.duplicates += 1;
            }
        }
        if paths.len() > MAX_RESULTS_FILES {
            out.refused.push((
                String::new(),
                format!("only the first {MAX_RESULTS_FILES} files were read"),
            ));
        }
        Ok(out)
    }

    /// Every imported assignment's summary.
    pub async fn class_results(&self) -> Result<Vec<ClassAssignment>> {
        Ok(summarize(&self.store.user.class_results().await?))
    }

    /// Write one assignment's CSV to `path`.
    pub async fn save_class_csv(&self, assignment: &str, path: &Path) -> Result<()> {
        let all = self.class_results().await?;
        let a = all
            .iter()
            .find(|a| a.id == assignment)
            .ok_or_else(|| not_found(assignment))?;
        std::fs::write(path, class_csv(a)).map_err(|e| {
            AppError::new(AppErrorCode::Internal, format!("cannot write the CSV: {e}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sub(p: &str, v: Verdict, at: i64, code: &str) -> WindowSubmission {
        WindowSubmission {
            problem_id: p.into(),
            language: Language::Python,
            code: code.into(),
            verdict: v,
            created_at: at,
        }
    }

    fn file() -> ResultsFile {
        let s = [
            sub("a", Verdict::WrongAnswer, 2_000, "x"),
            sub("a", Verdict::Accepted, 61_000, "good"),
            sub("a", Verdict::WrongAnswer, 90_000, "later"),
        ];
        let refs: Vec<&WindowSubmission> = s.iter().collect();
        let mut f = ResultsFile {
            format: RESULTS_FORMAT.into(),
            app_version: "1.0.0-beta.1".into(),
            student: "Ana".into(),
            assignment: ResultsAssignment {
                id: "week".into(),
                title: "Week 3".into(),
                pack_sha256: "a".repeat(64),
            },
            started_at: 1_000,
            ended_at: 100_000,
            include_code: true,
            entries: vec![
                entry_for("a", &refs, 1_000, true),
                entry_for("b", &[], 1_000, true),
            ],
            hashes: vec![],
            final_hash: String::new(),
        };
        let (h, last) = chain(&f);
        f.hashes = h;
        f.final_hash = last;
        f
    }

    #[test]
    fn entries_take_the_first_accepted() {
        let f = file();
        let a = &f.entries[0];
        assert_eq!(a.best_verdict, Some(Verdict::Accepted));
        assert_eq!((a.submits, a.first_accepted_s), (3, Some(60)));
        assert_eq!(a.code.as_deref(), Some("good"));
        let b = &f.entries[1];
        assert_eq!((b.submits, b.best_verdict, b.language), (0, None, None));
    }

    #[test]
    fn the_chain_detects_edits() {
        let f = file();
        assert!(chain_ok(&f));
        let bytes = serde_json::to_vec(&f).unwrap();
        assert_eq!(parse_results(&bytes).unwrap(), f);
        // An edited verdict, student, time or code breaks it.
        let mut e = f.clone();
        e.entries[1].submits = 1;
        e.entries[1].best_verdict = Some(Verdict::Accepted);
        e.entries[1].language = Some(Language::Python);
        assert!(!chain_ok(&e));
        let mut e = f.clone();
        e.student = "Bo".into();
        assert!(!chain_ok(&e));
        let mut e = f.clone();
        e.ended_at += 1;
        assert!(!chain_ok(&e));
        let mut e = f.clone();
        e.entries[0].code = Some("copied".into());
        assert!(!chain_ok(&e));
        // Reordering entries breaks it too.
        let mut e = f.clone();
        e.entries.swap(0, 1);
        assert!(!chain_ok(&e));
    }

    #[test]
    fn bad_results_files_are_refused() {
        let good = serde_json::to_value(file()).unwrap();
        let bad = |change: &dyn Fn(&mut serde_json::Value)| {
            let mut v = good.clone();
            change(&mut v);
            parse_results(&serde_json::to_vec(&v).unwrap()).unwrap_err()
        };
        assert!(bad(&|v| v["format"] = "katabox-results/v2".into()).contains("format"));
        assert!(bad(&|v| v["extra"] = 1.into()).contains("unknown field"));
        assert!(bad(&|v| v["student"] = "".into()).contains("student"));
        assert!(bad(&|v| v["student"] = "a\u{7}b".into()).contains("student"));
        assert!(bad(&|v| v["assignment"]["id"] = "two-sum".into()).contains("assignment"));
        assert!(bad(&|v| v["ended_at"] = 0.into()).contains("times"));
        assert!(bad(&|v| v["entries"][1]["problem_id"] = "a".into()).contains("repeated"));
        assert!(bad(&|v| v["entries"][0]["problem_id"] = "../x".into()).contains("problem id"));
        assert!(
            bad(&|v| v["entries"][0]["best_verdict"] = "great".into()).contains("results file")
        );
        assert!(bad(&|v| v["hashes"] = serde_json::json!([])).contains("hashes"));
        assert!(bad(&|v| v["include_code"] = false.into()).contains("bad values"));
        assert!(
            bad(&|v| v["entries"][0]["code"] = "x".repeat(MAX_CODE_BYTES + 1).into())
                .contains("bad values")
        );
        assert!(
            parse_results(&vec![b' '; MAX_RESULTS_BYTES as usize + 1])
                .unwrap_err()
                .contains("8 MB")
        );
    }

    #[test]
    fn csv_cells_defuse_formulas_and_quote() {
        assert_eq!(csv_cell("Ana"), "Ana");
        assert_eq!(csv_cell("=HYPERLINK(\"x\")"), "\"'=HYPERLINK(\"\"x\"\")\"");
        assert_eq!(csv_cell("+1"), "'+1");
        assert_eq!(csv_cell("-2"), "'-2");
        assert_eq!(csv_cell("@SUM(A1)"), "'@SUM(A1)");
        assert_eq!(csv_cell("\tx"), "'\tx");
        assert_eq!(csv_cell("a,b"), "\"a,b\"");
        assert_eq!(csv_cell("line\nbreak"), "\"line\nbreak\"");
        assert_eq!(csv_cell("Zoë \"Z\""), "\"Zoë \"\"Z\"\"\"");
    }

    #[test]
    fn the_summary_keeps_each_students_newest_file() {
        let f = file();
        let mut older = f.clone();
        older.ended_at = 50_000;
        older.entries.truncate(1);
        let mut bo = f.clone();
        bo.student = "=cmd|' /C calc'!A0".into();
        let row = |id: i64, f: &ResultsFile, ok: bool| kb_store::classroom::ClassResultRow {
            id,
            assignment_id: "week".into(),
            student: f.student.clone(),
            file_sha256: id.to_string(),
            data: serde_json::to_value(f).unwrap(),
            chain_ok: ok,
            imported_at: id,
        };
        let s = summarize(&[row(1, &older, true), row(2, &f, true), row(3, &bo, false)]);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].problems, ["a", "b"]);
        assert_eq!(s[0].students.len(), 2);
        let ana = s[0].students.iter().find(|x| x.student == "Ana").unwrap();
        assert_eq!(ana.cells.len(), 2);
        assert_eq!(ana.cells[0].as_ref().unwrap().submits, 3);
        let csv = class_csv(&s[0]);
        let lines: Vec<&str> = csv.split("\r\n").collect();
        assert_eq!(
            lines[0],
            "student,file_unchanged,problem,language,verdict,submits,first_accepted_s"
        );
        assert!(
            csv.contains("\r\n'=cmd|' /C calc'!A0,no,a,python,accepted,3,60\r\n"),
            "{csv}"
        );
        assert!(csv.contains("Ana,yes,b,,,0,"), "{csv}");
    }
}
