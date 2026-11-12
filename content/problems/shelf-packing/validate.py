def validate(inp):
    widths, width = inp["widths"], inp["width"]
    if type(width) is not int or not 1 <= width <= 1000:
        return "1 ≤ width ≤ 1000"
    if not isinstance(widths, list) or not 1 <= len(widths) <= 15:
        return "1 ≤ widths.length ≤ 15"
    if not all(type(w) is int and 1 <= w <= width for w in widths):
        return "1 ≤ widths[i] ≤ width"
    return None
