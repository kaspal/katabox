from itertools import combinations_with_replacement


def coin_combos(coins: list[int], target: int) -> list[list[int]]:
    # Obviously correct: every multiset of coins of every possible size, filtered and sorted.
    smallest = min(coins)
    out = []
    for size in range(1, target // smallest + 1):
        for bag in combinations_with_replacement(sorted(coins), size):
            if sum(bag) == target:
                out.append(list(bag))
    return sorted(out)
