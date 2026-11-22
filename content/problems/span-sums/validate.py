"""span-sums constraints."""


def validate(inp):
    rain, qs = inp["rain"], inp["queries"]
    if not isinstance(rain, list) or not 1 <= len(rain) <= 10**5:
        return "1 ≤ rain.length ≤ 10⁵"
    if not all(type(x) is int and 0 <= x <= 1000 for x in rain):
        return "0 ≤ rain[i] ≤ 1000"
    if not isinstance(qs, list) or not 1 <= len(qs) <= 10**4:
        return "1 ≤ queries.length ≤ 10⁴"
    for q in qs:
        if not isinstance(q, list) or len(q) != 2 or not all(type(x) is int for x in q):
            return "each query is [l, r]"
        if not 0 <= q[0] <= q[1] < len(rain):
            return "0 ≤ l ≤ r < rain.length"
    return None
