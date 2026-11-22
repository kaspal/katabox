"""first-at-least constraints."""


def validate(inp):
    a, x = inp["sorted"], inp["x"]
    if not isinstance(a, list) or not all(type(v) is int for v in a) or type(x) is not int:
        return "sorted is an array of integers; x is an integer"
    if not 1 <= len(a) <= 10**5:
        return "1 ≤ sorted.length ≤ 10⁵"
    if not all(-(10**9) <= v <= 10**9 for v in a + [x]):
        return "−10⁹ ≤ sorted[i], x ≤ 10⁹"
    if a != sorted(a):
        return "sorted is in non-decreasing order"
    return None
