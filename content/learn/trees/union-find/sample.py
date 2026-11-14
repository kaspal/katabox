class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.size = [1] * n

    def find(self, x):
        root = x
        while self.parent[root] != root:
            root = self.parent[root]  # @step climb
        while self.parent[x] != root:
            self.parent[x], x = root, self.parent[x]  # @step compress
        return root

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False  # @step same
        if self.size[ra] < self.size[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra  # @step link
        self.size[ra] += self.size[rb]
        return True


def run(n, ops):
    dsu, out = DSU(n), []
    for op in ops:
        if op[0] == "union":
            dsu.union(op[1], op[2])
        else:
            out.append(dsu.find(op[1]))
    out.append(sum(1 for x in range(n) if dsu.parent[x] == x))
    return out
