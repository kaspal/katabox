"""delivery-times constraints."""


def validate(inp):
    n, roads, start = inp["n"], inp["roads"], inp["start"]
    if type(n) is not int or not 1 <= n <= 10**4:
        return "1 ≤ n ≤ 10⁴"
    if not isinstance(roads, list) or len(roads) > 3 * 10**4:
        return "at most 3·10⁴ roads"
    for r in roads:
        if not isinstance(r, list) or len(r) != 3 or not all(type(x) is int for x in r):
            return "each road is [a, b, minutes]"
        a, b, w = r
        if not (0 <= a < n and 0 <= b < n and a != b and 1 <= w <= 1000):
            return "0 ≤ a, b < n, a ≠ b, 1 ≤ minutes ≤ 1000"
    if type(start) is not int or not 0 <= start < n:
        return "0 ≤ start < n"
    return None
