def span_sums(rain: list[int], queries: list[list[int]]) -> list[int]:
    # Obviously correct: add up each span day by day.
    return [sum(rain[l : r + 1]) for l, r in queries]
