from collections import deque


def two_teams(n: int, rivalries: list[list[int]]) -> bool:
    adj = [[] for _ in range(n)]
    for a, b in rivalries:
        adj[a].append(b)
        adj[b].append(a)
    colour = [-1] * n
    for s in range(n):
        if colour[s] != -1:
            continue
        colour[s] = 0
        q = deque([s])
        while q:
            x = q.popleft()
            for y in adj[x]:
                if colour[y] == -1:
                    colour[y] = 1 - colour[x]
                    q.append(y)
                elif colour[y] == colour[x]:
                    return False
    return True
