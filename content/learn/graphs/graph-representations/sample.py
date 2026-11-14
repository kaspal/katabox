def build(n, directed, edges):
    adj = [[] for _ in range(n)]
    matrix = [[0] * n for _ in range(n)]
    for u, v, w in edges:
        adj[u].append(v)  # @step list
        matrix[u][v] = w  # @step matrix
        if not directed:
            adj[v].append(u)  # @step list-back
            matrix[v][u] = w
    return adj, matrix


def run(n, directed, edges):
    adj, matrix = build(n, directed, edges)
    out = []
    for row in adj:
        out.append(len(row))
        out.extend(row)
    for row in matrix:
        out.extend(row)
    return out
