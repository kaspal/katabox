def fewest_coins(coins: list[int], amount: int) -> int:
    inf = amount + 1
    best = [0] + [inf] * amount
    for x in range(1, amount + 1):
        b = inf
        for c in coins:
            if c <= x and best[x - c] + 1 < b:
                b = best[x - c] + 1
        best[x] = b
    return best[amount] if best[amount] <= amount else -1
