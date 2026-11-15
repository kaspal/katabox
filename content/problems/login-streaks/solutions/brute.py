"""Each user's longest run of consecutive sign-in days, in plain Python."""
from datetime import date


def solve(tables):
    days = {}
    for _, user, day in tables["logins"]:
        days.setdefault(user, set()).add(date.fromisoformat(day).toordinal())
    rows = []
    for user, ds in days.items():
        best = 0
        for d in ds:
            if d - 1 not in ds:
                n = 1
                while d + n in ds:
                    n += 1
                best = max(best, n)
        rows.append((-best, user))
    rows.sort()
    return {"columns": ["username", "longest"], "rows": [[u, -b] for b, u in rows]}
