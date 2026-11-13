def validate(inp):
    g, t = inp["seats"], inp["ticket"]
    if not isinstance(g, list) or not 1 <= len(g) <= 300 or type(t) is not int or not 0 <= t <= 10**9:
        return "1 ≤ rows ≤ 300; 0 ≤ ticket ≤ 10⁹"
    c = len(g[0]) if isinstance(g[0], list) else 0
    if not 1 <= c <= 300:
        return "1 ≤ columns ≤ 300"
    for row in g:
        if not isinstance(row, list) or len(row) != c or not all(type(v) is int and 0 <= v <= 10**9 for v in row):
            return "rows of equal length with 0 ≤ seats[r][c] ≤ 10⁹"
    for i in range(len(g)):
        for j in range(c):
            if (j and g[i][j - 1] >= g[i][j]) or (i and g[i - 1][j] >= g[i][j]):
                return "rows and columns are strictly increasing"
    return None
