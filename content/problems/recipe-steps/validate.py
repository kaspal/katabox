def validate(inp):
    n, before = inp["n"], inp["before"]
    if type(n) is not int or not isinstance(before, list):
        return "n is an integer; before is an array"
    if not 1 <= n <= 2 * 10**4:
        return "1 ≤ n ≤ 2·10⁴"
    if not 0 <= len(before) <= 4 * 10**4:
        return "0 ≤ before.length ≤ 4·10⁴"
    for e in before:
        if not (isinstance(e, list) and len(e) == 2 and all(type(x) is int for x in e)):
            return "before[i] = [a, b] with integers"
        a, b = e
        if not (0 <= a < n and 0 <= b < n and a != b):
            return "0 ≤ a, b < n and a ≠ b"
    return None
