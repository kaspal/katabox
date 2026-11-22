"""top-scores--f2 constraints."""


def validate(inp):
    s, w, k = inp["scores"], inp["w"], inp["k"]
    if not isinstance(s, list) or not all(type(x) is int for x in s) or type(w) is not int or type(k) is not int:
        return "scores is an array of integers; w and k are integers"
    if not 1 <= k <= w <= len(s) <= 10**5:
        return "1 ≤ k ≤ w ≤ scores.length ≤ 10⁵"
    if not all(0 <= x <= 300 for x in s):
        return "0 ≤ scores[i] ≤ 300"
    return None
