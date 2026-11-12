def validate(inp):
    parent = inp["parent"]
    if not isinstance(parent, list) or not all(type(p) is int for p in parent):
        return "parent is an array of integers"
    n = len(parent)
    if not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if sum(1 for p in parent if p == -1) != 1:
        return "exactly one parent[i] is -1"
    if not all(p == -1 or 0 <= p < n for p in parent):
        return "every other parent[i] is between 0 and n − 1"
    # Everyone must reach the founder: mark people known to reach it.
    state = [0] * n  # 0 unknown, 1 on the current path, 2 reaches the founder
    for i in range(n):
        path = []
        p = i
        while p != -1 and state[p] == 0:
            state[p] = 1
            path.append(p)
            p = parent[p]
        if p != -1 and state[p] == 1:
            return "following parent never loops"
        for q in path:
            state[q] = 2
    return None
