def validate(inp):
    r, c, b = inp["rows"], inp["cols"], inp["blocked"]
    if type(r) is not int or type(c) is not int or not (1 <= r <= 10**5 and 1 <= c <= 10**5):
        return "1 ≤ rows, cols ≤ 10⁵"
    if not isinstance(b, list) or len(b) > 1000:
        return "0 ≤ blocked.length ≤ 1000"
    seen = set()
    for x in b:
        if not isinstance(x, list) or len(x) != 2 or not all(type(v) is int for v in x):
            return "each blocked crossing is [r, c]"
        if not (0 <= x[0] <= r and 0 <= x[1] <= c) or tuple(x) in ((0, 0), (r, c)) or tuple(x) in seen:
            return "blocked crossings are inside the grid, different, and not the start or the end"
        seen.add(tuple(x))
    return None
