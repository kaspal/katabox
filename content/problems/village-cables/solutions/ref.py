def cheapest_network(n: int, cables: list[list[int]]) -> int:
    parent = list(range(n))

    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    total = laid = 0
    for a, b, cost in sorted(cables, key=lambda c: c[2]):
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[ra] = rb
            total += cost
            laid += 1
            if laid == n - 1:
                break
    return total if laid == n - 1 else -1
