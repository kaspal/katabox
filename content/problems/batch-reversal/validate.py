def is_list(v, lo, hi):
    return isinstance(v, list) and all(type(x) is int and lo <= x <= hi for x in v)


def validate(inp):
    line, k = inp["line"], inp["k"]
    if not is_list(line, 1, 10**6):
        return "1 ≤ bottle number ≤ 10⁶"
    if len(line) > 10**5:
        return "0 ≤ number of bottles ≤ 10⁵"
    if type(k) is not int or not 1 <= k <= 10**5:
        return "1 ≤ k ≤ 10⁵"
    return None
