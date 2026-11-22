def _merge(bookings: list[list[int]]) -> list[list[int]]:
    out: list[list[int]] = []
    for s, e in sorted(bookings):
        if out and s <= out[-1][1]:
            out[-1][1] = max(out[-1][1], e)
        else:
            out.append([s, e])
    return out


def both_busy(room_a: list[list[int]], room_b: list[list[int]]) -> list[list[int]]:
    a, b = _merge(room_a), _merge(room_b)
    i = j = 0
    out = []
    while i < len(a) and j < len(b):
        s = max(a[i][0], b[j][0])
        e = min(a[i][1], b[j][1])
        if s <= e:
            out.append([s, e])
        if a[i][1] < b[j][1]:
            i += 1
        else:
            j += 1
    return out
