"""trapped-rain--f1 constraints."""


def validate(inp):
    h = inp["heights"]
    if not isinstance(h, list) or not 1 <= len(h) <= 200 or not all(isinstance(r, list) for r in h):
        return "1 ≤ rows ≤ 200"
    cols = len(h[0])
    if not 1 <= cols <= 200 or any(len(r) != cols for r in h):
        return "1 ≤ columns ≤ 200, every row the same length"
    if not all(type(x) is int and 0 <= x <= 10**4 for r in h for x in r):
        return "0 ≤ heights[r][c] ≤ 10⁴"
    return None
