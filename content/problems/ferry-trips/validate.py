def validate(inp):
    w, limit = inp["weights"], inp["limit"]
    if not isinstance(w, list) or not all(type(x) is int for x in w) or type(limit) is not int:
        return "weights is an array of integers; limit is an integer"
    if not 1 <= len(w) <= 10**5:
        return "1 ≤ weights.length ≤ 10⁵"
    if not 1 <= limit <= 3 * 10**4 or not all(1 <= x <= limit for x in w):
        return "1 ≤ weights[i] ≤ limit ≤ 3·10⁴"
    return None
