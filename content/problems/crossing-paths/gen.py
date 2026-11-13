"""Random pairs of paths, with many collinear and touching cases."""
import random


def path(rng, r):
    while True:
        p = [rng.randint(-r, r) for _ in range(4)]
        if p[:2] != p[2:]:
            return p


def pair(rng, r):
    a = path(rng, r)
    kind = rng.random()
    if kind < 0.25:
        # On the same line as the first path.
        dx, dy = a[2] - a[0], a[3] - a[1]
        s, t = rng.randint(-3, 3), rng.randint(-3, 3)
        if s == t:
            t += 1
        b = [a[0] + s * dx, a[1] + s * dy, a[0] + t * dx, a[1] + t * dy]
        if all(abs(v) <= 10**4 for v in b):
            return a + b
    if kind < 0.45:
        # Starting at an endpoint of the first path.
        q = a[:2] if rng.random() < 0.5 else a[2:]
        b = q + [rng.randint(-r, r), rng.randint(-r, r)]
        if b[:2] != b[2:]:
            return a + b
    return a + path(rng, r)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"pairs": [pair(rng, 10**4) for _ in range(10_000)]}
    return {"pairs": [pair(rng, 4) for _ in range(rng.randint(1, 6))]}


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
