def critical_bridges(n: int, bridges: list[list[int]]) -> list[list[int]]:
    # Correct but O(m·(n + m)): close each footbridge, then check whether its two ends
    # still reach each other.
    adj = [[] for _ in range(n)]
    for idx, (a, b) in enumerate(bridges):
        adj[a].append((b, idx))
        adj[b].append((a, idx))
    out = []
    for idx, (a, b) in enumerate(bridges):
        seen = [False] * n
        seen[a] = True
        stack = [a]
        while stack and not seen[b]:
            x = stack.pop()
            for y, j in adj[x]:
                if j != idx and not seen[y]:
                    seen[y] = True
                    stack.append(y)
        if not seen[b]:
            out.append(sorted((a, b)))
    return sorted(out)
