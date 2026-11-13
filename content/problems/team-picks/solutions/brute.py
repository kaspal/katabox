from math import comb


def team_picks(queries: list[list[int]]) -> list[int]:
    # Obviously correct: exact binomial coefficients, then the remainder.
    return [comb(n, k) % 1_000_003 for n, k in queries]
