def validate(inp):
    n = inp["n"]
    if type(n) is not int or not 1 <= n <= 10**6:
        return "1 ≤ n ≤ 10⁶"
    return None
