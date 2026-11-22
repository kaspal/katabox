"""cheapest-route--f2 constraints."""

def _roads(n, roads, max_toll):
    if not isinstance(roads, list):
        return "roads is a list"
    for r in roads:
        if not (isinstance(r, list) and len(r) == 3 and all(type(x) is int for x in r)):
            return "roads[i] = [a, b, toll]"
        a, b, t = r
        if not (0 <= a < n and 0 <= b < n and a != b and 1 <= t <= max_toll):
            return f"roads[i] = [a, b, toll] with 0 ≤ a, b < n, a ≠ b and 1 ≤ toll ≤ {max_toll}"
    return None


def validate(inp):
    n, roads, s, g = inp["n"], inp["roads"], inp["start"], inp["goal"]
    if type(n) is not int or not 1 <= n <= 2 * 10**4:
        return "1 ≤ n ≤ 2·10⁴"
    if not isinstance(roads, list) or len(roads) > 5 * 10**4:
        return "0 ≤ roads.length ≤ 5·10⁴"
    err = _roads(n, roads, 10**4)
    if err:
        return err
    if not all(type(x) is int for x in (s, g)) or not (0 <= s < n and 0 <= g < n):
        return "0 ≤ start, goal < n"
    return None
