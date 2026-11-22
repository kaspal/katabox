"""backpack-weight--f2 constraints."""


def validate(inp):
    w, v, cap = inp["weights"], inp["values"], inp["capacity"]
    for name, a in (("weights", w), ("values", v)):
        if not isinstance(a, list) or not all(type(x) is int for x in a):
            return f"{name} is an array of integers"
    if not 1 <= len(w) <= 100 or len(v) != len(w):
        return "1 ≤ weights.length = values.length ≤ 100"
    if not all(1 <= x <= 10**7 for x in w):
        return "1 ≤ weights[i] ≤ 10⁷"
    if not all(0 <= x <= 100 for x in v):
        return "0 ≤ values[i] ≤ 100"
    if type(cap) is not int or not 0 <= cap <= 10**9:
        return "0 ≤ capacity ≤ 10⁹"
    return None
