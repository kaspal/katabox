def step_order(n: int, before: list[list[int]]) -> list[int]:
    # Correct but O(n·(n + m)): before every step, rescan all rules to see which steps
    # are still blocked, then scan all steps for the lowest ready one.
    finished = [False] * n
    order = []
    for _ in range(n):
        blocked = [False] * n
        for a, b in before:
            if not finished[a]:
                blocked[b] = True
        nxt = -1
        for s in range(n):
            if not finished[s] and not blocked[s]:
                nxt = s
                break
        if nxt < 0:
            return []
        finished[nxt] = True
        order.append(nxt)
    return order
