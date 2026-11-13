def validate(inp):
    n = inp["n"]
    if type(n) is not int or not 0 <= n <= 10**5:
        return "0 ≤ n ≤ 10⁵"
    return None
