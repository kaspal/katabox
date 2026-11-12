def validate(inp):
    ids = inp["ids"]
    if not isinstance(ids, list) or not all(type(x) is int for x in ids):
        return "ids is an array of integers"
    if not 1 <= len(ids) <= 10**5 or len(ids) % 2 == 0:
        return "1 ≤ ids.length ≤ 10⁵ (always odd)"
    if not all(1 <= x <= 10**9 for x in ids):
        return "1 ≤ ids[i] ≤ 10⁹"
    counts = {}
    for x in ids:
        counts[x] = counts.get(x, 0) + 1
    if sorted(counts.values()) != [1] + [2] * (len(counts) - 1):
        return "Every value appears exactly twice, except one value that appears once."
    return None
