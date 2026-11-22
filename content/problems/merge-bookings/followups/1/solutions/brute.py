def both_busy(room_a: list[list[int]], room_b: list[list[int]]) -> list[list[int]]:
    # Obviously correct: join any two ranges that share a moment until none do (as the
    # base problem merges), then intersect every range of one room with every range of
    # the other. Ranges that only touch end to start stay apart, as in the base problem.
    def merged(sheet: list[list[int]]) -> list[list[int]]:
        ranges = [list(b) for b in sheet]
        changed = True
        while changed:
            changed = False
            for i in range(len(ranges)):
                for j in range(i + 1, len(ranges)):
                    a, b = ranges[i], ranges[j]
                    if a[0] <= b[1] and b[0] <= a[1]:
                        ranges[i] = [min(a[0], b[0]), max(a[1], b[1])]
                        del ranges[j]
                        changed = True
                        break
                if changed:
                    break
        return ranges

    out = []
    for a in merged(room_a):
        for b in merged(room_b):
            s, e = max(a[0], b[0]), min(a[1], b[1])
            if s <= e:
                out.append([s, e])
    return sorted(out)
