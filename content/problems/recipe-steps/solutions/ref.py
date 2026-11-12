import heapq


def step_order(n: int, before: list[list[int]]) -> list[int]:
    waiting = [0] * n
    out: list[list[int]] = [[] for _ in range(n)]
    for a, b in before:
        out[a].append(b)
        waiting[b] += 1
    heap = [s for s in range(n) if waiting[s] == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        s = heapq.heappop(heap)
        order.append(s)
        for t in out[s]:
            waiting[t] -= 1
            if waiting[t] == 0:
                heapq.heappush(heap, t)
    return order if len(order) == n else []
