def validate(inp):
    n, repairs = inp["length"], inp["repairs"]
    if type(n) is not int or not isinstance(repairs, list):
        return "length is an integer; repairs is a list"
    if not 1 <= n <= 10**5:
        return "1 ≤ length ≤ 10⁵"
    if len(repairs) > 10**5:
        return "0 ≤ repairs.length ≤ 10⁵"
    for r in repairs:
        if not isinstance(r, list) or len(r) != 3 or not all(type(v) is int for v in r):
            return "each repair is [a, b, crews]"
        a, b, c = r
        if not (0 <= a <= b < n and 1 <= c <= 100):
            return "0 ≤ a ≤ b < length and 1 ≤ crews ≤ 100 for each repair [a, b, crews]"
    return None
