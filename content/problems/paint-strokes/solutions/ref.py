def painted_length(strokes: list[list[int]]) -> int:
    strokes = sorted(strokes)
    total = 0
    run_start, run_end = strokes[0]
    for start, end in strokes[1:]:
        if start <= run_end:
            if end > run_end:
                run_end = end
        else:
            total += run_end - run_start
            run_start, run_end = start, end
    return total + run_end - run_start
