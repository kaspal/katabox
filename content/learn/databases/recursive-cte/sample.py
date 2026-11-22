import sqlite3


def chain_depths(a):
    n = len(a)
    depth = [None] * n
    frontier = []
    for i in range(n):
        if a[i] == -1:
            depth[i] = 0  # @step anchor
            frontier.append(i)
    while frontier:
        found = []
        for boss in frontier:
            for e in range(n):
                if a[e] == boss:
                    depth[e] = depth[boss] + 1  # @step join
                    found.append(e)
        frontier = found
    check_with_sqlite(a, depth)
    return depth


def check_with_sqlite(bosses, depth):
    """SQLite runs the same WITH RECURSIVE query; its depths must be ours."""
    db = sqlite3.connect(":memory:")
    db.execute("CREATE TABLE emp (id INTEGER PRIMARY KEY, boss INTEGER)")
    rows = [(i, None if b == -1 else b) for i, b in enumerate(bosses)]
    db.executemany("INSERT INTO emp VALUES (?, ?)", rows)
    result = db.execute(
        """WITH RECURSIVE chain(id, depth) AS (
             SELECT id, 0 FROM emp WHERE boss IS NULL
             UNION ALL
             SELECT e.id, c.depth + 1 FROM emp e JOIN chain c ON e.boss = c.id
           )
           SELECT depth FROM chain ORDER BY id"""
    ).fetchall()
    assert [row[0] for row in result] == depth, "SQLite's chain differs"
