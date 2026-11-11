def range_totals(rain: list[int], queries: list[list[int]]) -> list[int]:
    # Obviously correct: add each range day by day.
    out = []
    for first, last in queries:
        total = 0
        for d in range(first, last + 1):
            total += rain[d]
        out.append(total)
    return out
