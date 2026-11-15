"""Stations touched by at least three trips, in plain Python."""


def solve(tables):
    count = {}
    for _, a, b, _ in tables["trips"]:
        for s in {a, b}:
            count[s] = count.get(s, 0) + 1
    rows = sorted((-n, s) for s, n in count.items() if n >= 3)
    return {"columns": ["station", "trips"], "rows": [[s, -n] for n, s in rows]}
