def first_at_least(sorted: list[int], x: int) -> int:
    # Obviously correct: scan from the left.
    return next((i for i, v in enumerate(sorted) if v >= x), len(sorted))
