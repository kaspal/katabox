import heapq


def cheapest_k(prices: list[int], k: int) -> int:
    heap = []
    for p in prices:
        heapq.heappush(heap, p)
        if len(heap) > k:
            heapq.heappop(heap)
    return sum(heap)
