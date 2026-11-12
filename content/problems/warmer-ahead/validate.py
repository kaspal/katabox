def validate(inp):
    t = inp["temps"]
    if not isinstance(t, list) or not all(type(x) is int for x in t):
        return "temps is an array of integers"
    if not 1 <= len(t) <= 10**5:
        return "1 ≤ temps.length ≤ 10⁵"
    if not all(-50 <= x <= 50 for x in t):
        return "−50 ≤ temps[i] ≤ 50"
    return None
