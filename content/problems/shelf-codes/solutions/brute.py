def count_below(codes: list[int], queries: list[int]) -> list[int]:
    # Obviously correct: count the smaller numbers.
    return [sum(1 for c in codes if c < x) for x in queries]
