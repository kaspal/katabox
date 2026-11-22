"""deepest-report constraints."""


def validate(inp):
    m = inp["manager"]
    n = len(m) if isinstance(m, list) else 0
    if not 1 <= n <= 5 * 10**4:
        return "1 ≤ manager.length ≤ 5·10⁴"
    if not all(type(x) is int and -1 <= x < n for x in m):
        return "−1 ≤ manager[i] < manager.length"
    if sum(1 for x in m if x == -1) != 1:
        return "exactly one person (the boss) has manager −1"
    # Everyone reaches the boss: no cycles.
    state = [0] * n  # 0 new, 1 on the current walk, 2 reaches the boss
    for v in range(n):
        walk = []
        u = v
        while u != -1 and state[u] == 0:
            state[u] = 1
            walk.append(u)
            u = m[u]
        if u != -1 and state[u] == 1:
            return "the chart has a cycle"
        for w in walk:
            state[w] = 2
    return None
