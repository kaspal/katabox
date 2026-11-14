def kruskal(n, edges):
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    total, used = 0, []
    for u, v, w in sorted(edges, key=lambda e: (e[2], e[0], e[1])):
        ru, rv = find(u), find(v)  # @step consider
        if ru == rv:
            continue  # @step skip
        parent[ru] = rv
        total += w  # @step join
        used += [u, v]
    return [total] + used
