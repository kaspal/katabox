from collections import deque


def window_swings(readings: list[int], k: int) -> list[int]:
    maxq: deque[int] = deque()
    minq: deque[int] = deque()
    out = []
    for i, r in enumerate(readings):
        while maxq and readings[maxq[-1]] <= r:
            maxq.pop()
        maxq.append(i)
        while minq and readings[minq[-1]] >= r:
            minq.pop()
        minq.append(i)
        if maxq[0] <= i - k:
            maxq.popleft()
        if minq[0] <= i - k:
            minq.popleft()
        if i >= k - 1:
            out.append(readings[maxq[0]] - readings[minq[0]])
    return out
