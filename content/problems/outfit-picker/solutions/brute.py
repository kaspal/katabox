from itertools import product


def outfits(prices: list[list[int]], budget: int) -> int:
    # Obviously correct: list every outfit.
    return sum(1 for outfit in product(*prices) if sum(outfit) <= budget)
