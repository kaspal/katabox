"""best-week--f1 constraints."""


def validate(inp):
    p, k = inp["profits"], inp["k"]
    if not isinstance(p, list) or not all(type(x) is int for x in p) or type(k) is not int:
        return "profits is an array of integers; k is an integer"
    if not 1 <= k <= len(p) <= 2 * 10**5:
        return "1 ≤ k ≤ profits.length ≤ 2·10⁵"
    if not all(-(10**4) <= x <= 10**4 for x in p):
        return "−10⁴ ≤ profits[i] ≤ 10⁴"
    return None
