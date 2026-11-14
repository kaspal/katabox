def euler_path(n, edges):
    adj = [[] for _ in range(n)]
    for k, (u, v, _) in enumerate(edges):
        adj[u].append((v, k))
        adj[v].append((u, k))
    odd = [u for u in range(n) if len(adj[u]) % 2 == 1]
    if len(odd) not in (0, 2):
        return [-1]
    start = odd[0] if odd else edges[0][0]
    used = [False] * len(edges)
    nxt = [0] * n
    stack, path = [start], []
    while stack:
        u = stack[-1]
        while nxt[u] < len(adj[u]) and used[adj[u][nxt[u]][1]]:
            nxt[u] += 1
        if nxt[u] < len(adj[u]):
            v, k = adj[u][nxt[u]]
            used[k] = True
            stack.append(v)  # @step walk
        else:
            path.append(stack.pop())  # @step back
    if len(path) != len(edges) + 1:
        return [-1]
    return path[::-1]
