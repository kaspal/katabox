def validate(inp):
    routes = inp["routes"]
    if not isinstance(routes, list) or not 1 <= len(routes) <= 10**4:
        return "1 ≤ routes.length ≤ 10⁴"
    for r in routes:
        if not isinstance(r, list) or len(r) != 6 or not all(type(v) is int for v in r):
            return "Each route has 6 integers with absolute value at most 10⁴."
        if not all(abs(v) <= 10**4 for v in r):
            return "Each route has 6 integers with absolute value at most 10⁴."
        if (r[0], r[1]) == (r[2], r[3]) or (r[2], r[3]) == (r[4], r[5]):
            return "A ≠ B and B ≠ C."
    return None
