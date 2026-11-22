import heapq


def kth_after_each(scores: list[int], k: int) -> list[int]:
    top: list[int] = []  # a min-heap of the k best so far
    out = []
    for s in scores:
        if len(top) < k:
            heapq.heappush(top, s)
        elif s > top[0]:
            heapq.heapreplace(top, s)
        out.append(top[0] if len(top) == k else -1)
    return out
