MOD = 1_000_000_007


def grid_routes(floor: list[str]) -> int:
    rows, cols = len(floor), len(floor[0])
    memo = {}

    def routes(r: int, c: int) -> int:
        if r >= rows or c >= cols or floor[r][c] == "#":
            return 0
        if r == rows - 1 and c == cols - 1:
            return 1
        if r + c in memo:
            return memo[r + c]
        memo[r + c] = (routes(r + 1, c) + routes(r, c + 1)) % MOD
        return memo[r + c]

    return routes(0, 0)
