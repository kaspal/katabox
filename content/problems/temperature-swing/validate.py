def validate(inp):
    r = inp["readings"]
    if not isinstance(r, list) or not all(type(v) is int for v in r):
        return "readings is a list of integers"
    if not 1 <= len(r) <= 10**5:
        return "1 ≤ readings.length ≤ 10⁵"
    if not all(-100 <= v <= 100 for v in r):
        return "-100 ≤ readings[i] ≤ 100"
    return None
