def dfs(n, adj, source):
    seen = [False] * n
    order, finished = [], []

    def visit(u):
        seen[u] = True
        order.append(u)  # @step enter
        for v, _ in adj[u]:
            if not seen[v]:
                visit(v)  # @step go
        finished.append(u)  # @step leave

    visit(source)
    return order + finished
