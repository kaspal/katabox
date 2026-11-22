"""backpack-weight--f1 constraints."""


def validate(inp):
    w, v, c, cap = inp["weights"], inp["values"], inp["counts"], inp["capacity"]
    for name, a in (("weights", w), ("values", v), ("counts", c)):
        if not isinstance(a, list) or not all(type(x) is int for x in a):
            return f"{name} is an array of integers"
    if not 1 <= len(w) <= 50 or len(v) != len(w) or len(c) != len(w):
        return "1 ≤ weights.length = values.length = counts.length ≤ 50"
    if not all(1 <= x <= 10**4 for x in w) or not all(0 <= x <= 10**4 for x in v):
        return "1 ≤ weights[i] ≤ 10⁴; 0 ≤ values[i] ≤ 10⁴"
    if not all(1 <= x <= 100 for x in c):
        return "1 ≤ counts[i] ≤ 100"
    if type(cap) is not int or not 0 <= cap <= 5000:
        return "0 ≤ capacity ≤ 5000"
    return None
