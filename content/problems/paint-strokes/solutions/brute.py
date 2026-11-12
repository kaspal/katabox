def painted_length(strokes: list[list[int]]) -> int:
    # Obviously correct (for small coordinates): mark every painted centimetre.
    painted = set()
    for start, end in strokes:
        painted.update(range(start, end))
    return len(painted)
