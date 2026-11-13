def insert_slot(slots: list[list[int]], fresh: list[int]) -> list[list[int]]:
    s, e = fresh
    out, i, n = [], 0, len(slots)
    while i < n and slots[i][1] < s:
        out.append(slots[i])
        i += 1
    while i < n and slots[i][0] <= e:
        s, e = min(s, slots[i][0]), max(e, slots[i][1])
        i += 1
    out.append([s, e])
    out.extend(slots[i:])
    return out
