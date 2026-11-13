"""Random stamping sessions; the big one has 6·10⁴ nearly whole-sheet stamps."""
import random


def stamp(rng, r, c):
    t, b = sorted(rng.randrange(r) for _ in range(2))
    l, rt = sorted(rng.randrange(c) for _ in range(2))
    return [t, l, b, rt]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 500
        stamps = [[rng.randint(0, 19), rng.randint(0, 19), rng.randint(n - 20, n - 1), rng.randint(n - 20, n - 1)]
                  for _ in range(60_000)]
        return {"rows": n, "cols": n, "stamps": stamps}
    r, c = rng.randint(1, 5), rng.randint(1, 5)
    return {"rows": r, "cols": c, "stamps": [stamp(rng, r, c) for _ in range(rng.randint(1, 6))]}


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
