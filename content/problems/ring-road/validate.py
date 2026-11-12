def validate(inp):
    n, roads = inp["n"], inp["roads"]
    if type(n) is not int or not isinstance(roads, list):
        return "n is an integer; roads is an array"
    if not 3 <= n <= 10**5:
        return "3 ≤ n ≤ 10⁵"
    if len(roads) != n:
        return "roads.length = n"
    pairs = set()
    for r in roads:
        if not (isinstance(r, list) and len(r) == 2 and all(type(x) is int for x in r)):
            return "roads[i] = [a, b] with integers"
        a, b = r
        if not (0 <= a < n and 0 <= b < n and a != b):
            return "0 ≤ a, b < n and a ≠ b"
        key = (min(a, b), max(a, b))
        if key in pairs:
            return "no pair of villages is joined twice"
        pairs.add(key)
    near = [[] for _ in range(n)]
    for a, b in roads:
        near[a].append(b)
        near[b].append(a)
    seen = [False] * n
    seen[0] = True
    stack = [0]
    while stack:
        v = stack.pop()
        for u in near[v]:
            if not seen[u]:
                seen[u] = True
                stack.append(u)
    if not all(seen):
        return "every village can reach every other one"
    return None
