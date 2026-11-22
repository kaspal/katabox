"""distinct-pairs constraints."""


def validate(inp):
    p, budget = inp["prices"], inp["budget"]
    if not isinstance(p, list) or not 1 <= len(p) <= 10**5:
        return "1 ≤ prices.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**4 for x in p):
        return "1 ≤ prices[i] ≤ 10⁴"
    if any(p[i] > p[i + 1] for i in range(len(p) - 1)):
        return "prices is sorted in non-decreasing order"
    if type(budget) is not int or not 2 <= budget <= 2 * 10**4:
        return "2 ≤ budget ≤ 2·10⁴"
    return None
