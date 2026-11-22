"""fresh-playlist--f2 constraints."""

def _plays(p):
    if not isinstance(p, list) or not 1 <= len(p) <= 10**5:
        return "1 ≤ plays.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**9 for x in p):
        return "1 ≤ plays[i] ≤ 10⁹"
    return None


def validate(inp):
    err = _plays(inp["plays"])
    if err:
        return err
    k = inp["k"]
    if type(k) is not int or not 1 <= k <= 10**5:
        return "1 ≤ k ≤ 10⁵"
    return None
