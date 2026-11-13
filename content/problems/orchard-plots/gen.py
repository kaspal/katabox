"""Random orchards; the big one asks about 10⁵ nearly whole plots."""
import random


def orchard(rng, r, c, top):
    return [[rng.randint(0, top) for _ in range(c)] for _ in range(r)]


def plot(rng, r, c):
    t, b = sorted(rng.randrange(r) for _ in range(2))
    l, rt = sorted(rng.randrange(c) for _ in range(2))
    return [t, l, b, rt]


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): a 60 × 60 orchard and `size` random plots.
        return {"grid": orchard(rng, 60, 60, 9), "queries": [plot(rng, 60, 60) for _ in range(size)]}
    if size == "max":
        n = 300
        queries = [[rng.randint(0, 9), rng.randint(0, 9), rng.randint(n - 10, n - 1), rng.randint(n - 10, n - 1)]
                   for _ in range(100_000)]
        return {"grid": orchard(rng, n, n, 9), "queries": queries}
    r, c = rng.randint(1, 4), rng.randint(1, 4)
    return {"grid": orchard(rng, r, c, 9), "queries": [plot(rng, r, c) for _ in range(rng.randint(1, 5))]}


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
