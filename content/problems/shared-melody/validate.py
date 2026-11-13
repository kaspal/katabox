def validate(inp):
    a, b = inp["first"], inp["second"]
    if not isinstance(a, list) or not isinstance(b, list) or not all(type(x) is int for x in a + b):
        return "first and second are lists of integers"
    if not (1 <= len(a) <= 1500 and 1 <= len(b) <= 1500):
        return "1 ≤ first.length, second.length ≤ 1500"
    if not all(0 <= x <= 127 for x in a + b):
        return "0 ≤ note number ≤ 127"
    return None
