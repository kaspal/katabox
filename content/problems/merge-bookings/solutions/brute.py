def merge_bookings(bookings: list[list[int]]) -> list[list[int]]:
    # Obviously correct (few bookings): group bookings linked by shared moments
    # (a search over the "overlaps" graph), then take each group's extent.
    n = len(bookings)
    group = [-1] * n
    for i in range(n):
        if group[i] < 0:
            group[i], stack = i, [i]
            while stack:
                a = stack.pop()
                for b in range(n):
                    if group[b] < 0 and max(bookings[a][0], bookings[b][0]) <= min(bookings[a][1], bookings[b][1]):
                        group[b] = i
                        stack.append(b)
    out = []
    for g in set(group):
        members = [bookings[i] for i in range(n) if group[i] == g]
        out.append([min(m[0] for m in members), max(m[1] for m in members)])
    return sorted(out)
