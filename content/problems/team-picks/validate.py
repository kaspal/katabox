def validate(inp):
    q = inp["queries"]
    if not isinstance(q, list) or not 1 <= len(q) <= 10**5:
        return "1 ≤ queries.length ≤ 10⁵"
    for x in q:
        if not isinstance(x, list) or len(x) != 2 or not all(type(v) is int and 0 <= v <= 10**5 for v in x):
            return "each query is [n, k] with 0 ≤ n, k ≤ 10⁵"
    return None
