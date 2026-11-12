def painted_length(strokes: list[list[int]]) -> int:
    # Correct but O(n²) when strokes do not overlap: merge each stroke into a list of
    # disjoint runs by scanning the whole list.
    runs = []
    for start, end in strokes:
        kept = []
        for a, b in runs:
            if b < start or end < a:
                kept.append((a, b))
            else:
                start, end = min(start, a), max(end, b)
        kept.append((start, end))
        runs = kept
    return sum(b - a for a, b in runs)
