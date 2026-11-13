def validate(inp):
    p, b = inp["prices"], inp["budget"]
    if not isinstance(p, list) or not 1 <= len(p) <= 15:
        return "1 ≤ prices.length ≤ 15"
    if not all(type(v) is int and 1 <= v <= 1000 for v in p):
        return "1 ≤ prices[i] ≤ 1000"
    if type(b) is not int or not 1 <= b <= 15000:
        return "1 ≤ budget ≤ 15000"
    return None
