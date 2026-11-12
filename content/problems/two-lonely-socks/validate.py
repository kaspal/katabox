from collections import Counter


def validate(inp):
    ids = inp["ids"]
    if not isinstance(ids, list) or not all(type(x) is int for x in ids):
        return "ids is an array of integers"
    if not 2 <= len(ids) <= 10**5 + 2:
        return "2 ≤ ids.length ≤ 10⁵ + 2"
    if not all(1 <= x <= 10**9 for x in ids):
        return "1 ≤ ids[i] ≤ 10⁹"
    counts = sorted(Counter(ids).values())
    if counts.count(1) != 2 or any(c not in (1, 2) for c in counts):
        return "every number twice, except two numbers once"
    return None
