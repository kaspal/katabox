"""change-machine--f1 constraints."""


def validate(inp):
    coins, amount = inp["coins"], inp["amount"]
    if not isinstance(coins, list) or not all(type(c) is int for c in coins) or type(amount) is not int:
        return "coins is an array of integers; amount is an integer"
    if not 1 <= len(coins) <= 30 or len(set(coins)) != len(coins):
        return "1 ≤ coins.length ≤ 30, all different"
    if not all(1 <= c <= 10**4 for c in coins):
        return "1 ≤ coins[i] ≤ 10⁴"
    if not 0 <= amount <= 2 * 10**4:
        return "0 ≤ amount ≤ 2·10⁴"
    return None
