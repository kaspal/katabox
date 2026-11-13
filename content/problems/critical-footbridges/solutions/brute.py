def critical_bridges(n: int, bridges: list[list[int]]) -> list[list[int]]:
    # Obviously correct: close each footbridge and count the connected groups.
    def groups(edges):
        parent = list(range(n))

        def find(x):
            while parent[x] != x:
                x = parent[x]
            return x

        for a, b in edges:
            parent[find(a)] = find(b)
        return len({find(x) for x in range(n)})

    base = groups(bridges)
    out = []
    for i, (a, b) in enumerate(bridges):
        if groups(bridges[:i] + bridges[i + 1 :]) > base:
            out.append(sorted((a, b)))
    return sorted(out)
