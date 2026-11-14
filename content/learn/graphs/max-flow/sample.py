from collections import deque


def max_flow(n, edges):
    cap = [[0] * n for _ in range(n)]
    for u, v, c in edges:
        cap[u][v] = c
    source, sink, total = 0, n - 1, 0
    while True:
        parent = [-1] * n
        parent[source] = source
        queue = deque([source])
        while queue and parent[sink] == -1:
            u = queue.popleft()
            for v in range(n):
                if parent[v] == -1 and cap[u][v] > 0:
                    parent[v] = u  # @step reach
                    queue.append(v)
        if parent[sink] == -1:
            return [total]
        push, v = 999, sink
        while v != source:
            push = min(push, cap[parent[v]][v])
            v = parent[v]
        v = sink
        while v != source:
            cap[parent[v]][v] -= push  # @step augment
            cap[v][parent[v]] += push
            v = parent[v]
        total += push
