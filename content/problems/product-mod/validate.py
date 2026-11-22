"""product-mod constraints."""


def validate(inp):
    a = inp["nums"]
    if not isinstance(a, list) or not 1 <= len(a) <= 10**5:
        return "1 ≤ nums.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**9 for x in a):
        return "1 ≤ nums[i] ≤ 10⁹"
    return None
