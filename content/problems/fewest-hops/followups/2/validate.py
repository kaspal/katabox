"""fewest-hops--f2 constraints."""


def validate(inp):
    n, links = inp["n"], inp["links"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(links, list) or len(links) > 2 * 10**5:
        return "0 ≤ links.length ≤ 2·10⁵"
    for l in links:
        if not (isinstance(l, list) and len(l) == 2 and all(type(x) is int and 0 <= x < n for x in l) and l[0] != l[1]):
            return "links[i] = [a, b] with 0 ≤ a, b < n and a ≠ b"
    return None
