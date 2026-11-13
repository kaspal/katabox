def validate(inp):
    c = inp["cards"]
    if not isinstance(c, list) or not 1 <= len(c) <= 1000:
        return "1 ≤ cards.length ≤ 1000"
    if not all(type(v) is int and 0 <= v <= 10**9 for v in c):
        return "0 ≤ cards[i] ≤ 10⁹"
    return None
