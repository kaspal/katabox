def validate(inp):
    g, q = inp["grid"], inp["queries"]
    if not isinstance(g, list) or not 1 <= len(g) <= 300:
        return "1 ≤ rows ≤ 300"
    c = len(g[0]) if isinstance(g[0], list) else 0
    if not 1 <= c <= 300:
        return "1 ≤ columns ≤ 300"
    for row in g:
        if not isinstance(row, list) or len(row) != c or not all(type(v) is int and 0 <= v <= 1000 for v in row):
            return "rows of equal length with 0 ≤ grid[r][c] ≤ 1000"
    if not isinstance(q, list) or not 1 <= len(q) <= 10**5:
        return "1 ≤ queries.length ≤ 10⁵"
    for x in q:
        if not isinstance(x, list) or len(x) != 4 or not all(type(v) is int for v in x):
            return "each query is [top, left, bottom, right]"
        t, l, b, r = x
        if not (0 <= t <= b < len(g) and 0 <= l <= r < c):
            return "0 ≤ top ≤ bottom < rows, 0 ≤ left ≤ right < columns"
    return None
