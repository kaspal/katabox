def long_days(miles: list[int], trips: list[list[int]]) -> list[int]:
    # Obviously correct: count day by day.
    return [sum(1 for d in range(a, b + 1) if miles[d] >= x) for a, b, x in trips]
