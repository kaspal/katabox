"""family-generations--f1 constraints."""


def validate(inp):
    parent, pairs = inp["parent"], inp["pairs"]
    if not isinstance(parent, list) or not 1 <= len(parent) <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    n = len(parent)
    if not all(type(p) is int and -1 <= p < n for p in parent) or parent.count(-1) != 1:
        return "exactly one parent[i] is -1; every other is between 0 and n − 1"
    # Following parent always reaches the founder (no cycle). Linear: each person is
    # walked once (0 = unknown, 1 = on the current walk, 2 = reaches the founder).
    state = [0] * n
    for start in range(n):
        path = []
        v = start
        while v != -1 and state[v] == 0:
            state[v] = 1
            path.append(v)
            v = parent[v]
        if v != -1 and state[v] == 1:
            return "following parent always reaches the founder"
        for u in path:
            state[u] = 2
    if not isinstance(pairs, list) or not 1 <= len(pairs) <= 10**5:
        return "1 ≤ pairs.length ≤ 10⁵"
    if not all(isinstance(x, list) and len(x) == 2 and all(type(v) is int and 0 <= v < n for v in x) for x in pairs):
        return "pairs[i] = [a, b] with 0 ≤ a, b < n"
    return None
