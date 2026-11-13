def validate(inp):
    p = inp["plays"]
    if not isinstance(p, list) or not 1 <= len(p) <= 10**5:
        return "1 ≤ plays.length ≤ 10⁵"
    if not all(type(v) is int and 1 <= v <= 10**9 for v in p):
        return "1 ≤ plays[i] ≤ 10⁹"
    return None
