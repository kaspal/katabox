def orient(a, b, c):
    v = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    return (v > 0) - (v < 0)


def on_segment(a, b, c):
    return min(a[0], b[0]) <= c[0] <= max(a[0], b[0]) and min(a[1], b[1]) <= c[1] <= max(a[1], b[1])


def intersects(p1, p2, p3, p4):
    checks = [(p3, p4, p1), (p3, p4, p2), (p1, p2, p3), (p1, p2, p4)]
    signs = []
    for a, b, c in checks:
        signs.append(orient(a, b, c))  # @step side
    if signs[0] * signs[1] < 0 and signs[2] * signs[3] < 0:
        return [1] + signs  # @step cross
    for (a, b, c), s in zip(checks, signs):
        if s == 0 and on_segment(a, b, c):
            return [1] + signs  # @step touch
    return [0] + signs
