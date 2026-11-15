"""How many bosses each person has above them, in plain Python."""


def solve(tables):
    boss = {sid: b for sid, _, b in tables["staff"]}
    rows = []
    for sid, name, _ in tables["staff"]:
        depth, at = 0, boss[sid]
        while at is not None:
            depth, at = depth + 1, boss[at]
        rows.append((depth, name, sid))
    rows.sort()
    return {"columns": ["name", "depth"], "rows": [[name, d] for d, name, _ in rows]}
