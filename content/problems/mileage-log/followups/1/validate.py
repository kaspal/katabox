"""mileage-log--f1 constraints."""


def validate(inp):
    m, q = inp["miles"], inp["queries"]
    if not isinstance(m, list) or not m or not all(isinstance(r, list) and r for r in m):
        return "miles is a non-empty grid"
    rows, cols = len(m), len(m[0])
    if any(len(r) != cols for r in m) or rows * cols > 10**5:
        return "every van has the same number of days; vans × days ≤ 10⁵"
    if not all(type(x) is int and 0 <= x <= 1000 for r in m for x in r):
        return "0 ≤ miles[v][d] ≤ 1000"
    if not isinstance(q, list) or not 1 <= len(q) <= 10**5:
        return "1 ≤ queries.length ≤ 10⁵"
    for x in q:
        if not (isinstance(x, list) and len(x) == 4 and all(type(v) is int for v in x)):
            return "queries[i] = [v1, d1, v2, d2]"
        v1, d1, v2, d2 = x
        if not (0 <= v1 <= v2 < rows and 0 <= d1 <= d2 < cols):
            return "0 ≤ v1 ≤ v2 < vans and 0 ≤ d1 ≤ d2 < days"
    return None
