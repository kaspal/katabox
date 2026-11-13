def validate(inp):
    a = inp["items"]
    if not isinstance(a, list) or not 1 <= len(a) <= 10**5:
        return "1 ≤ items.length ≤ 10⁵"
    if not all(type(v) is int and v in (0, 1, 2) for v in a):
        return "items[i] is 0, 1 or 2"
    return None
