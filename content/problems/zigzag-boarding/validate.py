def is_list(v, lo, hi):
    return isinstance(v, list) and all(type(x) is int and lo <= x <= hi for x in v)


def validate(inp):
    q = inp["queue"]
    if not is_list(q, 1, 10**6):
        return "1 ≤ passenger number ≤ 10⁶"
    if len(q) > 10**5:
        return "0 ≤ number of passengers ≤ 10⁵"
    return None
