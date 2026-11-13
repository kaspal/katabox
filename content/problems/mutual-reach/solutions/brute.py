def mutual_reach(n: int, links: list[list[int]]) -> list[int]:
    # Obviously correct: compute who reaches whom, then group users that reach each other.
    adj = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
    reach = []
    for s in range(n):
        seen, stack = {s}, [s]
        while stack:
            x = stack.pop()
            for y in adj[x]:
                if y not in seen:
                    seen.add(y)
                    stack.append(y)
        reach.append(seen)
    done, sizes = set(), []
    for s in range(n):
        if s in done:
            continue
        group = {t for t in reach[s] if s in reach[t]}
        done |= group
        sizes.append(len(group))
    return sorted(sizes)
