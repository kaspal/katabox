def ring_length(n: int, roads: list[list[int]]) -> int:
    # Correct but O(n²): for every road, search for another path between its villages.
    near = [[] for _ in range(n)]
    for k, (a, b) in enumerate(roads):
        near[a].append((b, k))
        near[b].append((a, k))
    on_loop = 0
    for k, (a, b) in enumerate(roads):
        seen = [False] * n
        seen[a] = True
        stack = [a]
        while stack:
            v = stack.pop()
            for u, r in near[v]:
                if r != k and not seen[u]:
                    seen[u] = True
                    stack.append(u)
        if seen[b]:
            on_loop += 1
    return on_loop
