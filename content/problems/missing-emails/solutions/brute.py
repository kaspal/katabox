"""Readers without a usable email, in plain Python."""


def solve(tables):
    rows = [(city, name, rid) for rid, name, email, city in tables["readers"]
            if email is None or email.strip(" ") == ""]
    rows.sort()
    return {"columns": ["id", "name", "city"], "rows": [[rid, name, city] for city, name, rid in rows]}
