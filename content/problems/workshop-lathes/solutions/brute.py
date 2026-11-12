def finish_time(jobs: list[int], lathes: int) -> int:
    # Obviously correct: simulate minute by minute.
    free_at = [0] * lathes
    queue = list(jobs)
    minute = 0
    latest = 0
    while queue:
        for k in range(lathes):
            if queue and free_at[k] <= minute:
                free_at[k] = minute + queue.pop(0)
                latest = max(latest, free_at[k])
        minute += 1
    return latest
