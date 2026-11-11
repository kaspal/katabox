def validate(inp):
    rain, queries = inp["rain"], inp["queries"]
    if not isinstance(rain, list) or not all(type(r) is int for r in rain):
        return "rain is an array of integers"
    if not 1 <= len(rain) <= 5 * 10**4:
        return "1 ≤ rain.length ≤ 5·10⁴"
    if not all(0 <= r <= 10**4 for r in rain):
        return "0 ≤ rain[d] ≤ 10⁴"
    if not isinstance(queries, list) or not 1 <= len(queries) <= 5 * 10**4:
        return "1 ≤ queries.length ≤ 5·10⁴"
    for q in queries:
        if not (isinstance(q, list) and len(q) == 2 and all(type(x) is int for x in q)
                and 0 <= q[0] <= q[1] < len(rain)):
            return "queries[j] = [first, last] with 0 ≤ first ≤ last < rain.length"
    return None
