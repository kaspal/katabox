from functools import cache

MOD = 1_000_000_007


def grid_routes(floor: list[str]) -> int:
    rows, cols = len(floor), len(floor[0])

    @cache
    def routes(r: int, c: int) -> int:
        if r >= rows or c >= cols or floor[r][c] == "#":
            return 0
        if r == rows - 1 and c == cols - 1:
            return 1
        return (routes(r + 1, c) + routes(r, c + 1)) % MOD

    return routes(0, 0)
