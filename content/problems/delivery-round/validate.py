def validate(inp):
    dist = inp["dist"]
    if not isinstance(dist, list) or not 1 <= len(dist) <= 12:
        return "1 ≤ n ≤ 12"
    n = len(dist)
    for i, row in enumerate(dist):
        if not isinstance(row, list) or len(row) != n or not all(type(x) is int for x in row):
            return "dist is an n × n array of integers"
        for j, x in enumerate(row):
            if i == j and x != 0:
                return "dist[i][i] = 0"
            if i != j and not 1 <= x <= 10**4:
                return "1 ≤ dist[i][j] ≤ 10⁴ for i ≠ j"
    return None
