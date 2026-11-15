import sqlite3


def group_by(a):
    rows = [(a[2 * i], a[2 * i + 1]) for i in range(len(a) // 2)]
    groups = {}
    for g, v in rows:
        count, total = groups.get(g, (0, 0))
        groups[g] = (count + 1, total + v)  # @step add
    out = []
    for g in sorted(groups):
        out.extend([g, *groups[g]])  # @step emit
    check_with_sqlite(rows, out)
    return out


def check_with_sqlite(rows, out):
    """SQLite runs the same GROUP BY; its rows must be ours."""
    db = sqlite3.connect(":memory:")
    db.execute("CREATE TABLE t (g INTEGER, v INTEGER)")
    db.executemany("INSERT INTO t VALUES (?, ?)", rows)
    result = db.execute("SELECT g, COUNT(*), SUM(v) FROM t GROUP BY g ORDER BY g").fetchall()
    assert [x for row in result for x in row] == out, "SQLite groups differently"
