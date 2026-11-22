"""rising-run constraints."""


def validate(inp):
    h = inp["heights"]
    if not isinstance(h, list) or not 1 <= len(h) <= 10**5:
        return "1 ≤ heights.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**9 for x in h):
        return "1 ≤ heights[i] ≤ 10⁹"
    return None
