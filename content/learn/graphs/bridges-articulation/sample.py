def bridges_and_cuts(n, adj):
    tin = [-1] * n
    low = [0] * n
    bridges, cuts = [], set()
    timer = 0

    def visit(u, parent):
        nonlocal timer
        tin[u] = low[u] = timer  # @step enter
        timer += 1
        children = 0
        for v, _ in adj[u]:
            if v == parent:
                continue
            if tin[v] != -1:
                low[u] = min(low[u], tin[v])  # @step back
            else:
                visit(v, u)
                children += 1
                low[u] = min(low[u], low[v])  # @step low
                if low[v] > tin[u]:
                    bridges.append((min(u, v), max(u, v)))  # @step bridge
                if parent != -1 and low[v] >= tin[u]:
                    cuts.add(u)  # @step cut
        if parent == -1 and children > 1:
            cuts.add(u)  # @step root-cut

    for u in range(n):
        if tin[u] == -1:
            visit(u, -1)
    return [x for pair in sorted(bridges) for x in pair] + [-1] + sorted(cuts)
