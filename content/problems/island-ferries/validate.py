def validate(inp):
    n, links = inp["n"], inp["links"]
    if type(n) is not int or not isinstance(links, list):
        return "n is an integer; links is an array of pairs"
    if not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if len(links) > 10**5:
        return "0 ≤ links.length ≤ 10⁵"
    for l in links:
        if not (isinstance(l, list) and len(l) == 2 and all(type(x) is int for x in l)
                and 0 <= l[0] < n and 0 <= l[1] < n and l[0] != l[1]):
            return "links[i] = [a, b] with 0 ≤ a, b < n and a ≠ b (the same pair may appear more than once)"
    return None
