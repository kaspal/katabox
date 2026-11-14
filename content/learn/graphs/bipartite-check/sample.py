from collections import deque


def two_colour(n, adj):
    side = [-1] * n
    for s in range(n):
        if side[s] != -1:
            continue
        side[s] = 0  # @step start
        queue = deque([s])
        while queue:
            u = queue.popleft()
            for v, _ in adj[u]:
                if side[v] == -1:
                    side[v] = 1 - side[u]  # @step colour
                    queue.append(v)
                elif side[v] == side[u]:
                    return [-1]  # @step clash
    return side
