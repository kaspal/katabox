import heapq


def nearest_stalls(stalls: list[list[int]], k: int) -> list[int]:
    heap = []  # max-heap of the best k keys, stored negated
    for i, (x, y) in enumerate(stalls):
        heapq.heappush(heap, (-(x * x + y * y), -i))
        if len(heap) > k:
            heapq.heappop(heap)
    return [i for _, i in sorted((-d, -i) for d, i in heap)]
