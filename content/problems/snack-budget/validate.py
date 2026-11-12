def validate(inp):
    prices, budget = inp["prices"], inp["budget"]
    if not isinstance(prices, list) or not all(type(x) is int for x in prices) or type(budget) is not int:
        return "prices is an array of integers; budget is an integer"
    if not 1 <= len(prices) <= 10**5:
        return "1 ≤ prices.length ≤ 10⁵"
    if not all(1 <= x <= 10**4 for x in prices):
        return "1 ≤ prices[i] ≤ 10⁴"
    if not 0 <= budget <= 10**9:
        return "0 ≤ budget ≤ 10⁹"
    return None
