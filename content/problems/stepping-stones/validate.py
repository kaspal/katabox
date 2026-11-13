def validate(inp):
    j = inp["jumps"]
    if not isinstance(j, list) or not all(type(v) is int for v in j):
        return "jumps is a list of integers"
    if not 1 <= len(j) <= 10**5:
        return "1 ≤ jumps.length ≤ 10⁵"
    if not all(0 <= v <= 10**5 for v in j):
        return "0 ≤ jumps[i] ≤ 10⁵"
    return None
