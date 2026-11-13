def fence_area(trees: list[list[int]]) -> int:
    pts = sorted(map(tuple, trees))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    def half(points):
        hull = []
        for p in points:
            while len(hull) >= 2 and cross(hull[-2], hull[-1], p) <= 0:
                hull.pop()
            hull.append(p)
        return hull

    lower, upper = half(pts), half(reversed(pts))
    hull = lower[:-1] + upper[:-1]
    if len(hull) < 3:
        return 0
    return sum(hull[i][0] * hull[(i + 1) % len(hull)][1] - hull[(i + 1) % len(hull)][0] * hull[i][1] for i in range(len(hull)))
