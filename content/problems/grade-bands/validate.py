def validate(inp):
    s = inp["scores"]
    if not isinstance(s, list) or not all(type(v) is int for v in s):
        return "scores is a list of integers"
    if not 1 <= len(s) <= 10**4:
        return "1 ≤ scores.length ≤ 10⁴"
    if not all(0 <= v <= 100 for v in s):
        return "0 ≤ scores[i] ≤ 100"
    return None
