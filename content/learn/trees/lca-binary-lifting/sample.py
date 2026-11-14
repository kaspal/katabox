def build(parents):
    n = len(parents) + 1
    levels = max(1, n.bit_length())
    up = [[0] * n for _ in range(levels)]
    depth = [0] * n
    for v in range(1, n):
        up[0][v] = parents[v - 1]
        depth[v] = depth[parents[v - 1]] + 1
    for k in range(1, levels):
        for v in range(n):
            up[k][v] = up[k - 1][up[k - 1][v]]  # @step table
    return up, depth


def lca(up, depth, a, b):
    if depth[a] < depth[b]:
        a, b = b, a
    diff = depth[a] - depth[b]
    for k in range(len(up)):
        if diff >> k & 1:
            a = up[k][a]  # @step lift
    if a == b:
        return a  # @step same
    for k in range(len(up) - 1, -1, -1):
        if up[k][a] != up[k][b]:
            a, b = up[k][a], up[k][b]  # @step jump
    return up[0][a]  # @step answer


def run(parents, pairs):
    up, depth = build(parents)
    return [lca(up, depth, a, b) for a, b in pairs]
