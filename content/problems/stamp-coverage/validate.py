def validate(inp):
    r, c, s = inp["rows"], inp["cols"], inp["stamps"]
    if type(r) is not int or type(c) is not int or not (1 <= r <= 500 and 1 <= c <= 500):
        return "1 ≤ rows, cols ≤ 500"
    if not isinstance(s, list) or not 1 <= len(s) <= 10**5:
        return "1 ≤ stamps.length ≤ 10⁵"
    for x in s:
        if not isinstance(x, list) or len(x) != 4 or not all(type(v) is int for v in x):
            return "each stamp is [top, left, bottom, right]"
        t, l, b, rt = x
        if not (0 <= t <= b < r and 0 <= l <= rt < c):
            return "0 ≤ top ≤ bottom < rows, 0 ≤ left ≤ right < cols"
    return None
