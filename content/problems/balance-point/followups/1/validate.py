"""balance-point--f1 constraints."""


def validate(inp):
    w = inp["weights"]
    if not isinstance(w, list) or not all(type(x) is int for x in w):
        return "weights is an array of integers"
    if not 2 <= len(w) <= 10**5:
        return "2 ≤ weights.length ≤ 10⁵"
    if not all(-(10**4) <= x <= 10**4 for x in w):
        return "−10⁴ ≤ weights[i] ≤ 10⁴"
    return None
