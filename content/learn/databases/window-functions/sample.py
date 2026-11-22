import sqlite3


def running_totals(a):
    rows = [(a[2 * i], a[2 * i + 1]) for i in range(len(a) // 2)]
    running = {}
    out = []
    for p, v in rows:
        if p not in running:
            running[p] = v  # @step start
        else:
            running[p] += v  # @step add
        out.append(running[p])
    check_with_sqlite(rows, out)
    return out


def check_with_sqlite(rows, out):
    """SQLite computes the same window function; its totals must be ours."""
    db = sqlite3.connect(":memory:")
    db.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, p INTEGER, v INTEGER)")
    db.executemany("INSERT INTO t (p, v) VALUES (?, ?)", rows)
    result = db.execute(
        "SELECT SUM(v) OVER (PARTITION BY p ORDER BY id"
        " ROWS UNBOUNDED PRECEDING) FROM t ORDER BY id"
    ).fetchall()
    assert [row[0] for row in result] == out, "SQLite's running totals differ"
