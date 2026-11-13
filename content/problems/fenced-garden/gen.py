"""Random star-shaped fences (always simple) and points, many on the fence."""
import math
import random

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


def fence(rng, k, r):
    while True:
        cx, cy = rng.randint(-r // 4, r // 4), rng.randint(-r // 4, r // 4)
        angles = sorted(rng.uniform(0, 2 * math.pi) for _ in range(k))
        poly = []
        for a in angles:
            rad = rng.uniform(r * 0.2, r * 0.7)
            p = [round(cx + rad * math.cos(a)), round(cy + rad * math.sin(a))]
            if not poly or p != poly[-1]:
                poly.append(p)
        if len(poly) >= 3 and simple(poly):
            if rng.random() < 0.5:
                poly.reverse()
            return poly


def on_fence_point(rng, poly):
    a = rng.choice(poly)
    b = poly[(poly.index(a) + 1) % len(poly)]
    g = math.gcd(abs(b[0] - a[0]), abs(b[1] - a[1]))
    k = rng.randint(0, g)
    return [a[0] + (b[0] - a[0]) // g * k, a[1] + (b[1] - a[1]) // g * k]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        k, r, m = 300, 10**4, 1000
    else:
        k, r, m = rng.randint(3, 7), 12, rng.randint(1, 8)
    poly = fence(rng, k, r)
    pts = []
    for _ in range(m):
        if rng.random() < 0.3:
            pts.append(on_fence_point(rng, poly))
        else:
            pts.append([rng.randint(-r, r), rng.randint(-r, r)])
    return {"fence": poly, "points": pts}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
