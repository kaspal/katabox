def validate(inp):
    n, roads, start, goal = inp["n"], inp["roads"], inp["start"], inp["goal"]
    if type(n) is not int or not isinstance(roads, list) or type(start) is not int or type(goal) is not int:
        return "n, start and goal are integers; roads is an array of triples"
    if not 1 <= n <= 2 * 10**4:
        return "1 ≤ n ≤ 2·10⁴"
    if len(roads) > 5 * 10**4:
        return "0 ≤ roads.length ≤ 5·10⁴"
    for r in roads:
        if not (isinstance(r, list) and len(r) == 3 and all(type(x) is int for x in r)
                and 0 <= r[0] < n and 0 <= r[1] < n and r[0] != r[1] and 1 <= r[2] <= 10**4):
            return "roads[i] = [a, b, toll] with 0 ≤ a, b < n, a ≠ b and 1 ≤ toll ≤ 10⁴"
    if not 0 <= start < n or not 0 <= goal < n:
        return "0 ≤ start, goal < n"
    return None
