def range_totals(rain: list[int], queries: list[list[int]]) -> list[int]:
    # Correct but O(n·q): each query re-sums its range.
    return [sum(rain[first:last + 1]) for first, last in queries]
