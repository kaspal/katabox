def fleet_totals(miles: list[list[int]], queries: list[list[int]]) -> list[int]:
    rows, cols = len(miles), len(miles[0])
    p = [[0] * (cols + 1) for _ in range(rows + 1)]
    for r in range(rows):
        for c in range(cols):
            p[r + 1][c + 1] = miles[r][c] + p[r][c + 1] + p[r + 1][c] - p[r][c]
    return [p[v2 + 1][d2 + 1] - p[v1][d2 + 1] - p[v2 + 1][d1] + p[v1][d1] for v1, d1, v2, d2 in queries]
