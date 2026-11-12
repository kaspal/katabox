def lit_time(lamps: list[list[int]]) -> int:
    ordered = sorted(lamps)
    s, e = ordered[0]
    total = 0
    for start, end in ordered[1:]:
        if start <= e:
            e = max(e, end)
        else:
            total += e - s
            s, e = start, end
    return total + e - s
