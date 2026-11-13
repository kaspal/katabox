def validate(inp):
    p = inp["panels"]
    if not isinstance(p, list) or not 1 <= len(p) <= 10**5:
        return "1 ≤ panels.length ≤ 10⁵"
    if not all(type(v) is int and 0 <= v <= 10**4 for v in p):
        return "0 ≤ panels[i] ≤ 10⁴"
    return None
