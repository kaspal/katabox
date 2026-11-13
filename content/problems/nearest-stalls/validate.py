def validate(inp):
    stalls, k = inp["stalls"], inp["k"]
    if not isinstance(stalls, list) or not all(isinstance(s, list) and len(s) == 2 and all(type(v) is int for v in s) for s in stalls):
        return "stalls is a list of [x, y] pairs"
    if type(k) is not int or not 1 <= k <= len(stalls) <= 10**5:
        return "1 ≤ k ≤ stalls.length ≤ 10⁵"
    if not all(abs(v) <= 10**4 for s in stalls for v in s):
        return "All coordinates are integers with absolute value at most 10⁴."
    return None
