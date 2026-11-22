"""reachable-count constraints."""


def validate(inp):
    n, doors, start = inp["n"], inp["doors"], inp["start"]
    if type(n) is not int or not 1 <= n <= 10**4:
        return "1 ≤ n ≤ 10⁴"
    if not isinstance(doors, list) or len(doors) > 2 * 10**4:
        return "at most 2·10⁴ doors"
    for d in doors:
        if not isinstance(d, list) or len(d) != 2 or not all(type(x) is int and 0 <= x < n for x in d):
            return "each door is [a, b] with 0 ≤ a, b < n"
        if d[0] == d[1]:
            return "a door joins two different rooms"
    if type(start) is not int or not 0 <= start < n:
        return "0 ≤ start < n"
    return None
