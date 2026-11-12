from collections import Counter


def two_lonely(ids: list[int]) -> list[int]:
    # Obviously correct: count every id.
    return sorted(x for x, c in Counter(ids).items() if c == 1)
