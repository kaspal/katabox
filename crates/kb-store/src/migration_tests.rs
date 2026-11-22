//! Up-migration checks on a real-shaped database (expansion M0). Every migration from
//! 0005 on adds one entry to `SEEDS` (if it adds tables worth seeding) and is covered by
//! `every_version_upgrades_without_losing_rows`: a database at each older version, full
//! of realistic rows, is upgraded and every table that existed before keeps its rows,
//! byte for byte, in the columns it had.

use std::collections::BTreeMap;
use std::path::Path;

use kb_core::Verdict;
use sqlx::Row;

use crate::testutil::temp_dir;
use crate::user::tests::report;
use crate::user::{MIGRATIONS, UserDb};

/// Rows written for each schema version (applied in order up to the DB's version).
/// Times are fixed so fingerprints are stable.
fn seeds(version: i64) -> Vec<String> {
    let ok = serde_json::to_string(&report(Verdict::Accepted)).unwrap();
    let wa = serde_json::to_string(&report(Verdict::WrongAnswer)).unwrap();
    let q = |s: &str| s.replace('\'', "''");
    match version {
        1 => vec![
            "INSERT INTO settings VALUES ('theme', '\"dark\"', 1700000000000),
                 ('editor.keymap', '\"vim\"', 1700000000001),
                 ('app.install_id', '\"0123456789abcdef\"', 1700000000002)"
                .into(),
            "INSERT INTO drafts VALUES ('two-sum', 'python', 'def twoSum(nums, target):\n    pass\n', 1700000001000),
                 ('budget-pair', 'rust', 'fn best_pair() {}', 1700000001001)"
                .into(),
            format!(
                "INSERT INTO submissions (problem_id, language, code, verdict, time_us, memory_kb, report, created_at) VALUES
                 ('two-sum', 'python', 'def twoSum(n, t): return []', 'wrong_answer', 1200, 9000, '{}', 1700000002000),
                 ('two-sum', 'python', 'def twoSum(n, t):\n    seen = {{}}\n', 'accepted', 1500, 9100, '{}', 1700000003000),
                 ('budget-pair', 'typescript', 'function bestPair() {{ return 0; }}', 'accepted', 800, 30000, '{}', 1700000004000)",
                q(&wa),
                q(&ok),
                q(&ok)
            ),
            "INSERT INTO problem_status VALUES ('two-sum', 'solved', 1700000003000, 1700000003000),
                 ('budget-pair', 'solved', 1700000004000, 1700000004000),
                 ('balance-point', 'attempted', NULL, 1700000005000)"
                .into(),
            "INSERT INTO snapshots (problem_id, language, code, code_hash, source, created_at) VALUES
                 ('two-sum', 'python', 'a', 'h1', 'run', 1700000002500),
                 ('two-sum', 'python', 'b', 'h2', 'submit', 1700000003000),
                 ('budget-pair', 'typescript', 'c', 'h3', 'restore', 1700000004500)"
                .into(),
            "INSERT INTO notes VALUES ('two-sum', '# Hash map\nOne pass, ñ and emoji 🎯', 1700000006000)".into(),
            "INSERT INTO hint_unlocks VALUES ('two-sum', 1, 1700000001500), ('two-sum', 2, 1700000001600)".into(),
        ],
        2 => vec!["UPDATE snapshots SET verdict = 'accepted' WHERE source = 'submit'".into()],
        3 => vec![
            "INSERT INTO reviews VALUES ('two-sum', '2026-11-20', 7, 1, 0, 1700000007000),
                 ('budget-pair', '2026-11-18', 3, 0, 1, 1700000007001)"
                .into(),
            "INSERT INTO daily_picks VALUES ('2026-11-14', 'two-sum', 1700000003000),
                 ('2026-11-15', 'balance-point', NULL)"
                .into(),
            "INSERT INTO lists (name, created_at) VALUES ('Interview prep', 1700000008000), ('Graphs', 1700000008001)".into(),
            "INSERT INTO list_items VALUES (1, 'two-sum', 0, 1700000008100), (1, 'budget-pair', 1, 1700000008200),
                 (2, 'fewest-hops', 0, 1700000008300)"
                .into(),
        ],
        4 => vec![
            "INSERT INTO problem_opens VALUES ('two-sum', 1700000000500, 1700000009000),
                 ('fewest-hops', 1700000009100, 1700000009100)"
                .into(),
            "INSERT INTO interview_sessions (started_at, duration_s, problem_ids, languages, mix, ended_at, results) VALUES
                 (1700000010000, 2700, '[\"two-sum\",\"fewest-hops\"]', '[\"python\"]', 'easy+medium', 1700000012000, '{\"solved\":1}'),
                 (1700000020000, 2700, '[\"budget-pair\"]', '[\"rust\"]', 'easy', NULL, NULL)"
                .into(),
        ],
        5 => vec![
            "INSERT INTO kata_sessions (started_at, day, kind, source, problem_ids, duration_s, ended_at) VALUES
                 (1700000030000, '2026-11-16', 'daily', NULL, '[\"two-sum\",\"budget-pair\"]', 900, 1700000031000),
                 (1700000040000, '2026-11-17', 'pattern', 'arrays-hashing', '[\"two-sum\"]', 600, NULL)"
                .into(),
            "INSERT INTO kata_attempts VALUES
                 (1700000030000, 'two-sum', 1700000030100, 2, 'accepted', 1700000030900, 800, 'python', 'def f(): pass'),
                 (1700000030000, 'budget-pair', 1700000030950, 1, 'wrong_answer', NULL, NULL, NULL, NULL)"
                .into(),
        ],
        6 => vec![
            "INSERT INTO study_plans (created_at, goal, weeks, minutes_per_day, start_day, archived) VALUES
                 (1700000050000, '{\"patterns\":[\"arrays-hashing\"],\"left_over\":0}', 2, 30, '2026-11-23', 0)"
                .into(),
            "INSERT INTO plan_items VALUES
                 (1700000050000, 0, '2026-11-23', 'topic', 'hash-tables', 15, 'done', 1700000050100),
                 (1700000050000, 1, '2026-11-23', 'problem', 'two-sum', 20, 'pending', 1700000050000),
                 (1700000050000, 2, '2026-11-29', 'kata', NULL, 15, 'skipped', 1700000050200)"
                .into(),
        ],
        7 => vec![
            "INSERT INTO mistakes VALUES
                 ('two-sum', 'python', 1700000002000, '[\"wa\",\"edge:duplicates\"]', '[\"wa\"]', 'forgot equal values', 1700000060000)"
                .into(),
            "INSERT INTO reflections VALUES ('two-sum', 'one pass with a map of seen values', 'O(n)', 'O(n)', 1700000061000)".into(),
        ],
        8 => vec![
            "INSERT INTO snippets VALUES ('0123456789abcdef', 'python', 'BFS', '[\"graphs\"]', 'from collections import deque', 1700000070000, 1700000070000)".into(),
        ],
        11 => vec![
            "INSERT INTO focus_sessions VALUES (1700000100000, 1700001600000, 25, 5, 1, '{\"touched\":2,\"solved\":1,\"katas\":0,\"plan_items\":0}')".into(),
        ],
        10 => vec![
            "INSERT INTO stress_brutes VALUES ('two-sum', 'python', 'def two_sum(nums, target):\n    return []\n', 1700000090000)".into(),
        ],
        9 => vec![
            "INSERT INTO reading_answers VALUES ('print-basics/1', 2, 1700000080000, 1700000080000),
                 ('complexity-loops/3', 1, NULL, 1700000081000)"
                .into(),
        ],
        _ => vec![],
    }
}

/// A user.db at `version` (a prefix of [`MIGRATIONS`]) with realistic rows in every
/// table that exists at that version.
pub(crate) async fn real_shaped_db(path: &Path, version: usize) {
    let db = UserDb::open_with(path, &MIGRATIONS[..version])
        .await
        .unwrap();
    for v in 1..=version as i64 {
        for sql in seeds(v) {
            sqlx::raw_sql(&sql)
                .execute(&db.pool)
                .await
                .unwrap_or_else(|e| panic!("seed v{v}: {e}\n{sql}"));
        }
    }
    db.close().await;
}

/// Table name -> (its columns, a fingerprint of every row in those columns).
type Fingerprints = BTreeMap<String, (Vec<String>, i64, String)>;

async fn fingerprints(db: &UserDb, only: Option<&Fingerprints>) -> Fingerprints {
    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM sqlite_master WHERE type = 'table'
         AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(&db.pool)
    .await
    .unwrap();
    let mut out = Fingerprints::new();
    for t in tables {
        let cols: Vec<String> = match only {
            Some(before) => match before.get(&t) {
                Some((cols, _, _)) => cols.clone(),
                None => continue,
            },
            None => sqlx::query(&format!("PRAGMA table_info({t})"))
                .fetch_all(&db.pool)
                .await
                .unwrap()
                .iter()
                .map(|r| r.get::<String, _>("name"))
                .collect(),
        };
        let quoted = cols
            .iter()
            .map(|c| format!("quote({c})"))
            .collect::<Vec<_>>()
            .join(" || '|' || ");
        let order = cols.join(", ");
        let rows: Vec<String> =
            sqlx::query_scalar(&format!("SELECT {quoted} FROM {t} ORDER BY {order}"))
                .fetch_all(&db.pool)
                .await
                .unwrap();
        out.insert(t, (cols, rows.len() as i64, rows.join("\n")));
    }
    out
}

/// Seed a DB at `from`, upgrade it to the latest version and check that every old
/// table kept every row (in its old columns) and that a backup was written.
pub(crate) async fn assert_upgrade_keeps(name: &str, from: usize) {
    let dir = temp_dir(name);
    let path = dir.join("user.db");
    real_shaped_db(&path, from).await;
    let old = UserDb::open_with(&path, &MIGRATIONS[..from]).await.unwrap();
    let before = fingerprints(&old, None).await;
    old.close().await;
    assert!(before.values().any(|(_, n, _)| *n > 0), "seeded rows");

    let db = UserDb::open(&path).await.unwrap();
    assert_eq!(db.schema_version().await.unwrap(), MIGRATIONS.len() as i64);
    if from < MIGRATIONS.len() {
        let bak = db.backup_path().expect("backup before migrating");
        assert!(bak.exists());
    }
    let after = fingerprints(&db, Some(&before)).await;
    db.close().await;
    for (t, (_, n, rows)) in &before {
        let (_, n2, rows2) = after.get(t).unwrap_or_else(|| panic!("table {t} dropped"));
        assert_eq!(n, n2, "row count of {t} from v{from}");
        assert_eq!(rows, rows2, "rows of {t} from v{from}");
    }
}

#[tokio::test]
async fn every_version_upgrades_without_losing_rows() {
    for from in 1..=MIGRATIONS.len() {
        assert_upgrade_keeps(&format!("upgrade-v{from}"), from).await;
    }
}

#[tokio::test]
async fn the_latest_schema_takes_every_seed() {
    // Seeds written for older versions still insert at the latest one (new columns
    // must be nullable or defaulted, as the migration rules require).
    let dir = temp_dir("seed-latest");
    real_shaped_db(&dir.join("user.db"), MIGRATIONS.len()).await;
}
