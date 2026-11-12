def validate(inp):
    h = inp["heights"]
    if not isinstance(h, list) or not all(isinstance(r, list) for r in h):
        return "heights is an array of arrays"
    if not 1 <= len(h) <= 200:
        return "1 ≤ heights.length ≤ 200"
    cols = len(h[0])
    if not 1 <= cols <= 200 or any(len(r) != cols for r in h):
        return "1 ≤ heights[i].length ≤ 200, all rows the same length"
    if not all(type(x) is int and 0 <= x <= 10**6 for r in h for x in r):
        return "0 ≤ heights[i][j] ≤ 10⁶"
    return None
