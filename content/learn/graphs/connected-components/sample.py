def components(n, adj):
    label = [-1] * n
    count = 0
    for s in range(n):
        if label[s] != -1:
            continue
        label[s] = count  # @step new
        stack = [s]
        while stack:
            u = stack.pop()
            for v, _ in adj[u]:
                if label[v] == -1:
                    label[v] = count  # @step fill
                    stack.append(v)
        count += 1
    return label
