"""family-generations--f1 constraints."""


def validate(inp):
    parent, pairs = inp["parent"], inp["pairs"]
    if not isinstance(parent, list) or not 1 <= len(parent) <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    n = len(parent)
    if not all(type(p) is int and -1 <= p < n for p in parent) or parent.count(-1) != 1:
        return "exactly one parent[i] is -1; every other is between 0 and n − 1"
    for v in range(n):
        seen = 0
        while parent[v] != -1:
            v = parent[v]
            seen += 1
            if seen > n:
                return "following parent always reaches the founder"
    if not isinstance(pairs, list) or not 1 <= len(pairs) <= 10**5:
        return "1 ≤ pairs.length ≤ 10⁵"
    if not all(isinstance(x, list) and len(x) == 2 and all(type(v) is int and 0 <= v < n for v in x) for x in pairs):
        return "pairs[i] = [a, b] with 0 ≤ a, b < n"
    return None
