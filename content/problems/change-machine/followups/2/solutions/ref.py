def fewest_tokens(tokens: list[int], amount: int) -> int:
    inf = len(tokens) + 1
    best = [inf] * (amount + 1)
    best[0] = 0
    for t in tokens:
        for a in range(amount, t - 1, -1):
            if best[a - t] + 1 < best[a]:
                best[a] = best[a - t] + 1
    return best[amount] if best[amount] < inf else -1
