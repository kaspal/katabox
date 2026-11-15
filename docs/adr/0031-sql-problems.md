# 31. SQL problems

Date: 2026-11-14 · Status: proposed (features-content M7: the engine; M12: the app,
after the maintainer allowed the kb-bridge arms on 2026-11-15).

## Context
SQL is a common interview and practice topic. KataBox already ships Python, whose
standard library includes sqlite3 (SQLite 3.53.1 with the pinned Python 3.14.10). So an
SQL problem needs no new runtime download and no kb-runner change: the query runs inside
the existing sandbox, through the bundled Python.

## Decision
- **A third problem mode**, `mode = "sql"` (`ProblemMode::Sql`). The answer is one
  SELECT (or WITH … SELECT) over the problem's tables.
- **The package:**
  - `schema.sql` in a restricted subset: `CREATE TABLE` with INTEGER / REAL / TEXT
    columns, PRIMARY KEY, NOT NULL and REFERENCES t(c), plus `--` comments.
    `kb_judge::sql::parse_schema` accepts nothing else: no other statements, no
    defaults, checks, quoted names or keyword names. So the schema is always safe to
    run.
  - Tests are `{"input": {"tables": {name: [[row], …]}}, "output": {"columns": […],
    "rows": [[…]]}}`. `check_tables` checks the tables against the schema: shape, types,
    NULLs, unique primary keys, references, and integers within ±(2⁵³ − 1).
  - `solutions/ref.sql`, and `solutions/wrong.sql`: the plausible mistake the tests must
    catch.
- **Running** (`kb_judge::sql::judge_sql`): one sandboxed run per test of
  `languages/sql/harness.py` on the Python host, with Python's limits scaled as usual
  (ADR 0017). The harness:
  1. builds an in-memory database from the schema and the test's rows;
  2. **locks it down before the query is parsed**: `PRAGMA query_only`; an authorizer
     that allows only SELECT, READ, FUNCTION and RECURSIVE, and refuses
     `load_extension`, `random` and `randomblob` (a non-deterministic answer could
     never match); and SQLite limits on length, SQL length, expression depth, compound
     SELECTs, variables, function arguments, VDBE ops, attached databases, columns and
     LIKE patterns;
  3. checks the first keyword (SELECT or WITH) and **compiles** the query with
     `EXPLAIN`, which runs nothing. Syntax errors, unknown names, denied actions, more
     than one statement, or a statement that is not a SELECT become a **compile error**
     for the whole submission;
  4. runs the query with a progress-handler deadline (TLE), and reads at most 10 000
     rows (`sql_too_many_rows`). Blobs and non-finite numbers are refused;
  5. prints one nonce record (the ADR 0008 idea; a query cannot print anyway).
- **Checking** (`Checker::Rows`): `rows-ordered` when the statement asks for an order,
  `rows-unordered` (a multiset) otherwise, optionally with `:<eps>`.
  - **Columns first:** names compared case-insensitively, in order. A mismatch is
    reported as `sql_columns`, with both lists.
  - **Then the rows:** the number of rows, then the first differing row.
  - **Values:** numbers compare by value (2 equals 2.0), text exactly, NULL only with
    NULL.
- **Message codes:** `sql_error`, `sql_not_select`, `sql_multiple_statements`,
  `sql_not_authorized`, `sql_too_many_rows`, `sql_columns` (EN/ES in the UI).
- **Tests:**
  - unit tests for the schema parser, the table checks, the result shape, the records
    and the checker;
  - `sql_windows`: a seed package (`tests/fixtures/sql/late-returns`) end to end;
  - `sql_attacks_windows`, run twice. The plan's attacks: ATTACH, load_extension,
    PRAGMA, VACUUM INTO, writes (plain and inside WITH), several statements, a pragma
    table, a huge cross join, a runaway recursive CTE, deep nesting. Then limits and
    determinism: random, randomblob, an over-long value, an over-long query, too many
    compound terms, too many arguments, a blob, parameters, catalogue reads, and the
    same answer every time. Each is stopped by its code and leaves nothing on disk.

## In the app (features-content M12)
- **`Language::Sql`**, its id and extension `sql`. `Language::ALL` stays the five code
  languages (installs, language servers, debug, stats rows); `Language::EVERY` adds SQL
  for parsing. An SQL problem takes only SQL and SQL only fits SQL problems:
  kb-app refuses any other pairing with `unsupported_language`, and the UI offers only
  SQL on an SQL problem without ever saving it as the editor language.
- **No toolchain of its own:** kb-toolchain maps SQL to the Python unit, and the UI
  asks for Python when it is missing.
- **kb-bridge** (the maintainer's decision of 2026-11-15): SQL is reported unavailable
  ("no language server for SQL"), its LSP id is `sql`, and it has no project config.
  No server, settings or message policy changed. The editor uses
  `@codemirror/lang-sql` 6.10.0 (MIT, exact pin, lazy) with the SQLite dialect and
  completion of the problem's tables and columns instead of a server.
- **content.db schema 8:** `problems.schema_sql` (the text the judge runs) and
  `problems.sql_schema` (the tables as JSON for the panel and completion); a CHECK ties
  both to `mode = 'sql'`. `Problem::from_sql_parts` rebuilds an SQL problem from them
  with the same checks as a package.
- **kb-app:** Run and Submit go through `judge_sql`; the starter is one SELECT over the
  first table (`sql::starter`); custom tests (`invalid_custom_input`), the debugger
  (`debug_unsupported`), the shrinker and the profiler (no profile) are off, each with
  its reason in the UI. Mock interviews never pick an SQL problem, and the daily
  problem and "next problem" leave SQL out until the user has run or submitted one
  (`practice::hide_unopened_sql`).
- **The UI:** a Tables tab (columns, types, keys, five sample rows of public test 1),
  tests as tables, and a wrong answer as expected rows next to yours with the first
  difference marked (an arrow, bold and a screen-reader note besides the colour); a
  column mismatch is named first. The library filters by "SQL".
- **kb-pack:** SQL packages (`schema.sql`, `ref.sql`, `wrong.sql`, `brute.py` with
  `solve(tables)`, `validate.py`, `gen.py`; nothing in other languages; no profile).
  `validate --run` checks the validator, `ref.sql` through the real harness (Accepted,
  within half its per-test CPU cap), `wrong.sql` (Wrong Answer), and 500 generated
  inputs on which `ref.sql` agrees with `brute.py`. The stress test runs both in one
  sandboxed run each (`kb_judge::brute_rows`, `kb_judge::sql_rows`: plain sqlite3,
  read-only, for package tooling only). `generate` fills outputs from `ref.sql`, and
  `new --mode sql` scaffolds a package.
- **Patterns:** `sql-basics`, `sql-joins`, `sql-aggregation`, `sql-advanced`, last in
  the track; SQL problems use only these, and only SQL problems do.

## Consequences
- No new runtime, dependency or runner capability: the attack surface is SQLite's
  read-only path inside the existing sandbox.
- One Python start per test, about 50 ms each, like program problems (ADR 0028).
