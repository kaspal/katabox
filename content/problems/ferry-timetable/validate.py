def validate(inp):
    n, routes, queries = inp["n"], inp["routes"], inp["queries"]
    if type(n) is not int or not 1 <= n <= 100:
        return "1 ≤ n ≤ 100"
    if not isinstance(routes, list) or len(routes) > 5000:
        return "0 ≤ routes.length ≤ 5000, with 0 ≤ a, b < n, a ≠ b and 1 ≤ minutes ≤ 10⁴"
    for r in routes:
        if not isinstance(r, list) or len(r) != 3 or not all(type(v) is int for v in r):
            return "0 ≤ routes.length ≤ 5000, with 0 ≤ a, b < n, a ≠ b and 1 ≤ minutes ≤ 10⁴"
        a, b, w = r
        if not (0 <= a < n and 0 <= b < n and a != b and 1 <= w <= 10**4):
            return "0 ≤ routes.length ≤ 5000, with 0 ≤ a, b < n, a ≠ b and 1 ≤ minutes ≤ 10⁴"
    if not isinstance(queries, list) or not 1 <= len(queries) <= 10**4:
        return "1 ≤ queries.length ≤ 10⁴, with 0 ≤ from, to < n"
    for qy in queries:
        if not isinstance(qy, list) or len(qy) != 2 or not all(type(v) is int and 0 <= v < n for v in qy):
            return "1 ≤ queries.length ≤ 10⁴, with 0 ≤ from, to < n"
    return None
