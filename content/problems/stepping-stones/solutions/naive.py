def can_cross(jumps: list[int]) -> bool:
    # Correct but O(n · max jump): a search that tries every jump length from every
    # reachable stone.
    n = len(jumps)
    seen = [False] * n
    seen[0] = True
    stack = [0]
    while stack:
        i = stack.pop()
        for k in range(i + 1, min(n, i + jumps[i] + 1)):
            if not seen[k]:
                seen[k] = True
                stack.append(k)
    return seen[n - 1]
