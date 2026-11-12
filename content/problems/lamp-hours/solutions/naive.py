def lit_time(lamps: list[list[int]]) -> int:
    # Correct but O(n²): cut time at every start and end; a piece between two cuts is
    # lit when some lamp covers it, which is checked against every lamp.
    cuts = sorted({t for lamp in lamps for t in lamp})
    total = 0
    for a, b in zip(cuts, cuts[1:]):
        for start, end in lamps:
            if start <= a and b <= end:
                total += b - a
                break
    return total
