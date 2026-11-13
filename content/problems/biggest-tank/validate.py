def validate(inp):
    b = inp["boards"]
    if not isinstance(b, list) or not 2 <= len(b) <= 2 * 10**5:
        return "2 ≤ boards.length ≤ 2·10⁵"
    if not all(type(v) is int and 0 <= v <= 10**4 for v in b):
        return "0 ≤ boards[i] ≤ 10⁴"
    return None
