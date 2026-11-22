"""cheapest-k constraints."""


def validate(inp):
    p, k = inp["prices"], inp["k"]
    if not isinstance(p, list) or not 1 <= len(p) <= 10**5:
        return "1 ≤ prices.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**4 for x in p):
        return "1 ≤ prices[i] ≤ 10⁴"
    if type(k) is not int or not 1 <= k <= len(p):
        return "1 ≤ k ≤ prices.length"
    return None
