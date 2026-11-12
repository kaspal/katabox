from fractions import Fraction
from itertools import permutations


def best_haul(weights: list[int], prices: list[int], capacity: int) -> float:
    # Obviously correct for small inputs: fill the bag in every order of the stalls
    # (whole sacks while they fit, then a part), with exact fractions, and keep the best.
    best = Fraction(0)
    for order in permutations(range(len(weights))):
        value, room = Fraction(0), capacity
        for i in order:
            take = min(weights[i], room)
            value += Fraction(prices[i] * take, weights[i])
            room -= take
        best = max(best, value)
    return float(best)
