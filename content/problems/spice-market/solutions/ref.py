from functools import cmp_to_key


def best_haul(weights: list[int], prices: list[int], capacity: int) -> float:
    def richer_first(i: int, j: int) -> int:
        # prices[i] / weights[i] > prices[j] / weights[j], without rounding
        return prices[j] * weights[i] - prices[i] * weights[j]

    order = sorted(range(len(weights)), key=cmp_to_key(richer_first))
    whole = 0
    room = capacity
    for i in order:
        if weights[i] <= room:
            whole += prices[i]
            room -= weights[i]
        else:
            return whole + prices[i] * room / weights[i]
    return float(whole)
