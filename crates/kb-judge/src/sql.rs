//! SQL problems (ADR 0031): one SELECT over the problem's tables, judged with the bundled
//! Python's sqlite3 in the sandbox.
//!
//! - [`parse_schema`]: the restricted `schema.sql` a package may use (`CREATE TABLE` with
//!   INTEGER / REAL / TEXT columns, PRIMARY KEY, NOT NULL, REFERENCES). Anything else is
//!   refused, so the schema is always safe to run.
//! - [`check_tables`]: a test's tables against the schema (shape, types, keys).
//! - [`judge_sql`]: one sandboxed run per test of `languages/sql/harness.py`, which locks
//!   the connection down before the query is parsed (read-only, an authorizer that only
//!   allows reading, SQLite limits, a CPU deadline) and prints one nonce record. A query
//!   that does not compile (syntax, unknown names, not a SELECT, a denied action) is a
//!   compile error for the whole submission; rows are compared by the problem's rows
//!   checker in Rust.

use std::collections::HashSet;

use kb_core::{
    CompileInfo, Failure, JudgeEvent, Language, Limits, Message, MessageCode, Report, TestCase,
    TestOutcome, TestResult, Verdict, overall_verdict,
};
use kb_runner::{SandboxSpec, Termination};
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::checker::CheckOutcome;
use crate::judge::{JudgeEnv, WorkDir, compile_failure, new_nonce};
use crate::languages::definition;
use crate::{JudgeError, Problem, limits};

/// The harness, run by the bundled Python.
pub const HARNESS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../languages/sql/harness.py"
));
/// The most rows a query may return.
pub const MAX_ROWS: usize = 10_000;
/// The harness's output cap (rows are compact JSON).
const OUTPUT_BYTES: usize = 8 * 1024 * 1024;
/// The language that runs SQL problems.
pub const HOST: Language = Language::Python;

/// A column's declared type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnType {
    Integer,
    Real,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ColumnType,
    pub primary_key: bool,
    pub not_null: bool,
    /// `REFERENCES table(column)`.
    pub references: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

/// A parsed `schema.sql`, and its text (what the harness runs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlSchema {
    pub tables: Vec<Table>,
    pub text: String,
}

impl SqlSchema {
    /// The tables as JSON, for the schema panel (name, columns with type and keys).
    pub fn to_json(&self) -> Value {
        serde_json::to_value(&self.tables).unwrap_or(Value::Null)
    }

    fn table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }
}

/// Words that would make a table or column name ambiguous in a query.
const RESERVED: &[&str] = &[
    "select",
    "from",
    "where",
    "group",
    "order",
    "by",
    "having",
    "limit",
    "offset",
    "join",
    "on",
    "as",
    "and",
    "or",
    "not",
    "null",
    "table",
    "create",
    "insert",
    "update",
    "delete",
    "into",
    "values",
    "union",
    "all",
    "distinct",
    "case",
    "when",
    "then",
    "else",
    "end",
    "with",
    "in",
    "is",
    "like",
    "between",
    "exists",
    "primary",
    "key",
    "references",
    "index",
    "default",
    "check",
    "unique",
    "left",
    "right",
    "inner",
    "outer",
    "cross",
    "natural",
    "using",
    "asc",
    "desc",
    "collate",
    "cast",
    "over",
    "partition",
    "window",
    "recursive",
];

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Word(String),
    Open,
    Close,
    Comma,
}

fn tokens(stmt: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let mut chars = stmt.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '(' {
            chars.next();
            out.push(Tok::Open);
        } else if c == ')' {
            chars.next();
            out.push(Tok::Close);
        } else if c == ',' {
            chars.next();
            out.push(Tok::Comma);
        } else if c.is_ascii_alphanumeric() || c == '_' {
            let mut w = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    w.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            out.push(Tok::Word(w));
        } else {
            return Err(format!("unexpected character {c:?}"));
        }
    }
    Ok(out)
}

fn name(word: &str, what: &str) -> Result<String, String> {
    let ok = word.len() <= 30
        && word.starts_with(|c: char| c.is_ascii_lowercase() || c == '_')
        && word
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    if !ok {
        return Err(format!(
            "{what} {word:?}: names are lowercase letters, digits and _, starting with a letter or _ (at most 30)"
        ));
    }
    if RESERVED.contains(&word) {
        return Err(format!("{what} {word:?} is an SQL keyword"));
    }
    Ok(word.to_string())
}

/// Parse a package's `schema.sql`: `CREATE TABLE name (col TYPE [PRIMARY KEY] [NOT NULL]
/// [REFERENCES t(c)], ...);` statements, `--` comments, nothing else.
pub fn parse_schema(text: &str) -> Result<SqlSchema, String> {
    let without_comments: String = text
        .lines()
        .map(|l| l.split_once("--").map_or(l, |(code, _)| code))
        .collect::<Vec<_>>()
        .join("\n");
    let mut tables: Vec<Table> = Vec::new();
    for stmt in without_comments.split(';') {
        if stmt.trim().is_empty() {
            continue;
        }
        let toks = tokens(stmt)?;
        let kw = |i: usize, w: &str| matches!(toks.get(i), Some(Tok::Word(x)) if x.eq_ignore_ascii_case(w));
        if !(kw(0, "create") && kw(1, "table")) {
            return Err("each statement is CREATE TABLE name (...)".into());
        }
        let table_name = match toks.get(2) {
            Some(Tok::Word(w)) => name(w, "table")?,
            _ => return Err("CREATE TABLE needs a name".into()),
        };
        if tables.iter().any(|t| t.name == table_name) {
            return Err(format!("table {table_name:?} is defined twice"));
        }
        if toks.get(3) != Some(&Tok::Open) || toks.last() != Some(&Tok::Close) {
            return Err(format!("table {table_name:?}: columns go in (...)"));
        }
        let mut columns: Vec<Column> = Vec::new();
        for def in toks[4..toks.len() - 1].split(|t| *t == Tok::Comma) {
            let words: Vec<&Tok> = def.iter().collect();
            let word = |i: usize| match words.get(i) {
                Some(Tok::Word(w)) => Some(w.as_str()),
                _ => None,
            };
            let col_name = name(
                word(0).ok_or(format!("table {table_name:?}: an empty column"))?,
                "column",
            )?;
            if columns.iter().any(|c| c.name == col_name) {
                return Err(format!("{table_name}.{col_name} is defined twice"));
            }
            let ty = match word(1).map(str::to_ascii_uppercase).as_deref() {
                Some("INTEGER") => ColumnType::Integer,
                Some("REAL") => ColumnType::Real,
                Some("TEXT") => ColumnType::Text,
                _ => {
                    return Err(format!(
                        "{table_name}.{col_name}: the type is INTEGER, REAL or TEXT"
                    ));
                }
            };
            let mut col = Column {
                name: col_name,
                ty,
                primary_key: false,
                not_null: false,
                references: None,
            };
            let mut i = 2;
            while i < words.len() {
                let w = word(i).map(str::to_ascii_uppercase);
                match w.as_deref() {
                    Some("PRIMARY")
                        if word(i + 1).is_some_and(|k| k.eq_ignore_ascii_case("key")) =>
                    {
                        col.primary_key = true;
                        i += 2;
                    }
                    Some("NOT") if word(i + 1).is_some_and(|k| k.eq_ignore_ascii_case("null")) => {
                        col.not_null = true;
                        i += 2;
                    }
                    Some("REFERENCES") => {
                        let (Some(t), Some(Tok::Open), Some(c), Some(Tok::Close)) =
                            (word(i + 1), words.get(i + 2), word(i + 3), words.get(i + 4))
                        else {
                            return Err(format!(
                                "{table_name}.{}: REFERENCES table(column)",
                                col.name
                            ));
                        };
                        let target = tables.iter().find(|x| x.name == t).ok_or(format!(
                            "{table_name}.{}: table {t:?} must be defined first",
                            col.name
                        ))?;
                        if !target.columns.iter().any(|x| x.name == c) {
                            return Err(format!(
                                "{table_name}.{}: {t}.{c} does not exist",
                                col.name
                            ));
                        }
                        col.references = Some((t.to_string(), c.to_string()));
                        i += 5;
                    }
                    _ => {
                        return Err(format!(
                            "{table_name}.{}: allowed after the type: PRIMARY KEY, NOT NULL, REFERENCES t(c)",
                            col.name
                        ));
                    }
                }
            }
            columns.push(col);
        }
        if columns.iter().filter(|c| c.primary_key).count() > 1 {
            return Err(format!(
                "table {table_name:?}: one PRIMARY KEY column at most"
            ));
        }
        tables.push(Table {
            name: table_name,
            columns,
        });
    }
    if tables.is_empty() {
        return Err("schema.sql defines no table".into());
    }
    Ok(SqlSchema {
        tables,
        text: text.to_string(),
    })
}

/// The largest integer a test value may hold (exact in JSON readers and TypeScript).
const SAFE_INT: i64 = (1 << 53) - 1;

/// Check a test's input, `{"tables": {name: [[value, ...], ...]}}`, against the schema:
/// every table present, rows of the right length and types, primary keys unique and not
/// NULL, NOT NULL respected, references pointing at existing values.
pub fn check_tables(schema: &SqlSchema, input: &Map<String, Value>) -> Result<(), String> {
    if input.len() != 1 {
        return Err("the input is {\"tables\": {...}}".into());
    }
    let tables = input
        .get("tables")
        .and_then(Value::as_object)
        .ok_or("the input is {\"tables\": {...}}")?;
    for name in tables.keys() {
        if schema.table(name).is_none() {
            return Err(format!("table {name:?} is not in schema.sql"));
        }
    }
    for table in &schema.tables {
        let rows = tables
            .get(&table.name)
            .and_then(Value::as_array)
            .ok_or(format!(
                "table {:?} is missing (an empty table is [])",
                table.name
            ))?;
        for (r, row) in rows.iter().enumerate() {
            let row = row
                .as_array()
                .filter(|x| x.len() == table.columns.len())
                .ok_or(format!(
                    "{} row {}: {} values expected",
                    table.name,
                    r + 1,
                    table.columns.len()
                ))?;
            for (col, v) in table.columns.iter().zip(row) {
                let at = || format!("{} row {}, {}", table.name, r + 1, col.name);
                match v {
                    Value::Null if col.not_null || col.primary_key => {
                        return Err(format!("{}: NULL is not allowed", at()));
                    }
                    Value::Null => {}
                    Value::Number(n) if col.ty == ColumnType::Integer => {
                        let i = n.as_i64().ok_or(format!("{}: a whole number", at()))?;
                        if i.abs() > SAFE_INT {
                            return Err(format!("{}: beyond ±(2⁵³ − 1)", at()));
                        }
                    }
                    Value::Number(_) if col.ty == ColumnType::Real => {}
                    Value::String(_) if col.ty == ColumnType::Text => {}
                    _ => return Err(format!("{}: not a {:?} value", at(), col.ty)),
                }
            }
        }
        if let Some(k) = table.columns.iter().position(|c| c.primary_key) {
            let mut seen = HashSet::new();
            for row in rows {
                if !seen.insert(row[k].to_string()) {
                    return Err(format!("{}: duplicate primary key {}", table.name, row[k]));
                }
            }
        }
    }
    // References, after every table is known to be well formed.
    for table in &schema.tables {
        let rows = tables[&table.name].as_array().expect("checked above");
        for (k, col) in table.columns.iter().enumerate() {
            let Some((t, c)) = &col.references else {
                continue;
            };
            let target = schema.table(t).expect("checked by parse_schema");
            let ci = target
                .columns
                .iter()
                .position(|x| &x.name == c)
                .expect("checked");
            let values: HashSet<String> = tables[t]
                .as_array()
                .expect("checked above")
                .iter()
                .map(|row| row[ci].to_string())
                .collect();
            for (r, row) in rows.iter().enumerate() {
                if !row[k].is_null() && !values.contains(&row[k].to_string()) {
                    return Err(format!(
                        "{} row {}, {}: {} is not in {t}.{c}",
                        table.name,
                        r + 1,
                        col.name,
                        row[k]
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Check an expected output: `{"columns": [names], "rows": [[values]]}` with as many
/// values per row as columns, each a number, text or null.
pub fn check_result_shape(v: &Value) -> Result<(), String> {
    let obj = v
        .as_object()
        .ok_or("the output is {\"columns\": [...], \"rows\": [...]}")?;
    if obj.len() != 2 {
        return Err("the output has exactly \"columns\" and \"rows\"".into());
    }
    let columns = obj
        .get("columns")
        .and_then(Value::as_array)
        .filter(|c| !c.is_empty() && c.iter().all(Value::is_string))
        .ok_or("\"columns\" is a non-empty list of names")?;
    let rows = obj
        .get("rows")
        .and_then(Value::as_array)
        .ok_or("\"rows\" is a list")?;
    for (r, row) in rows.iter().enumerate() {
        let row = row
            .as_array()
            .filter(|x| x.len() == columns.len())
            .ok_or(format!("row {}: {} values expected", r + 1, columns.len()))?;
        if let Some(bad) = row
            .iter()
            .find(|x| x.is_array() || x.is_object() || x.is_boolean())
        {
            return Err(format!(
                "row {}: {bad} is not a number, text or null",
                r + 1
            ));
        }
    }
    Ok(())
}

/// The starter query of an SQL problem: a comment naming the dialect, then a SELECT
/// over the first table (it runs, and is wrong, until the user writes their query).
pub fn starter(schema: &SqlSchema) -> String {
    let first = schema
        .tables
        .first()
        .map(|t| t.name.as_str())
        .unwrap_or("t");
    format!("-- One SELECT statement (SQLite 3.53)\nSELECT *\nFROM {first};\n")
}

/// What the harness reported for one test.
enum HarnessRecord {
    Rows(Value),
    Failed {
        code: String,
        compile: bool,
        message: String,
    },
}

fn read_record(stdout: &str, nonce: &str) -> Option<HarnessRecord> {
    let line = stdout.lines().find_map(|l| l.strip_prefix(nonce))?;
    let v: Value = serde_json::from_str(line).ok()?;
    if v.get("ok")? == &Value::Bool(true) {
        return Some(HarnessRecord::Rows(json!({
            "columns": v.get("columns")?.clone(),
            "rows": v.get("rows")?.clone(),
        })));
    }
    Some(HarnessRecord::Failed {
        code: v.get("code")?.as_str()?.to_string(),
        compile: v.get("phase")?.as_str()? == "compile",
        message: v.get("message")?.as_str()?.to_string(),
    })
}

fn message_code(code: &str) -> MessageCode {
    match code {
        "sql_not_select" => MessageCode::SqlNotSelect,
        "sql_multiple_statements" => MessageCode::SqlMultipleStatements,
        "sql_not_authorized" => MessageCode::SqlNotAuthorized,
        "sql_too_many_rows" => MessageCode::SqlTooManyRows,
        _ => MessageCode::SqlError,
    }
}

/// Judge `query` on `tests` of an SQL problem: one sandboxed run per test.
pub fn judge_sql(
    problem: &Problem,
    tests: &[TestCase],
    query: &str,
    env: &JudgeEnv,
    on_event: &mut dyn FnMut(JudgeEvent),
) -> Result<Report, JudgeError> {
    let schema = problem
        .sql
        .as_ref()
        .ok_or_else(|| JudgeError::Problem(format!("{} is not an SQL problem", problem.id)))?;
    if tests.is_empty() {
        return Err(JudgeError::Problem(format!(
            "{}: no tests to run",
            problem.id
        )));
    }
    let def = definition(HOST)?;
    let toolchain = env.toolchain(HOST)?;
    let (program, args) = def.program_command(toolchain, problem.memory_mb, None)?;
    let mut limits = Limits::default_for(HOST);
    limits.memory_mb = problem.memory_mb;
    limits.cpu_time_ms = limits::cpu_cap_ms(problem.time_limit_ms, &def, env.time_scale);
    let shown_limit_ms = limits::effective_limit_ms(problem.time_limit_ms, &def, env.time_scale);
    let total = u32::try_from(tests.len()).unwrap_or(u32::MAX);
    let finish = |report: Report, on_event: &mut dyn FnMut(JudgeEvent)| {
        on_event(JudgeEvent::Finished {
            verdict: report.verdict,
        });
        Ok(report)
    };

    on_event(JudgeEvent::Running { total });
    let mut report = Report {
        verdict: Verdict::Accepted,
        tests: Vec::new(),
        total_tests: total,
        first_failure: None,
        compile_output: None,
        time_us: 0,
        cpu_us: 0,
        memory_kb: 0,
        compile: None,
    };
    let mut first_failure = None;
    for (i, tc) in tests.iter().enumerate() {
        let tables = tc.input.get("tables").cloned().unwrap_or(Value::Null);
        let nonce = new_nonce();
        let stdin = json!({
            "nonce": nonce,
            "schema": schema.text,
            "tables": tables,
            "query": query,
            "max_rows": MAX_ROWS,
            "deadline_ms": shown_limit_ms,
        });
        let work = WorkDir::create(&env.runs_dir)?;
        std::fs::write(work.path.join(def.solution_file()), HARNESS)?;
        let mut spec = SandboxSpec::new(&env.sandbox_root, program.clone(), &work.path, &limits);
        spec.args = args.clone();
        spec.read_only_dirs = vec![toolchain.to_path_buf()];
        spec.stdin = serde_json::to_vec(&stdin).map_err(|e| JudgeError::Internal(e.to_string()))?;
        spec.max_output_bytes = OUTPUT_BYTES;
        spec.wall_time = spec.cpu_time * env.wall_factor.max(1);
        let outcome = crate::sandbox::run(&spec)?;
        let cpu_us = u64::try_from((outcome.user_time + outcome.kernel_time).as_micros())
            .unwrap_or(u64::MAX);
        let memory_kb = u32::try_from(outcome.peak_memory_bytes.min(spec.memory_bytes) / 1024)
            .unwrap_or(u32::MAX);
        let stdout = String::from_utf8_lossy(&outcome.stdout);
        let record = read_record(&stdout, &nonce);

        let tle = || {
            (
                Verdict::TimeLimitExceeded,
                Some(Message::new(
                    MessageCode::TimeLimit,
                    format!("time limit of {shown_limit_ms} ms exceeded"),
                )),
            )
        };
        let (verdict, message, actual) = match (outcome.termination, record) {
            (Termination::CpuTimeLimit | Termination::WallTimeLimit, _) => {
                let (v, m) = tle();
                (v, m, None)
            }
            (
                _,
                Some(HarnessRecord::Failed {
                    code,
                    compile: true,
                    message,
                }),
            ) if i == 0 => {
                // The query does not compile: the same for every test.
                let info = CompileInfo {
                    time_us: cpu_us,
                    cached: false,
                };
                let report =
                    compile_failure(tests, Message::new(message_code(&code), message), info);
                return finish(report, on_event);
            }
            (_, Some(HarnessRecord::Failed { code, .. })) if code == "time_limit" => {
                let (v, m) = tle();
                (v, m, None)
            }
            (_, Some(HarnessRecord::Failed { code, message, .. })) => {
                let verdict = if code == "sql_too_many_rows" {
                    Verdict::OutputLimitExceeded
                } else {
                    Verdict::RuntimeError
                };
                (
                    verdict,
                    Some(Message::new(message_code(&code), message)),
                    None,
                )
            }
            (_, Some(HarnessRecord::Rows(rows))) => {
                match problem.checker.check(&tc.expected, &rows) {
                    CheckOutcome::Ok => (Verdict::Accepted, None, Some(rows)),
                    CheckOutcome::Wrong(m) => (Verdict::WrongAnswer, Some(m), Some(rows)),
                }
            }
            (Termination::Exited, None) if outcome.memory_limit_hit => (
                Verdict::MemoryLimitExceeded,
                Some(Message::new(
                    MessageCode::MemoryLimit,
                    "memory limit exceeded",
                )),
                None,
            ),
            (Termination::OutputLimit, None) => (
                Verdict::OutputLimitExceeded,
                Some(Message::new(
                    MessageCode::OutputLimit,
                    "the result is too large",
                )),
                None,
            ),
            (_, None) => (
                Verdict::InternalError,
                Some(Message::new(
                    MessageCode::HarnessSilent,
                    format!(
                        "the SQL harness stopped without a result (exit code {})",
                        outcome.exit_code
                    ),
                )),
                None,
            ),
        };
        let time_us = if verdict == Verdict::TimeLimitExceeded {
            u64::from(shown_limit_ms) * 1000
        } else {
            cpu_us
        };
        let test = TestOutcome {
            result: TestResult {
                index: tc.index,
                kind: tc.kind,
                verdict,
                time_us,
                memory_kb,
                message,
            },
            actual,
            stdout: String::new(),
            stderr: None,
        };
        if first_failure.is_none() && !verdict.is_accepted() {
            first_failure = Some(Failure {
                kind: tc.kind,
                index: tc.index,
                verdict,
                input: tc.input.clone(),
                expected: tc.expected.clone(),
                actual: test.actual.clone(),
                message: test.result.message.clone(),
                stdout: String::new(),
                stderr: None,
            });
        }
        report.time_us = report.time_us.saturating_add(time_us);
        report.cpu_us = report.cpu_us.saturating_add(cpu_us);
        report.memory_kb = report.memory_kb.max(memory_kb);
        on_event(JudgeEvent::Checked {
            kind: tc.kind,
            index: tc.index,
            verdict,
            done: u32::try_from(i + 1).unwrap_or(u32::MAX),
            total,
        });
        let stop = matches!(
            verdict,
            Verdict::TimeLimitExceeded | Verdict::MemoryLimitExceeded
        );
        report.tests.push(test);
        if stop {
            break;
        }
    }
    let results: Vec<TestResult> = report.tests.iter().map(|t| t.result.clone()).collect();
    report.verdict = overall_verdict(&results);
    report.first_failure = first_failure;
    finish(report, on_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIBRARY: &str = "-- a small library
CREATE TABLE members (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT);
CREATE TABLE loans (
  id INTEGER PRIMARY KEY,
  member_id INTEGER NOT NULL REFERENCES members(id),
  due TEXT NOT NULL,
  fine REAL
);
";

    #[test]
    fn parses_the_restricted_schema() {
        let s = parse_schema(LIBRARY).unwrap();
        assert_eq!(s.tables.len(), 2);
        let loans = &s.tables[1];
        assert_eq!(loans.name, "loans");
        assert_eq!(
            loans.columns[1].references,
            Some(("members".into(), "id".into()))
        );
        assert!(loans.columns[1].not_null);
        assert_eq!(loans.columns[3].ty, ColumnType::Real);
        assert!(s.tables[0].columns[0].primary_key);
        let j = s.to_json();
        assert_eq!(j[1]["columns"][1]["type"], "integer");
    }

    #[test]
    fn refuses_anything_else() {
        for bad in [
            "",
            "DROP TABLE x;",
            "CREATE TABLE t (a INTEGER); INSERT INTO t VALUES (1);",
            "CREATE TABLE t (a BLOB);",
            "CREATE TABLE t (a INTEGER DEFAULT 3);",
            "CREATE TABLE t (a INTEGER, a TEXT);",
            "CREATE TABLE t (a INTEGER PRIMARY KEY, b INTEGER PRIMARY KEY);",
            "CREATE TABLE t (a INTEGER REFERENCES u(x));",
            "CREATE TABLE \"T\" (a INTEGER);",
            "CREATE TABLE t (\"order\" INTEGER);",
            "CREATE TABLE t (order INTEGER);",
            "CREATE TABLE t (a INTEGER); CREATE TABLE t (b TEXT);",
            "CREATE TABLE t (a INTEGER CHECK (a > 0));",
            "ATTACH 'x' AS y;",
        ] {
            assert!(parse_schema(bad).is_err(), "{bad}");
        }
    }

    fn input(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn checks_test_tables_against_the_schema() {
        let s = parse_schema(LIBRARY).unwrap();
        let good = json!({"tables": {
            "members": [[1, "ana", null], [2, "bo", "bo@x"]],
            "loans": [[10, 1, "2026-01-05", null], [11, 2, "2026-01-07", 2.5]]
        }});
        check_tables(&s, &input(good)).unwrap();
        let cases = [
            (json!({"tables": {"members": []}}), "missing"),
            (
                json!({"tables": {"members": [], "loans": [], "x": []}}),
                "not in schema.sql",
            ),
            (
                json!({"tables": {"members": [[1, "a"]], "loans": []}}),
                "3 values",
            ),
            (
                json!({"tables": {"members": [[1, null, null]], "loans": []}}),
                "NULL",
            ),
            (
                json!({"tables": {"members": [[1.5, "a", null]], "loans": []}}),
                "whole number",
            ),
            (
                json!({"tables": {"members": [[1, 7, null]], "loans": []}}),
                "not a Text",
            ),
            (
                json!({"tables": {"members": [[1, "a", null], [1, "b", null]], "loans": []}}),
                "duplicate",
            ),
            (
                json!({"tables": {"members": [[1, "a", null]], "loans": [[1, 2, "d", null]]}}),
                "not in members.id",
            ),
            (
                json!({"tables": {"members": [[9007199254740992i64, "a", null]], "loans": []}}),
                "2⁵³",
            ),
        ];
        for (v, want) in cases {
            let e = check_tables(&s, &input(v.clone())).unwrap_err();
            assert!(e.contains(want), "{v}: {e}");
        }
    }

    #[test]
    fn expected_outputs_have_the_result_shape() {
        check_result_shape(&json!({"columns": ["a", "b"], "rows": [[1, "x"], [null, 2.5]]}))
            .unwrap();
        for bad in [
            json!({"columns": [], "rows": []}),
            json!({"columns": ["a"], "rows": [[1, 2]]}),
            json!({"columns": ["a"], "rows": [[[1]]]}),
            json!({"columns": ["a"], "rows": [], "x": 1}),
            json!([1, 2]),
        ] {
            assert!(check_result_shape(&bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn reads_harness_records_by_nonce_only() {
        let out = "noise\nNONCE{\"ok\": true, \"columns\": [\"a\"], \"rows\": [[1]]}\n";
        assert!(matches!(
            read_record(out, "NONCE"),
            Some(HarnessRecord::Rows(_))
        ));
        assert!(read_record("{\"ok\": true}", "NONCE").is_none());
        let failed = "NONCE{\"ok\": false, \"code\": \"sql_error\", \"phase\": \"compile\", \"message\": \"no such table: x\"}";
        match read_record(failed, "NONCE") {
            Some(HarnessRecord::Failed {
                code,
                compile,
                message,
            }) => {
                assert_eq!((code.as_str(), compile), ("sql_error", true));
                assert!(message.contains("no such table"));
            }
            _ => panic!(),
        }
    }
}
