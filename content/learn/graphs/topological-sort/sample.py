from collections import deque


def topo_order(n, adj):
    indegree = [0] * n
    for u in range(n):
        for v, _ in adj[u]:
            indegree[v] += 1
    queue = deque(u for u in range(n) if indegree[u] == 0)
    order = []
    while queue:
        u = queue.popleft()
        order.append(u)  # @step take
        for v, _ in adj[u]:
            indegree[v] -= 1  # @step lower
            if indegree[v] == 0:
                queue.append(v)  # @step ready
    return order
