def validate(inp):
    n, cables = inp["n"], inp["cables"]
    if type(n) is not int or not isinstance(cables, list):
        return "n is an integer; cables is an array"
    if not 1 <= n <= 2 * 10**4:
        return "1 ≤ n ≤ 2·10⁴"
    if not 0 <= len(cables) <= 4 * 10**4:
        return "0 ≤ cables.length ≤ 4·10⁴"
    for c in cables:
        if not (isinstance(c, list) and len(c) == 3 and all(type(x) is int for x in c)):
            return "cables[i] = [a, b, cost] with integers"
        a, b, cost = c
        if not (0 <= a < n and 0 <= b < n and a != b and 1 <= cost <= 10**6):
            return "0 ≤ a, b < n, a ≠ b and 1 ≤ cost ≤ 10⁶"
    return None
