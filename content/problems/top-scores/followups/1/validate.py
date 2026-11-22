"""top-scores--f1 constraints."""


def validate(inp):
    s, k = inp["scores"], inp["k"]
    if not isinstance(s, list) or not all(type(x) is int for x in s) or type(k) is not int:
        return "scores is an array of integers; k is an integer"
    if not 1 <= k <= len(s) <= 10**5:
        return "1 ≤ k ≤ scores.length ≤ 10⁵"
    if not all(0 <= x <= 300 for x in s):
        return "0 ≤ scores[i] ≤ 300"
    return None
