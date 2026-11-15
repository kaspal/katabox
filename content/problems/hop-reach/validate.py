import re

PIER = re.compile(r"[A-Z][a-z]{2,14}")


def validate(inp):
    routes = inp["tables"]["routes"]
    if not 1 <= len(routes) <= 400:
        return "1 ≤ routes ≤ 400"
    ids, pairs = set(), set()
    for rid, a, b in routes:
        if not 1 <= rid <= 10**6 or rid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(rid)
        if not PIER.fullmatch(a) or not PIER.fullmatch(b) or a == b:
            return "a route joins two different piers, each one capitalised word"
        if (a, b) in pairs:
            return "each one-way connection appears once"
        pairs.add((a, b))
    return None
