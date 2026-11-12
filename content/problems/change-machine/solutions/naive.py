def fewest_coins(coins: list[int], amount: int) -> int:
    # Correct but exponential: try every count of every token.
    def search(i, rest):
        if rest == 0:
            return 0
        if i == len(coins):
            return None
        best = None
        for count in range(rest // coins[i] + 1):
            sub = search(i + 1, rest - count * coins[i])
            if sub is not None and (best is None or count + sub < best):
                best = count + sub
        return best

    result = search(0, amount)
    return -1 if result is None else result
