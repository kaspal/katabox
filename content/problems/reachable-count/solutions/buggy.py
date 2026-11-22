from collections import deque


def reachable_count(n: int, doors: list[list[int]], start: int) -> int:
    adj = [[] for _ in range(n)]
    for a, b in doors:
        adj[a].append(b)
        adj[b].append(a)
    seen = [False] * n
    queue = deque([start])
    count = 0
    while queue:
        v = queue.popleft()
        count += 1
        for u in adj[v]:
            if not seen[u]:
                seen[u] = True
                queue.append(u)
    return count
