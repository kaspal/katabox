//! The SQL harness's protections (ADR 0031): every way a query could escape "read the
//! test's tables", sent as an ordinary submission. Each must be stopped and named by its
//! code, and nothing may be written to disk. Two suites, run one after the other:
//! the plan's attacks, then SQLite's limits and determinism. Needs the dev sandbox root:
//!
//! ```text
//! cargo test -p kb-judge --test sql_attacks_windows -- --nocapture --test-threads=1
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use kb_core::{MessageCode, Report, Verdict};
use kb_judge::sql::judge_sql;
use kb_judge::{JudgeEnv, Problem};
use kb_runner::ROOT_MARKER;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(root.join(ROOT_MARKER).is_file(), "sandbox root missing");
    JudgeEnv::dev(root)
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(files_under(&p));
            } else {
                out.push(p);
            }
        }
    }
    out
}

/// The two suites share the runs folder, whose file count is part of the check: they
/// take turns even when the harness runs tests in parallel.
static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Judge `query` on the fixture's first test only; check nothing new appears on disk.
fn attack(query: &str) -> Report {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sql/late-returns");
    let p = Problem::load(&fixture).unwrap();
    let env = env();
    let before = files_under(&env.runs_dir).len();
    let started = Instant::now();
    let r = judge_sql(&p, &p.tests[..1], query, &env, &mut |_| {}).unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(30),
        "{query:.80}: too slow"
    );
    assert!(
        files_under(&env.runs_dir).len() <= before,
        "{query:.80}: left files behind"
    );
    for name in ["evil.db", "x.db", "out.db"] {
        assert!(
            !fixture.join(name).exists() && !env.sandbox_root.join(name).exists(),
            "{query:.80}: wrote {name}"
        );
    }
    r
}

/// What stopped the query: the compile message, or the first test's.
fn why(r: &Report) -> Option<kb_core::Message> {
    r.compile_output
        .clone()
        .or_else(|| r.tests.first().and_then(|t| t.result.message.clone()))
}

fn stopped_with(query: &str, verdict: Verdict, code: MessageCode) {
    let r = attack(query);
    assert_eq!(r.verdict, verdict, "{query:.80}: {:?}", why(&r));
    let m = why(&r).unwrap();
    assert_eq!(m.code, code, "{query:.80}: {}", m.text);
    println!("ok  {query:.60} → {:?} {:?}: {:.80}", verdict, code, m.text);
}

#[test]
fn plan_attacks_are_contained() {
    let _turn = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    use MessageCode::*;
    use Verdict::*;
    stopped_with("ATTACH 'evil.db' AS evil", CompilationError, SqlNotSelect);
    stopped_with(
        "SELECT load_extension('evil')",
        CompilationError,
        SqlNotAuthorized,
    );
    stopped_with("PRAGMA writable_schema = 1", CompilationError, SqlNotSelect);
    stopped_with("VACUUM INTO 'out.db'", CompilationError, SqlNotSelect);
    stopped_with(
        "INSERT INTO members VALUES (9, 'x')",
        CompilationError,
        SqlNotSelect,
    );
    stopped_with("DELETE FROM loans", CompilationError, SqlNotSelect);
    stopped_with("CREATE TABLE x (a INTEGER)", CompilationError, SqlNotSelect);
    stopped_with(
        "WITH t AS (SELECT 1) DELETE FROM loans",
        CompilationError,
        SqlNotAuthorized,
    );
    stopped_with(
        "WITH t AS (SELECT 1) INSERT INTO members VALUES (9, 'x')",
        CompilationError,
        SqlNotAuthorized,
    );
    stopped_with(
        "SELECT 1; ATTACH 'evil.db' AS evil",
        CompilationError,
        SqlMultipleStatements,
    );
    stopped_with(
        "SELECT 1; SELECT 2",
        CompilationError,
        SqlMultipleStatements,
    );
    stopped_with(
        "SELECT * FROM pragma_database_list",
        RuntimeError,
        SqlNotAuthorized,
    );
    // A huge cross join: the row cap stops it after 10 000 rows.
    stopped_with(
        "SELECT a.id FROM loans a, loans b, loans c, loans d, loans e, loans f, loans g",
        OutputLimitExceeded,
        SqlTooManyRows,
    );
    // A runaway recursive CTE: the deadline stops it.
    stopped_with(
        "WITH RECURSIVE r(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM r) SELECT count(*) FROM r",
        TimeLimitExceeded,
        TimeLimit,
    );
    // Deep nesting: SQLite's expression depth limit (parentheses alone add no depth).
    let deep = format!("SELECT {}1{}", "abs(".repeat(300), ")".repeat(300));
    stopped_with(&deep, CompilationError, SqlError);
}

#[test]
fn limits_and_determinism_hold() {
    let _turn = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    use MessageCode::*;
    use Verdict::*;
    // Non-deterministic functions are refused (the reference's answer could not match).
    stopped_with("SELECT random()", CompilationError, SqlNotAuthorized);
    stopped_with(
        "SELECT hex(randomblob(4))",
        CompilationError,
        SqlNotAuthorized,
    );
    // A value longer than SQLITE_LIMIT_LENGTH (1.2 MB of hex).
    stopped_with(
        "SELECT length(hex(zeroblob(600000)))",
        RuntimeError,
        SqlError,
    );
    // A query longer than SQLITE_LIMIT_SQL_LENGTH.
    let long = format!("SELECT '{}'", "x".repeat(120_000));
    stopped_with(&long, CompilationError, SqlError);
    // More compound SELECTs than SQLITE_LIMIT_COMPOUND_SELECT.
    let compound = vec!["SELECT 1"; 60].join(" UNION ALL ");
    stopped_with(&compound, CompilationError, SqlError);
    // Too many function arguments (SQLITE_LIMIT_FUNCTION_ARG).
    let args = (0..40)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    stopped_with(&format!("SELECT max({args})"), CompilationError, SqlError);
    // Blobs are not results.
    stopped_with("SELECT zeroblob(4)", RuntimeError, SqlError);
    // Parameters are not allowed (SQLITE_LIMIT_VARIABLE_NUMBER).
    stopped_with("SELECT ?2", CompilationError, SqlError);
    // Reading the catalogue is fine and changes nothing.
    let r = attack("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name");
    assert_eq!(
        r.verdict,
        Verdict::WrongAnswer,
        "{:?}",
        r.tests[0].result.message
    );
    // The same query gives the same answer every time.
    let q = "SELECT m.name, l.title, CAST(julianday(l.returned) - julianday(l.due) AS INTEGER) AS days_late \
             FROM loans l JOIN members m ON m.id = l.member_id WHERE l.returned > l.due ORDER BY days_late DESC, l.title";
    for _ in 0..3 {
        assert_eq!(attack(q).verdict, Verdict::Accepted);
    }
}
