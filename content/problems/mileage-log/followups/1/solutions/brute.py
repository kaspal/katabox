def fleet_totals(miles: list[list[int]], queries: list[list[int]]) -> list[int]:
    # Obviously correct: add up every cell of every rectangle.
    return [sum(miles[v][d] for v in range(v1, v2 + 1) for d in range(d1, d2 + 1)) for v1, d1, v2, d2 in queries]
