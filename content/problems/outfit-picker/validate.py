def validate(inp):
    prices, budget = inp["prices"], inp["budget"]
    if not isinstance(prices, list) or type(budget) is not int:
        return "prices is a 2-D array; budget is an integer"
    if not 1 <= len(prices) <= 6:
        return "1 ≤ prices.length ≤ 6"
    for g in prices:
        if not isinstance(g, list) or not 1 <= len(g) <= 6:
            return "1 ≤ prices[c].length ≤ 6"
        if not all(type(x) is int and 1 <= x <= 1000 for x in g):
            return "1 ≤ prices[c][i] ≤ 1000"
    if not 1 <= budget <= 6000:
        return "1 ≤ budget ≤ 6000"
    return None
