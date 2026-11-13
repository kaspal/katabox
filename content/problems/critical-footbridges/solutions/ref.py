def critical_bridges(n: int, bridges: list[list[int]]) -> list[list[int]]:
    adj = [[] for _ in range(n)]
    for idx, (a, b) in enumerate(bridges):
        adj[a].append((b, idx))
        adj[b].append((a, idx))
    tin = [-1] * n
    low = [0] * n
    timer = 0
    out = []
    for s in range(n):
        if tin[s] != -1:
            continue
        tin[s] = low[s] = timer
        timer += 1
        stack = [(s, 0, -1)]  # island, next neighbour index, footbridge used
        while stack:
            v, i, via = stack[-1]
            if i < len(adj[v]):
                stack[-1] = (v, i + 1, via)
                u, idx = adj[v][i]
                if idx == via:
                    continue
                if tin[u] == -1:
                    tin[u] = low[u] = timer
                    timer += 1
                    stack.append((u, 0, idx))
                else:
                    low[v] = min(low[v], tin[u])
            else:
                stack.pop()
                if stack:
                    p = stack[-1][0]
                    low[p] = min(low[p], low[v])
                    if low[v] > tin[p]:
                        out.append(sorted((p, v)))
    return sorted(out)
