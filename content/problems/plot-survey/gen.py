"""Land plots (simple lattice polygons) as stdin text (a program problem, ADR 0028)."""
import math
import random


def render(points):
    return "\n".join([str(len(points))] + [f"{x} {y}" for x, y in points]) + "\n"


def star(rng, n, radius, cx=0, cy=0):
    """A star-shaped polygon around (cx, cy): vertices sorted by angle are always a simple
    polygon. Rounded points that land on an angle already taken are dropped."""
    pts = {}
    for _ in range(n):
        a = rng.uniform(0, 2 * math.pi)
        r = rng.uniform(radius / 3, radius)
        x, y = cx + round(r * math.cos(a)), cy + round(r * math.sin(a))
        if (x, y) == (cx, cy):
            continue
        g = math.gcd(x - cx, y - cy)
        key = ((x - cx) // g, (y - cy) // g)  # one point per direction from the centre
        pts.setdefault(key, (x, y))
    out = sorted(pts.values(), key=lambda p: math.atan2(p[1] - cy, p[0] - cx))
    # The centre must see the polygon all round: no angle gap of half a turn or more.
    angles = [math.atan2(y - cy, x - cx) for x, y in out]
    gaps = [b - a for a, b in zip(angles, angles[1:])] + [angles[0] + 2 * math.pi - angles[-1]]
    if len(out) < 3 or max(gaps) >= math.pi - 1e-9:
        return [(cx - radius, cy - radius), (cx + radius, cy - radius), (cx, cy + radius)]
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        if seed % 2 == 1:
            # 1000 vertices spread over the whole ±10⁶ field: an area of about 10¹², far too
            # many points to test one by one.
            pts = star(rng, 1000, 1_000_000)
        else:
            # A comb: a non-convex plot of 998 vertices, 249 thin teeth on a wide base.
            pts = [(-1_000_000, -1_000_000)]
            x = -999_000
            for _ in range(249):
                pts += [(x, 0), (x, 900_000 + rng.randint(0, 99_000)), (x + 1000, 900_000), (x + 1000, 0)]
                x += 7_000
            pts.append((1_000_000, -1_000_000))
            pts.reverse()
        k = rng.randrange(len(pts))
        pts = pts[k:] + pts[:k]  # start anywhere
        return render(pts)
    if seed % 5 == 0:
        # Edge shapes: a unit square, a thin triangle with no inside point, collinear
        # vertices along an edge, a clockwise L, a square around the origin.
        kind = rng.randrange(5)
        shapes = [
            [(0, 0), (1, 0), (1, 1), (0, 1)],
            [(0, 0), (7, 1), (3, 1)],
            [(0, 0), (2, 0), (4, 0), (4, 4), (0, 4)],
            [(0, 0), (0, 5), (2, 5), (2, 2), (6, 2), (6, 0)],
            [(-20, -20), (20, -20), (20, 20), (-20, 20)],
        ]
        return render(shapes[kind])
    pts = star(rng, rng.randint(3, 9), rng.randint(3, 12), rng.randint(-5, 5), rng.randint(-5, 5))
    if rng.random() < 0.5:
        pts.reverse()
    return render(pts)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
