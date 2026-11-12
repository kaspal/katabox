def validate(inp):
    n, merges = inp["n"], inp["merges"]
    if type(n) is not int or not isinstance(merges, list):
        return "n is an integer; merges is an array"
    if not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not 1 <= len(merges) <= 10**5:
        return "1 ≤ merges.length ≤ 10⁵"
    for m in merges:
        if not (isinstance(m, list) and len(m) == 2 and all(type(x) is int and 0 <= x < n for x in m)):
            return "merges[i] = [a, b] with 0 ≤ a, b < n"
    return None
