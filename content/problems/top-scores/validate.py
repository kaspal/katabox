def validate(inp):
    scores, k = inp["scores"], inp["k"]
    if not isinstance(scores, list) or not all(type(x) is int for x in scores) or type(k) is not int:
        return "scores is an array of integers; k is an integer"
    if not 1 <= len(scores) <= 10**5:
        return "1 ≤ scores.length ≤ 10⁵"
    if not 1 <= k <= len(scores):
        return "1 ≤ k ≤ scores.length"
    if not all(0 <= x <= 300 for x in scores):
        return "0 ≤ scores[i] ≤ 300"
    return None
