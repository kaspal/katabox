import heapq


def running_medians(readings: list[int]) -> list[float]:
    low: list[int] = []  # max-heap, stored negated
    high: list[int] = []  # min-heap
    out = []
    for x in readings:
        if not low or x <= -low[0]:
            heapq.heappush(low, -x)
        else:
            heapq.heappush(high, x)
        if len(low) > len(high) + 1:
            heapq.heappush(high, -heapq.heappop(low))
        elif len(high) > len(low):
            heapq.heappush(low, -heapq.heappop(high))
        if len(low) > len(high):
            out.append(float(-low[0]))
        else:
            out.append((-low[0] + high[0]) / 2)
    return out
