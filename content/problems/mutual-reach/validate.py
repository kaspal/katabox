def validate(inp):
    n, links = inp["n"], inp["links"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(links, list) or len(links) > 2 * 10**5:
        return "0 ≤ links.length ≤ 2·10⁵"
    for e in links:
        if not isinstance(e, list) or len(e) != 2 or not all(type(v) is int for v in e):
            return "each link is [a, b]"
        if not (0 <= e[0] < n and 0 <= e[1] < n and e[0] != e[1]):
            return "0 ≤ a, b < n and a ≠ b"
    return None
