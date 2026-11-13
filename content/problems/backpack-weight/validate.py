def validate(inp):
    w, v, cap = inp["weights"], inp["values"], inp["capacity"]
    if not isinstance(w, list) or not isinstance(v, list) or not all(type(x) is int for x in w + v) or type(cap) is not int:
        return "weights and values are lists of integers; capacity is an integer"
    if not 1 <= len(w) == len(v) <= 200:
        return "1 ≤ weights.length = values.length ≤ 200"
    if not all(1 <= x <= 10**4 for x in w):
        return "1 ≤ weights[i] ≤ 10⁴"
    if not all(0 <= x <= 10**4 for x in v):
        return "0 ≤ values[i] ≤ 10⁴"
    if not 0 <= cap <= 10**4:
        return "0 ≤ capacity ≤ 10⁴"
    return None
