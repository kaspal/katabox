def validate(inp):
    n, edges = inp["n"], inp["rivalries"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(edges, list) or len(edges) > 2 * 10**5:
        return "0 ≤ rivalries.length ≤ 2·10⁵"
    seen = set()
    for e in edges:
        if not isinstance(e, list) or len(e) != 2 or not all(type(v) is int for v in e):
            return "each rivalry is [a, b]"
        a, b = e
        key = (min(a, b), max(a, b))
        if not (0 <= a < n and 0 <= b < n) or a == b or key in seen:
            return "0 ≤ a, b < n, a ≠ b, and no pair is listed twice."
        seen.add(key)
    return None
