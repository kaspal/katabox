def finish_time(jobs: list[int], lathes: int) -> int:
    # Correct but O(n·m): look through every lathe for the one free first.
    free = [0] * lathes
    latest = 0
    for job in jobs:
        best = 0
        for k in range(1, lathes):
            if free[k] < free[best]:
                best = k
        free[best] += job
        if free[best] > latest:
            latest = free[best]
    return latest
