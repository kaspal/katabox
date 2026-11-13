def validate(inp):
    c = inp["columns"]
    if not isinstance(c, list) or not 1 <= len(c) <= 2 * 10**5:
        return "1 ≤ columns.length ≤ 2·10⁵"
    if not all(type(v) is int and 0 <= v <= 10**4 for v in c):
        return "0 ≤ columns[i] ≤ 10⁴"
    return None
