"""path-tiles--f1 constraints."""


def validate(inp):
    n = inp["n"]
    if type(n) is not int or not 1 <= n <= 10**15:
        return "1 ≤ n ≤ 10¹⁵"
    return None
