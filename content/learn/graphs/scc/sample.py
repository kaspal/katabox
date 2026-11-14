def kosaraju(n, adj):
    seen = [False] * n
    finished = []

    def first(u):
        seen[u] = True
        for v, _ in adj[u]:
            if not seen[v]:
                first(v)
        finished.append(u)  # @step finish

    for u in range(n):
        if not seen[u]:
            first(u)

    back = [[] for _ in range(n)]
    for u in range(n):
        for v, _ in adj[u]:
            back[v].append(u)
    label = [-1] * n

    def second(u, c):
        label[u] = c  # @step assign
        for v in back[u]:
            if label[v] == -1:
                second(v, c)

    count = 0
    for u in reversed(finished):
        if label[u] == -1:
            second(u, count)
            count += 1
    return label
