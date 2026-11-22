"""exact-baskets constraints."""


def validate(inp):
    w, k, target = inp["weights"], inp["k"], inp["target"]
    if not isinstance(w, list) or not 1 <= len(w) <= 12:
        return "1 ≤ weights.length ≤ 12"
    if not all(type(x) is int and 1 <= x <= 30 for x in w) or len(set(w)) != len(w):
        return "the weights are distinct, 1 ≤ weights[i] ≤ 30"
    if type(k) is not int or not 1 <= k <= len(w):
        return "1 ≤ k ≤ weights.length"
    if type(target) is not int or not 1 <= target <= 360:
        return "1 ≤ target ≤ 360"
    return None
