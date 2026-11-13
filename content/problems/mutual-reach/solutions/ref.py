def mutual_reach(n: int, links: list[list[int]]) -> list[int]:
    adj = [[] for _ in range(n)]
    radj = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        radj[b].append(a)
    seen = [False] * n
    order = []
    for s in range(n):
        if seen[s]:
            continue
        seen[s] = True
        stack = [(s, 0)]
        while stack:
            x, i = stack[-1]
            if i < len(adj[x]):
                stack[-1] = (x, i + 1)
                y = adj[x][i]
                if not seen[y]:
                    seen[y] = True
                    stack.append((y, 0))
            else:
                stack.pop()
                order.append(x)
    group = [-1] * n
    sizes = []
    for s in reversed(order):
        if group[s] != -1:
            continue
        group[s] = len(sizes)
        count, stack = 0, [s]
        while stack:
            x = stack.pop()
            count += 1
            for y in radj[x]:
                if group[y] == -1:
                    group[y] = len(sizes)
                    stack.append(y)
        sizes.append(count)
    return sorted(sizes)
