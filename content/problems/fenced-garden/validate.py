def cross(o, a, b):
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def on_segment(p, a, b):
    return cross(a, b, p) == 0 and min(a[0], b[0]) <= p[0] <= max(a[0], b[0]) and min(a[1], b[1]) <= p[1] <= max(a[1], b[1])


def segments_meet(p1, p2, p3, p4):
    d1, d2, d3, d4 = cross(p3, p4, p1), cross(p3, p4, p2), cross(p1, p2, p3), cross(p1, p2, p4)
    if ((d1 > 0) != (d2 > 0)) and d1 != 0 and d2 != 0 and ((d3 > 0) != (d4 > 0)) and d3 != 0 and d4 != 0:
        return True
    return on_segment(p1, p3, p4) or on_segment(p2, p3, p4) or on_segment(p3, p1, p2) or on_segment(p4, p1, p2)


def simple(poly):
    """A simple polygon: distinct vertices, edges meet only at shared endpoints."""
    n = len(poly)
    if len(set(map(tuple, poly))) != n:
        return False
    edges = [(poly[i], poly[(i + 1) % n]) for i in range(n)]
    for i in range(n):
        for j in range(i + 1, n):
            a, b = edges[i]
            c, d = edges[j]
            if j == i + 1 or (i == 0 and j == n - 1):
                # Adjacent edges share one vertex; they must not overlap beyond it.
                shared = b if j == i + 1 else a
                other1 = a if j == i + 1 else b
                other2 = d if j == i + 1 else c
                if cross(shared, other1, other2) == 0 and (
                    (other1[0] - shared[0]) * (other2[0] - shared[0]) + (other1[1] - shared[1]) * (other2[1] - shared[1]) > 0
                ):
                    return False
                continue
            if segments_meet(a, b, c, d):
                return False
    return True


def validate(inp):
    fence, points = inp["fence"], inp["points"]
    ok = lambda p: isinstance(p, list) and len(p) == 2 and all(type(v) is int and abs(v) <= 10**4 for v in p)
    if not isinstance(fence, list) or not isinstance(points, list) or not all(map(ok, fence)) or not all(map(ok, points)):
        return "All coordinates are integers with absolute value at most 10⁴."
    if not 3 <= len(fence) <= 300:
        return "3 ≤ fence.length ≤ 300, and the fence is a simple polygon with distinct corners."
    if not 1 <= len(points) <= 1000:
        return "1 ≤ points.length ≤ 1000"
    if not simple(fence):
        return "3 ≤ fence.length ≤ 300, and the fence is a simple polygon with distinct corners."
    return None
