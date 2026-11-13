def validate(inp):
    n, roads, start = inp["n"], inp["roads"], inp["start"]
    if type(n) is not int or not 1 <= n <= 500:
        return "1 ≤ n ≤ 500"
    if not isinstance(roads, list) or len(roads) > 3000:
        return "0 ≤ roads.length ≤ 3000, with 0 ≤ a, b < n and a ≠ b"
    for r in roads:
        if not isinstance(r, list) or len(r) != 3 or not all(type(v) is int for v in r):
            return "0 ≤ roads.length ≤ 3000, with 0 ≤ a, b < n and a ≠ b"
        a, b, c = r
        if not (0 <= a < n and 0 <= b < n and a != b):
            return "0 ≤ roads.length ≤ 3000, with 0 ≤ a, b < n and a ≠ b"
        if not -1000 <= c <= 1000:
            return "-1000 ≤ cost ≤ 1000"
    if type(start) is not int or not 0 <= start < n:
        return "0 ≤ start < n"
    return None
