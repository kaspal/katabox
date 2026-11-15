//! SQL problems end to end (ADR 0031): a package loads (schema, tables, results), its
//! reference query is accepted, a wrong query is caught, and compile errors, wrong column
//! names and the row checkers report what the UI needs. Needs the dev sandbox root with
//! Python installed:
//!
//! ```text
//! cargo test -p kb-judge --test sql_windows -- --nocapture --test-threads=1
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::{MessageCode, Report, Verdict};
use kb_judge::sql::judge_sql;
use kb_judge::{JudgeEnv, Problem, ProblemMode};
use kb_runner::ROOT_MARKER;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sql/late-returns")
}

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(root.join(ROOT_MARKER).is_file(), "sandbox root missing");
    JudgeEnv::dev(root)
}

fn run(query: &str) -> Report {
    let p = Problem::load(&fixture()).unwrap();
    judge_sql(&p, &p.tests, query, &env(), &mut |_| {}).unwrap()
}

fn solution(name: &str) -> String {
    std::fs::read_to_string(fixture().join("solutions").join(name)).unwrap()
}

#[test]
fn a_package_loads_with_its_schema() {
    let p = Problem::load(&fixture()).unwrap();
    assert_eq!(p.mode, ProblemMode::Sql);
    let schema = p.sql.as_ref().unwrap();
    assert_eq!(schema.tables.len(), 2);
    assert_eq!(p.tests.len(), 3);
    assert!(p.checker.is_rows());
    // The ordinary judge refuses it: SQL has its own entry point.
    let e = kb_judge::judge(&p, &p.tests, kb_core::Language::Python, "x", &env()).unwrap_err();
    assert!(e.to_string().contains("judge_sql"), "{e}");
}

#[test]
fn the_reference_is_accepted_and_the_wrong_query_is_caught() {
    let ok = run(&solution("ref.sql"));
    assert_eq!(ok.verdict, Verdict::Accepted, "{:?}", ok.first_failure);
    assert_eq!(ok.tests.len(), 3);
    // A trailing semicolon, comments and odd spacing are fine.
    let ok = run(&format!(
        "-- mine\n  {}  ;;\n",
        solution("ref.sql").trim_end().trim_end_matches(';')
    ));
    assert_eq!(ok.verdict, Verdict::Accepted, "{:?}", ok.first_failure);

    let wrong = run(&solution("wrong.sql"));
    assert_eq!(wrong.verdict, Verdict::WrongAnswer);
    let f = wrong.first_failure.unwrap();
    let m = f.message.unwrap();
    assert_eq!(m.code, MessageCode::OutputMismatch);
    assert!(m.text.contains("expected 2 rows, got 3"), "{}", m.text);
    // The rows the query returned are shown next to the expected ones.
    assert_eq!(f.actual.unwrap()["rows"].as_array().unwrap().len(), 3);
}

#[test]
fn columns_order_and_errors_are_reported_by_code() {
    // A wrong column name comes first, with both lists.
    let renamed = solution("ref.sql").replace("AS days_late", "AS late");
    let r = run(&renamed.replace("ORDER BY days_late", "ORDER BY late"));
    let m = r.first_failure.unwrap().message.unwrap();
    assert_eq!(m.code, MessageCode::SqlColumns);
    assert!(
        m.text.contains("days_late") && m.text.contains("late\""),
        "{}",
        m.text
    );
    // rows-ordered: the right rows in the wrong order fail.
    let r = run(&solution("ref.sql").replace("days_late DESC, l.title", "l.title"));
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    assert!(
        r.first_failure
            .unwrap()
            .message
            .unwrap()
            .text
            .starts_with("row 1")
    );
    // A query that does not compile is a compile error for the whole submission.
    let r = run("SELECT name FROM nowhere");
    assert_eq!(r.verdict, Verdict::CompilationError);
    let m = r.compile_output.clone().unwrap();
    assert_eq!(m.code, MessageCode::SqlError);
    assert!(m.text.contains("no such table"), "{}", m.text);
    let r = run("SELEC name FROM members");
    assert_eq!(r.verdict, Verdict::CompilationError);
    // Numbers compare by value: 3.0 is 3.
    let r = run(&solution("ref.sql").replace("AS INTEGER)", "AS REAL)"));
    assert_eq!(r.verdict, Verdict::Accepted, "{:?}", r.first_failure);
}
