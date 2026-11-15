import re

NAME = re.compile(r"[A-Z][a-z]{2,11}")


def validate(inp):
    teams = inp["tables"]["teams"]
    if not 1 <= len(teams) <= 2000:
        return "1 ≤ teams ≤ 2000"
    ids = set()
    for tid, name, div, points in teams:
        if not 1 <= tid <= 10**6 or tid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(tid)
        if not NAME.fullmatch(name) or div not in ("A", "B", "C", "D"):
            return "a name is one capitalised word; division is A, B, C or D"
        if not 0 <= points <= 300:
            return "0 ≤ points ≤ 300"
    return None
