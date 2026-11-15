"""Late returns in plain Python (no SQL): the expected rows for the stress test."""
from datetime import date


def solve(tables):
    names = {mid: name for mid, name in tables["members"]}
    late = []
    for loan_id, member_id, title, due, returned in tables["loans"]:
        if returned is None:
            continue
        days = (date.fromisoformat(returned) - date.fromisoformat(due)).days
        if days > 0:
            late.append((-days, title, loan_id, names[member_id]))
    late.sort()
    return {
        "columns": ["name", "title", "days_late"],
        "rows": [[name, title, -neg] for neg, title, _, name in late],
    }
