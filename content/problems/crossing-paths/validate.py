def validate(inp):
    pairs = inp["pairs"]
    if not isinstance(pairs, list) or not 1 <= len(pairs) <= 10**4:
        return "1 ≤ pairs.length ≤ 10⁴"
    for p in pairs:
        if not isinstance(p, list) or len(p) != 8 or not all(type(v) is int and abs(v) <= 10**4 for v in p):
            return "All coordinates are integers with absolute value at most 10⁴."
        if p[0:2] == p[2:4] or p[4:6] == p[6:8]:
            return "Each path has two different endpoints."
    return None
