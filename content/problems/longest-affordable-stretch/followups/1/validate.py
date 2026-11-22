"""longest-affordable-stretch--f1 constraints."""


def validate(inp):
    c, b = inp["costs"], inp["budget"]
    if not isinstance(c, list) or not all(type(x) is int for x in c) or type(b) is not int:
        return "costs is an array of integers; budget is an integer"
    if not 1 <= len(c) <= 5 * 10**4:
        return "1 ≤ costs.length ≤ 5·10⁴"
    if not all(-(10**4) <= x <= 10**4 for x in c):
        return "−10⁴ ≤ costs[i] ≤ 10⁴"
    if not -(10**9) <= b <= 10**9:
        return "−10⁹ ≤ budget ≤ 10⁹"
    return None
