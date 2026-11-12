def validate(inp):
    limits = inp["limits"]
    if not isinstance(limits, list) or not all(type(x) is int for x in limits):
        return "limits is an array of integers"
    if not 1 <= len(limits) <= 2 * 10**4:
        return "1 ≤ limits.length ≤ 2·10⁴"
    if not all(1 <= x <= 10**6 for x in limits):
        return "1 ≤ limits[i] ≤ 10⁶"
    return None
