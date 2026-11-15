"""Distinct unordered city pairs, in plain Python."""


def solve(tables):
    pairs = set()
    for _, origin, dest in tables["flights"]:
        a, b = origin.strip(" ").lower(), dest.strip(" ").lower()
        if a != b:
            pairs.add((min(a, b), max(a, b)))
    return {"columns": ["city_a", "city_b"], "rows": [list(p) for p in sorted(pairs)]}
