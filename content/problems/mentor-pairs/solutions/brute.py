"""Each staff member with their mentor's name, in plain Python."""


def solve(tables):
    names = {sid: name for sid, name, _ in tables["staff"]}
    rows = sorted((name, sid, names.get(mentor) if mentor is not None else None)
                  for sid, name, mentor in tables["staff"])
    return {"columns": ["name", "mentor"], "rows": [[name, mentor] for name, _, mentor in rows]}
