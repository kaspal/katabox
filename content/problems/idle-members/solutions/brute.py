"""Members with no visit in November 2026, in plain Python."""


def solve(tables):
    came = {mid for _, mid, day in tables["visits"] if day.startswith("2026-11-")}
    rows = sorted((name, mid) for mid, name in tables["members"] if mid not in came)
    return {"columns": ["id", "name"], "rows": [[mid, name] for name, mid in rows]}
