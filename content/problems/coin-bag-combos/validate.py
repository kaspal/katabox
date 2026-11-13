def ways(coins, target):
    dp = [1] + [0] * target
    for c in coins:
        for t in range(c, target + 1):
            dp[t] += dp[t - c]
    return dp[target]


def validate(inp):
    coins, target = inp["coins"], inp["target"]
    if not isinstance(coins, list) or not all(type(c) is int for c in coins) or type(target) is not int:
        return "coins is a list of integers; target is an integer"
    if not 1 <= len(coins) <= 10 or len(set(coins)) != len(coins):
        return "1 ≤ coins.length ≤ 10, all values different"
    if not all(1 <= c <= 50 for c in coins):
        return "1 ≤ coins[i] ≤ 50"
    if not 1 <= target <= 60:
        return "1 ≤ target ≤ 60"
    if ways(coins, target) > 1000:
        return "There are at most 1000 ways to pay."
    return None
