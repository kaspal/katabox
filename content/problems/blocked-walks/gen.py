"""Random street grids with roadworks; the big ones are 10⁵ × 10⁵."""
import random


def closed(rng, rows, cols, k, near_diagonal):
    out = set()
    tries = 0
    while len(out) < k and tries < 50 * k + 50:
        tries += 1
        if near_diagonal:
            r = rng.randint(0, rows)
            c = min(cols, max(0, r * cols // rows + rng.randint(-50, 50)))
        else:
            r, c = rng.randint(0, rows), rng.randint(0, cols)
        if (r, c) not in ((0, 0), (rows, cols)):
            out.add((r, c))
    pts = [list(p) for p in out]
    rng.shuffle(pts)
    return pts


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 100_000
        return {"rows": n, "cols": n, "blocked": closed(rng, n, n, 1000, seed % 2 == 0)}
    rows, cols = rng.randint(1, 5), rng.randint(1, 5)
    return {"rows": rows, "cols": cols, "blocked": closed(rng, rows, cols, rng.randint(0, 4), False)}


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
