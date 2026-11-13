"""Random groves; the big cases are strictly convex polygons (every tree a corner)."""
import math
import random


def convex(rng, n):
    """n trees on a strictly convex polygon: edges are distinct primitive integer
    vectors sorted by angle (half of them, plus their opposites, so it closes)."""
    m = 2
    while True:
        half = [(dx, dy) for dx in range(0, m + 1) for dy in range(-m, m + 1)
                if (dx > 0 or dy > 0) and math.gcd(dx, dy) == 1]
        if len(half) >= n // 2:
            break
        m += 4
    chosen = rng.sample(half, max(2, n // 2))
    edges = sorted(chosen + [(-dx, -dy) for dx, dy in chosen], key=lambda v: math.atan2(v[1], v[0]))
    x = y = 0
    pts = []
    for dx, dy in edges:
        pts.append([x, y])
        x, y = x + dx, y + dy
    rng.shuffle(pts)
    return pts


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n trees, all of them corners.
        return {"trees": convex(rng, size)}
    if size == "max":
        # Gift wrapping is n² steps here (every tree is a corner).
        return {"trees": convex(rng, 40_000)}
    n = rng.randint(1, 8)
    return {"trees": [[rng.randint(-6, 6), rng.randint(-6, 6)] for _ in range(n)]}


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
