def select(intervals):
    order = sorted(
        range(len(intervals)),
        key=lambda k: (intervals[k][1], intervals[k][0], k),
    )
    chosen, free_at = [], None
    for k in order:
        start, end = intervals[k]
        if free_at is None or start >= free_at:
            chosen.append(k)  # @step take
            free_at = end
        else:
            continue  # @step clash
    return [len(chosen)] + sorted(chosen)
