import math


def fence_area(trees: list[list[int]]) -> int:
    # Obviously correct (small inputs): a point is a corner when it is not inside or on
    # any triangle or segment of the other distinct points; order the corners by angle
    # around their centroid and apply the shoelace formula.
    pts = sorted(set(map(tuple, trees)))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    def in_triangle(p, a, b, c):
        d1, d2, d3 = cross(a, b, p), cross(b, c, p), cross(c, a, p)
        if d1 == d2 == d3 == 0:
            xs, ys = (a[0], b[0], c[0]), (a[1], b[1], c[1])
            return min(xs) <= p[0] <= max(xs) and min(ys) <= p[1] <= max(ys)
        neg = d1 < 0 or d2 < 0 or d3 < 0
        pos = d1 > 0 or d2 > 0 or d3 > 0
        return not (neg and pos)

    corners = []
    for p in pts:
        others = [q for q in pts if q != p]
        covered = any(
            in_triangle(p, a, b, c)
            for i, a in enumerate(others)
            for j, b in enumerate(others)
            for c in others[j:]
            if j >= i
        )
        if not covered:
            corners.append(p)
    if len(corners) < 3:
        return 0
    cx = sum(p[0] for p in corners) / len(corners)
    cy = sum(p[1] for p in corners) / len(corners)
    corners.sort(key=lambda p: math.atan2(p[1] - cy, p[0] - cx))
    k = len(corners)
    return abs(sum(corners[i][0] * corners[(i + 1) % k][1] - corners[(i + 1) % k][0] * corners[i][1] for i in range(k)))
