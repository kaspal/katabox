def validate(inp):
    prices, target = inp["prices"], inp["target"]
    if not isinstance(prices, list) or not all(type(x) is int for x in prices) or type(target) is not int:
        return "prices is an array of integers; target is an integer"
    if not 3 <= len(prices) <= 2000:
        return "3 ≤ prices.length ≤ 2000"
    if not all(1 <= x <= 10**6 for x in prices):
        return "1 ≤ prices[i] ≤ 10⁶"
    if not 1 <= target <= 3 * 10**6:
        return "1 ≤ target ≤ 3·10⁶"
    return None
