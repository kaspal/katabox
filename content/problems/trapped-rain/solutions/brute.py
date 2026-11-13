def trapped_rain(columns: list[int]) -> int:
    # Obviously correct (low walls): fill level by level, counting the cells of each
    # level that have a column at least that tall on both sides.
    water = 0
    for level in range(1, max(columns) + 1):
        tall = [k for k, h in enumerate(columns) if h >= level]
        if len(tall) >= 2:
            water += sum(1 for k in range(tall[0], tall[-1] + 1) if columns[k] < level)
    return water
