INF = 999


def coin_change(amount, coins):
    fewest = [0] + [INF] * amount
    for total in range(1, amount + 1):
        for coin in coins:
            if coin <= total and fewest[total - coin] + 1 < fewest[total]:
                fewest[total] = fewest[total - coin] + 1  # @step improve
    return [fewest[amount] if fewest[amount] < INF else -1]
