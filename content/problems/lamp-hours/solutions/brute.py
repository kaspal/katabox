def lit_time(lamps: list[list[int]]) -> int:
    # Obviously correct: mark every lit second.
    lit = set()
    for start, end in lamps:
        lit.update(range(start, end))
    return len(lit)
