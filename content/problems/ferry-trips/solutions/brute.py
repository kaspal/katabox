from functools import lru_cache


def min_trips(weights: list[int], limit: int) -> int:
    # Obviously correct: try every way to split the hikers into trips of one or two.
    w = tuple(sorted(weights))

    @lru_cache(maxsize=None)
    def best(rest):
        if not rest:
            return 0
        first, others = rest[0], rest[1:]
        result = 1 + best(others)  # first crosses alone
        for i, x in enumerate(others):
            if first + x <= limit:
                result = min(result, 1 + best(others[:i] + others[i + 1:]))
        return result

    return best(w)
