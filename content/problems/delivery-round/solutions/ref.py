from functools import cache


def shortest_round(dist: list[list[int]]) -> int:
    n = len(dist)
    full = (1 << n) - 1

    @cache
    def best(mask: int, last: int) -> int:
        if mask == full:
            return dist[last][0]
        return min(dist[last][j] + best(mask | 1 << j, j) for j in range(n) if not mask >> j & 1)

    return best(1, 0)
