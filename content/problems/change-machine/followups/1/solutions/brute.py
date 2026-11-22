def count_ways(coins: list[int], amount: int) -> int:
    # Obviously correct for small amounts: choose how many of each token, in order.
    def ways(i: int, left: int) -> int:
        if i == len(coins):
            return 1 if left == 0 else 0
        return sum(ways(i + 1, left - k * coins[i]) for k in range(left // coins[i] + 1))

    return ways(0, amount) % 1_000_000_007
