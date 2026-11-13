def blocked_walks(rows: int, cols: int, blocked: list[list[int]]) -> int:
    # Correct but O(rows · cols): count walks crossing by crossing, one row at a time.
    P = 1_000_003
    closed = {}
    for r, c in blocked:
        closed.setdefault(r, set()).add(c)
    row = [0] * (cols + 1)
    for r in range(rows + 1):
        shut = closed.get(r, ())
        for c in range(cols + 1):
            if c in shut:
                row[c] = 0
            elif r == 0 and c == 0:
                row[c] = 1
            elif c > 0:
                row[c] = (row[c] + row[c - 1]) % P
    return row[cols]
