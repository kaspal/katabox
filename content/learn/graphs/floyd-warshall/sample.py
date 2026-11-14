INF = 999


def floyd_warshall(n, edges):
    dist = [[0 if i == j else INF for j in range(n)] for i in range(n)]
    for u, v, w in edges:
        dist[u][v] = w
    for k in range(n):
        through = dist[k]  # @step pivot
        for i in range(n):
            for j in range(n):
                if dist[i][k] + through[j] < dist[i][j]:
                    dist[i][j] = dist[i][k] + through[j]  # @step improve
    return [x for row in dist for x in row]
