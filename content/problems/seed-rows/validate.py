def validate(inp):
    length, widths = inp["length"], inp["widths"]
    if type(length) is not int or not isinstance(widths, list) or not all(type(w) is int for w in widths):
        return "length is an integer; widths is an array of integers"
    if not 1 <= length <= 10**4:
        return "1 ≤ length ≤ 10⁴"
    if not 1 <= len(widths) <= 50:
        return "1 ≤ widths.length ≤ 50"
    if not all(1 <= w <= 100 for w in widths):
        return "1 ≤ widths[i] ≤ 100"
    return None
