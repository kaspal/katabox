def fence_area(trees: list[list[int]]) -> int:
    # Correct but O(n·h): gift wrapping, scanning every tree for each corner.
    pts = sorted(set(map(tuple, trees)))
    if len(pts) < 3:
        return 0

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    def dist2(a, b):
        return (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2

    start = pts[0]
    hull, current = [], start
    while True:
        hull.append(current)
        candidate = pts[0] if pts[0] != current else pts[1]
        for p in pts:
            if p == current:
                continue
            c = cross(current, candidate, p)
            if c < 0 or (c == 0 and dist2(current, p) > dist2(current, candidate)):
                candidate = p
        current = candidate
        if current == start:
            break
    if len(hull) < 3:
        return 0
    k = len(hull)
    return abs(sum(hull[i][0] * hull[(i + 1) % k][1] - hull[(i + 1) % k][0] * hull[i][1] for i in range(k)))
