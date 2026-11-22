def reachable_count(n: int, doors: list[list[int]], start: int) -> int:
    # Obviously correct: grow the reached set until nothing changes.
    reached = {start}
    changed = True
    while changed:
        changed = False
        for a, b in doors:
            if (a in reached) != (b in reached):
                reached |= {a, b}
                changed = True
    return len(reached)
