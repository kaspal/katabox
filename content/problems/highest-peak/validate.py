def validate(inp):
    h = inp["heights"]
    if not isinstance(h, list) or not 1 <= len(h) <= 10**5:
        return "1 ≤ heights.length ≤ 10⁵"
    if not all(type(v) is int and 0 <= v <= 10**9 for v in h):
        return "0 ≤ heights[i] ≤ 10⁹"
    k = 0
    while k + 1 < len(h) and h[k] < h[k + 1]:
        k += 1
    while k + 1 < len(h) and h[k] > h[k + 1]:
        k += 1
    if k != len(h) - 1:
        return "heights strictly increases and then strictly decreases"
    return None
