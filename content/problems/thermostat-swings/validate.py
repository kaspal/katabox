def validate(inp):
    readings, k = inp["readings"], inp["k"]
    if not isinstance(readings, list) or not all(type(r) is int for r in readings):
        return "readings is an array of integers"
    if type(k) is not int or not 1 <= k <= len(readings) <= 10**5:
        return "1 ≤ k ≤ readings.length ≤ 10⁵"
    if not all(-10**4 <= r <= 10**4 for r in readings):
        return "−10⁴ ≤ readings[i] ≤ 10⁴"
    return None
