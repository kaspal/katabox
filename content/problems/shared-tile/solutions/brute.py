def largest_tile(width: int, height: int) -> int:
    # Obviously correct: the largest size that divides both sides.
    for s in range(min(width, height), 0, -1):
        if width % s == 0 and height % s == 0:
            return s
    return 1
