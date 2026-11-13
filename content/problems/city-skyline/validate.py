def validate(inp):
    b = inp["buildings"]
    if not isinstance(b, list) or not 1 <= len(b) <= 10**5:
        return "1 ≤ buildings.length ≤ 10⁵"
    for x in b:
        if not isinstance(x, list) or len(x) != 3 or not all(type(v) is int for v in x):
            return "each building is [left, right, height]"
        if not (0 <= x[0] < x[1] <= 10**6 and 1 <= x[2] <= 10**4):
            return "0 ≤ left < right ≤ 10⁶, 1 ≤ height ≤ 10⁴"
    return None
