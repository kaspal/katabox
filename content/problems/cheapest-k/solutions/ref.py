import heapq


def cheapest_k(prices: list[int], k: int) -> int:
    # Keep the k cheapest so far in a max-heap (negated values): the root is the
    # most expensive of them, the one to drop when a cheaper price arrives.
    heap = []
    for p in prices:
        heapq.heappush(heap, -p)
        if len(heap) > k:
            heapq.heappop(heap)
    return -sum(heap)
