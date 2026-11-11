def range_totals(rain: list[int], queries: list[list[int]]) -> list[int]:
    prefix = [0] * (len(rain) + 1)
    for d, r in enumerate(rain):
        prefix[d + 1] = prefix[d] + r
    return [prefix[last + 1] - prefix[first] for first, last in queries]
