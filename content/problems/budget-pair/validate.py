def validate(inp):
    prices, budget = inp["prices"], inp["budget"]
    if not isinstance(prices, list) or not all(type(p) is int for p in prices) or type(budget) is not int:
        return "prices is an array of integers; budget is an integer"
    if not 2 <= len(prices) <= 5 * 10**4:
        return "2 ≤ prices.length ≤ 5·10⁴"
    if not all(1 <= p <= 10**6 for p in prices):
        return "1 ≤ prices[i] ≤ 10⁶"
    if any(prices[i] > prices[i + 1] for i in range(len(prices) - 1)):
        return "prices is sorted in non-decreasing order"
    if not 1 <= budget <= 2 * 10**6:
        return "1 ≤ budget ≤ 2·10⁶"
    return None
