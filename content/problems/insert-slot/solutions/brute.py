def insert_slot(slots: list[list[int]], fresh: list[int]) -> list[list[int]]:
    # Obviously correct: put everything together, sort, and merge from scratch.
    every = slots + [fresh]
    out = []
    for s, e in sorted(every):
        if out and s <= out[-1][1]:
            out[-1][1] = max(out[-1][1], e)
        else:
            out.append([s, e])
    return out
