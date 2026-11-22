"""fewest-hops--f1 constraints."""


def validate(inp):
    n, links = inp["n"], inp["links"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(links, list) or len(links) > 2 * 10**5:
        return "0 ≤ links.length ≤ 2·10⁵"
    seen = set()
    for link in links:
        if not (isinstance(link, list) and len(link) == 2 and all(type(v) is int and 0 <= v < n for v in link)):
            return "links[i] = [a, b] with 0 ≤ a, b < n"
        a, b = link
        if a == b:
            return "a ≠ b"
        key = (min(a, b), max(a, b))
        if key in seen:
            return "no pair of stations is linked twice"
        seen.add(key)
    return None
