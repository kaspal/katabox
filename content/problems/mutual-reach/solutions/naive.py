def mutual_reach(n: int, links: list[list[int]]) -> list[int]:
    # Correct but O(n·(n + m)): one search per user, forwards and backwards.
    adj = [[] for _ in range(n)]
    radj = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        radj[b].append(a)

    def reach(start, graph):
        seen = [False] * n
        seen[start] = True
        stack = [start]
        while stack:
            x = stack.pop()
            for y in graph[x]:
                if not seen[y]:
                    seen[y] = True
                    stack.append(y)
        return seen

    group = [-1] * n
    sizes = []
    for s in range(n):
        if group[s] != -1:
            continue
        fwd, back = reach(s, adj), reach(s, radj)
        members = [v for v in range(n) if fwd[v] and back[v]]
        for v in members:
            group[v] = len(sizes)
        sizes.append(len(members))
    return sorted(sizes)
