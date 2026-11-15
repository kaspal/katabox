# KataBox SQL harness (ADR 0031). Runs one SQL query against one test's tables with the
# bundled Python's sqlite3, inside the sandbox, and prints one record:
#
#   <nonce>{"ok": true, "columns": [...], "rows": [[...], ...]}
#   <nonce>{"ok": false, "code": "<message code>", "phase": "compile" | "run", "message": "..."}
#
# stdin is JSON: {"nonce", "schema", "tables", "query", "max_rows", "deadline_ms"}. The
# schema has already been checked by the judge (kb_judge::sql::parse_schema), so it is
# safe to run; the user's query only ever runs after the connection is locked down:
# read-only (query_only), an authorizer that allows nothing but reading, SQLite's own
# limits, and a progress handler that stops the query at the deadline.
import json
import sqlite3
import sys
import time

DENIED_FUNCTIONS = {"load_extension", "random", "randomblob"}
ALLOWED_ACTIONS = {
    sqlite3.SQLITE_SELECT,
    sqlite3.SQLITE_READ,
    sqlite3.SQLITE_FUNCTION,
    getattr(sqlite3, "SQLITE_RECURSIVE", 33),
}
LIMITS = {
    "SQLITE_LIMIT_LENGTH": 1_000_000,
    "SQLITE_LIMIT_SQL_LENGTH": 100_000,
    "SQLITE_LIMIT_EXPR_DEPTH": 100,
    "SQLITE_LIMIT_COMPOUND_SELECT": 50,
    "SQLITE_LIMIT_VARIABLE_NUMBER": 1,
    "SQLITE_LIMIT_FUNCTION_ARG": 16,
    "SQLITE_LIMIT_VDBE_OP": 100_000,
    "SQLITE_LIMIT_ATTACHED": 0,
    "SQLITE_LIMIT_COLUMN": 200,
    "SQLITE_LIMIT_LIKE_PATTERN_LENGTH": 1000,
}


def main():
    data = json.loads(sys.stdin.buffer.read().decode("utf-8"))
    nonce = data["nonce"]

    def emit(record):
        sys.stdout.write(nonce + json.dumps(record, ensure_ascii=False, allow_nan=False) + "\n")
        sys.stdout.flush()

    def fail(code, phase, message):
        emit({"ok": False, "code": code, "phase": phase, "message": message[:2000]})

    conn = sqlite3.connect(":memory:", isolation_level=None)
    conn.executescript(data["schema"])
    for name, rows in data["tables"].items():
        if rows:
            marks = ", ".join("?" * len(rows[0]))
            conn.executemany(f'INSERT INTO "{name}" VALUES ({marks})', rows)

    # Lock down before the user's query is even parsed.
    conn.execute("PRAGMA query_only = ON")
    for name, value in LIMITS.items():
        category = getattr(sqlite3, name, None)
        if category is not None:
            conn.setlimit(category, value)

    def authorizer(action, arg1, arg2, db_name, trigger):
        if action == sqlite3.SQLITE_FUNCTION:
            return sqlite3.SQLITE_DENY if (arg2 or "").lower() in DENIED_FUNCTIONS else sqlite3.SQLITE_OK
        return sqlite3.SQLITE_OK if action in ALLOWED_ACTIONS else sqlite3.SQLITE_DENY

    conn.set_authorizer(authorizer)

    query = data["query"].strip()
    while query.endswith(";"):
        query = query[:-1].rstrip()
    head = first_word(query)
    if head is None:
        fail("sql_error", "compile", "the query is empty")
        return
    if head not in ("select", "with"):
        fail("sql_not_select", "compile", f"the query must be one SELECT (or WITH … SELECT), not {head.upper()}")
        return

    # Compile without running: EXPLAIN prepares the statement (names, syntax, the
    # authorizer) and returns its bytecode, which is thrown away.
    try:
        conn.execute("EXPLAIN " + query).fetchall()
    except sqlite3.ProgrammingError as e:
        if "one statement" in str(e):
            fail("sql_multiple_statements", "compile", "send one statement only")
        else:
            fail("sql_error", "compile", str(e))
        return
    except sqlite3.DatabaseError as e:
        text = str(e)
        if "not authorized" in text or "prohibited" in text:
            fail("sql_not_authorized", "compile", text)
        else:
            fail("sql_error", "compile", text)
        return

    deadline = time.process_time() + data["deadline_ms"] / 1000.0
    conn.set_progress_handler(lambda: 1 if time.process_time() > deadline else 0, 1000)
    max_rows = data["max_rows"]
    try:
        cursor = conn.execute(query)
        columns = [d[0] for d in cursor.description or []]
        rows = cursor.fetchmany(max_rows + 1)
    except sqlite3.OperationalError as e:
        if "interrupted" in str(e):
            fail("time_limit", "run", "the query ran out of time")
        elif "not authorized" in str(e):
            fail("sql_not_authorized", "run", str(e))
        else:
            fail("sql_error", "run", str(e))
        return
    except sqlite3.DatabaseError as e:
        if "not authorized" in str(e):
            fail("sql_not_authorized", "run", str(e))
        else:
            fail("sql_error", "run", str(e))
        return
    if len(rows) > max_rows:
        fail("sql_too_many_rows", "run", f"the query returned more than {max_rows} rows")
        return
    out = []
    for row in rows:
        values = []
        for v in row:
            if isinstance(v, (bytes, bytearray, memoryview)):
                fail("sql_error", "run", "a value is a blob: return numbers, text or NULL")
                return
            if isinstance(v, float) and (v != v or v in (float("inf"), float("-inf"))):
                fail("sql_error", "run", "a value is not a finite number")
                return
            values.append(v)
        out.append(values)
    emit({"ok": True, "columns": columns, "rows": out})


def first_word(query):
    """The query's first keyword, lowercase, after comments; None when there is none."""
    i, n = 0, len(query)
    while i < n:
        if query[i].isspace():
            i += 1
        elif query.startswith("--", i):
            j = query.find("\n", i)
            i = n if j < 0 else j + 1
        elif query.startswith("/*", i):
            j = query.find("*/", i + 2)
            i = n if j < 0 else j + 2
        else:
            break
    j = i
    while j < n and (query[j].isalpha() or query[j] == "_"):
        j += 1
    return query[i:j].lower() or None


main()
