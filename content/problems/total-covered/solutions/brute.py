def total_covered(shifts: list[list[int]]) -> int:
    # Obviously correct: mark every covered unit of time [t, t + 1).
    covered = set()
    for s, e in shifts:
        covered.update(range(s, e))
    return len(covered)
