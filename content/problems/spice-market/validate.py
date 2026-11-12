def validate(inp):
    weights, prices, capacity = inp["weights"], inp["prices"], inp["capacity"]
    for arr, name in ((weights, "weights"), (prices, "prices")):
        if not isinstance(arr, list) or not all(type(x) is int and 1 <= x <= 10**4 for x in arr):
            return "1 ≤ " + name + "[i] ≤ 10⁴"
    if not 1 <= len(weights) <= 10**5 or len(prices) != len(weights):
        return "1 ≤ weights.length ≤ 10⁵ and prices.length = weights.length"
    if type(capacity) is not int or not 1 <= capacity <= 10**9:
        return "1 ≤ capacity ≤ 10⁹"
    return None
