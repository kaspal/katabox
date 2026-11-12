def validate(inp):
    r = inp["readings"]
    if not isinstance(r, list) or not all(type(x) is int for x in r):
        return "readings is an array of integers"
    if not 1 <= len(r) <= 10**5:
        return "1 ≤ readings.length ≤ 10⁵"
    if not all(-10**4 <= x <= 10**4 for x in r):
        return "−10⁴ ≤ readings[i] ≤ 10⁴"
    return None
