import re

PAIR = re.compile(r"(-?[0-9]+) (-?[0-9]+)")


def cross(o, a, b):
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def on_segment(p, a, b):
    return min(a[0], b[0]) <= p[0] <= max(a[0], b[0]) and min(a[1], b[1]) <= p[1] <= max(a[1], b[1])


def touch(a, b, c, d):
    """Do segments ab and cd share at least one point?"""
    d1, d2, d3, d4 = cross(c, d, a), cross(c, d, b), cross(a, b, c), cross(a, b, d)
    if ((d1 > 0) != (d2 > 0)) and d1 != 0 and d2 != 0 and ((d3 > 0) != (d4 > 0)) and d3 != 0 and d4 != 0:
        return True
    return (
        (d1 == 0 and on_segment(a, c, d))
        or (d2 == 0 and on_segment(b, c, d))
        or (d3 == 0 and on_segment(c, a, b))
        or (d4 == 0 and on_segment(d, a, b))
    )


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+", lines[0]):
        return "the first line is n"
    n = int(lines[0])
    if not 3 <= n <= 1000:
        return "3 ≤ n ≤ 1000"
    if len(lines) != 1 + n:
        return "then exactly n vertex lines"
    pts = []
    for line in lines[1:]:
        m = PAIR.fullmatch(line)
        if not m:
            return "each vertex is x y"
        x, y = int(m.group(1)), int(m.group(2))
        if not (abs(x) <= 1_000_000 and abs(y) <= 1_000_000):
            return "|x|, |y| ≤ 10⁶"
        pts.append((x, y))
    if len(set(pts)) != n:
        return "the vertices are different points"
    edges = [(pts[i], pts[(i + 1) % n]) for i in range(n)]
    # Consecutive edges only share their common vertex: they may not fold back.
    for i in range(n):
        a, b = edges[i]
        _, c = edges[(i + 1) % n]
        if cross(a, b, c) == 0 and (b[0] - a[0]) * (c[0] - b[0]) + (b[1] - a[1]) * (c[1] - b[1]) < 0:
            return "the polygon is simple: an edge may not fold back over the previous one"
    if sum(cross((0, 0), a, b) for a, b in edges) == 0:
        return "the plot has a positive area"
    # Other edges never touch (bounding boxes first, to keep this quick).
    boxes = [(min(a[0], b[0]), max(a[0], b[0]), min(a[1], b[1]), max(a[1], b[1])) for a, b in edges]
    for i in range(n):
        x0, x1, y0, y1 = boxes[i]
        for j in range(i + 2, n):
            if i == 0 and j == n - 1:
                continue
            u0, u1, v0, v1 = boxes[j]
            if u1 < x0 or u0 > x1 or v1 < y0 or v0 > y1:
                continue
            if touch(*edges[i], *edges[j]):
                return "the polygon is simple: edges meet only at shared corners of neighbouring edges"
    return None
