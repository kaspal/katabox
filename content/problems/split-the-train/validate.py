def is_list(v, lo, hi):
    return isinstance(v, list) and all(type(x) is int and lo <= x <= hi for x in v)


def validate(inp):
    train, k = inp["train"], inp["k"]
    if not is_list(train, 1, 10**6):
        return "1 ≤ wagon number ≤ 10⁶"
    if len(train) > 10**4:
        return "0 ≤ number of wagons ≤ 10⁴"
    if type(k) is not int or not 1 <= k <= 100:
        return "1 ≤ k ≤ 100"
    return None
