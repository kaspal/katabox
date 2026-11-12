import heapq


def finish_time(jobs: list[int], lathes: int) -> int:
    free = [0] * min(lathes, len(jobs))
    latest = 0
    for job in jobs:
        end = heapq.heappop(free) + job
        heapq.heappush(free, end)
        if end > latest:
            latest = end
    return latest
