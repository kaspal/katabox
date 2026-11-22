import sqlite3


def hash_join(a):
    n = a[0]
    left = [(a[1 + 2 * i], a[2 + 2 * i]) for i in range(n)]
    rest = a[1 + 2 * n :]
    right = [(rest[2 * i], rest[2 * i + 1]) for i in range(len(rest) // 2)]
    index = {}
    for key, b in right:
        index.setdefault(key, []).append(b)  # @step build
    out = []
    for key, x in left:
        matches = index.get(key, [])  # @step probe
        for b in matches:
            out.extend([key, x, b])  # @step match
    check_with_sqlite(left, right, out)
    return out


def check_with_sqlite(left, right, out):
    """SQLite runs the same join; its rows must be ours (L, then R order)."""
    db = sqlite3.connect(":memory:")
    db.execute("CREATE TABLE l (key INTEGER, a INTEGER)")
    db.execute("CREATE TABLE r (key INTEGER, b INTEGER)")
    db.executemany("INSERT INTO l VALUES (?, ?)", left)
    db.executemany("INSERT INTO r VALUES (?, ?)", right)
    rows = db.execute(
        "SELECT l.key, l.a, r.b FROM l JOIN r ON l.key = r.key"
        " ORDER BY l.rowid, r.rowid"
    ).fetchall()
    got = [v for row in rows for v in row]
    assert got == out, "SQLite gives a different join"
