"""club-merges--f1 constraints."""


def validate(inp):
    n, merges = inp["n"], inp["merges"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(merges, list) or not 1 <= len(merges) <= 10**5:
        return "1 ≤ merges.length ≤ 10⁵"
    if not all(isinstance(x, list) and len(x) == 2 and all(type(v) is int and 0 <= v < n for v in x) for x in merges):
        return "merges[i] = [a, b] with 0 ≤ a, b < n"
    return None
