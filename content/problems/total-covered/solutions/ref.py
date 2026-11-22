def total_covered(shifts: list[list[int]]) -> int:
    total = 0
    cur_s = cur_e = None
    for s, e in sorted(shifts):
        if cur_e is not None and s <= cur_e:
            cur_e = max(cur_e, e)
        else:
            if cur_e is not None:
                total += cur_e - cur_s
            cur_s, cur_e = s, e
    if cur_e is not None:
        total += cur_e - cur_s
    return total
