from itertools import combinations


def menu_combos(prices: list[int], budget: int) -> list[list[int]]:
    # Obviously correct: every combination of every size, then sorted.
    n = len(prices)
    out = [list(c) for k in range(1, n + 1) for c in combinations(range(n), k)
           if sum(prices[i] for i in c) == budget]
    return sorted(out)
