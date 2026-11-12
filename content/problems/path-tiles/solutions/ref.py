MOD = 1_000_000_007


def tilings(n: int) -> int:
    a, b, c = 0, 0, 1  # ways for lengths i − 3, i − 2, i − 1 (starting at i = 1)
    for _ in range(n):
        a, b, c = b, c, (a + b + c) % MOD
    return c
