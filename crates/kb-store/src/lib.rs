//! Storage for KataBox: two SQLite files (ADR 0004, schema and migration rules in
//! ADR 0010).
//!
//! - [`ContentDb`] — `content.db`, read-only at runtime. Built from problem packages by
//!   `kb-pack build` through [`content::build`]: problems, statements and hints in each
//!   UI language, editorials, tests, reference solutions and FTS5 search.
//! - [`UserDb`] — `user.db`, read-write in WAL mode. Numbered migrations embedded in the
//!   binary run at open, after a backup copy. Settings, drafts, submissions, problem
//!   status, solution history (snapshots), notes and hint unlocks.
//!
//! [`Store`] opens both and merges what needs both (the problem list with its status).
//! Times are UTC: `INTEGER` milliseconds since the Unix epoch in user.db.
//!
//! sqlx is used with runtime queries only (no compile-time checked macros), so building
//! needs no `DATABASE_URL`.

pub mod blob;
pub mod content;
pub mod focus;
pub mod journal;
pub mod kata;
pub mod learn;
#[cfg(test)]
mod migration_tests;
pub mod modes;
pub mod packs;
pub mod plans;
pub mod practice;
pub mod reading;
pub mod reading_answers;
pub mod snippets;
pub mod stress;
pub mod transfer;
pub mod user;

use std::path::Path;

pub use content::{
    CONTENT_SCHEMA_VERSION, ContentDb, ContentMeta, FollowupRef, JudgeData, PackMeta, PackProblem,
    PackSql, PackText, ProblemFilter, ProblemSummary, ProblemView, ProfileData, PublicTest,
    StatusFilter, UiLang, build, build_with_learn,
};
pub use learn::{
    LearnComplexity, LearnInput, LearnPack, LearnPath, LearnSample, LearnSubject, LearnTopic,
    LearnTopicSummary, PackPath, PackQuizOption, PackQuizQuestion, PackSubject, PackTopic,
    QuizQuestion,
};
pub use modes::{AcceptedSolution, SessionRow, SubmitFact};
pub use practice::{ActivityRow, DailyPick, INSTALL_ID_SETTING, ListRow, ReviewRow};
pub use reading::{
    DominantLine, PackReadingItem, PackReadingSet, ReadingItemData, ReadingItemView, ReadingKind,
    ReadingLink, ReadingSetSummary, ReadingSetView,
};
pub use transfer::ExportData;
pub use user::{
    BACKUPS_KEPT, Draft, DraftRef, ProblemStatus, SNAPSHOT_CAP, Snapshot, SnapshotSource,
    StatusInfo, Submission, SubmissionSummary, UserDb,
};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("database: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("user.db schema version {found} is newer than this app supports ({supported})")]
    NewerSchema { found: i64, supported: i64 },
    #[error("content.db schema version {found} is not supported (expected {expected})")]
    ContentSchema { found: i64, expected: i64 },
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid pack: {0}")]
    InvalidPack(String),
    #[error("unexpected data in the database: {0}")]
    Corrupt(String),
    #[error("invalid request: {0}")]
    Invalid(String),
}

pub type Result<T, E = StoreError> = std::result::Result<T, E>;

/// Both databases. Most calls go straight to [`Store::content`] or [`Store::user`].
#[derive(Debug, Clone)]
pub struct Store {
    pub content: ContentDb,
    pub user: UserDb,
}

impl Store {
    pub async fn open(content_db: &Path, user_db: &Path) -> Result<Store> {
        Ok(Store {
            content: ContentDb::open(content_db).await?,
            user: UserDb::open(user_db).await?,
        })
    }

    /// The problem list with each problem's status, filtered by `filter` (status
    /// included).
    pub async fn list_problems(&self, filter: &ProblemFilter) -> Result<Vec<ProblemSummary>> {
        let statuses = self.user.statuses().await?;
        let mut list = self.content.list_problems(filter).await?;
        for p in &mut list {
            p.status = statuses.get(&p.id).copied();
        }
        if let Some(want) = filter.status {
            list.retain(|p| want.matches(p.status));
        }
        Ok(list)
    }

    pub async fn close(&self) {
        self.content.close().await;
        self.user.close().await;
    }
}

/// Milliseconds since the Unix epoch, UTC.
pub(crate) fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::Verdict;

    #[tokio::test]
    async fn problem_list_merges_status() {
        let (dir, content) = content::tests::sample_db("store").await;
        let store = Store {
            content,
            user: UserDb::open(&dir.join("user.db")).await.unwrap(),
        };
        let report = user::tests::report(Verdict::Accepted);
        store
            .user
            .record_submission("gap-b", kb_core::Language::Python, "x", &report)
            .await
            .unwrap();
        let list = |status| {
            let store = store.clone();
            async move {
                store
                    .list_problems(&ProblemFilter {
                        status,
                        ..Default::default()
                    })
                    .await
                    .unwrap()
            }
        };
        let all = list(None).await;
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].status, None);
        assert_eq!(all[1].status, Some(ProblemStatus::Solved));
        let ids = |v: Vec<ProblemSummary>| v.into_iter().map(|p| p.id).collect::<Vec<_>>();
        assert_eq!(ids(list(Some(StatusFilter::Solved)).await), ["gap-b"]);
        assert_eq!(ids(list(Some(StatusFilter::Todo)).await), ["gap-a"]);
        assert!(list(Some(StatusFilter::Attempted)).await.is_empty());
        store.close().await;
    }
}

#[cfg(test)]
pub(crate) mod testutil {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A fresh, empty temp folder for one test.
    pub fn temp_dir(name: &str) -> PathBuf {
        static N: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "kb-store-{name}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
