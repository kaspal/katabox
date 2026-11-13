def fresh_stretch(plays: list[int]) -> int:
    last, start, best = {}, 0, 0
    for end, x in enumerate(plays):
        if last.get(x, -1) >= start:
            start = last[x] + 1
        last[x] = end
        best = max(best, end - start + 1)
    return best
