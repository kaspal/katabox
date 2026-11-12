from collections import deque


def ring_length(n: int, roads: list[list[int]]) -> int:
    near: list[list[int]] = [[] for _ in range(n)]
    for a, b in roads:
        near[a].append(b)
        near[b].append(a)
    degree = [len(x) for x in near]
    removed = [False] * n
    queue = deque(v for v in range(n) if degree[v] == 1)
    left = n
    while queue:
        v = queue.popleft()
        removed[v] = True
        left -= 1
        for u in near[v]:
            if not removed[u]:
                degree[u] -= 1
                if degree[u] == 1:
                    queue.append(u)
    return left
