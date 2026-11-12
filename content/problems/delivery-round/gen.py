"""Delivery rounds: one-way travel times between stops."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n stops.
        return {"dist": [[0 if i == j else rng.randint(1, 10_000) for j in range(size)] for i in range(size)]}
    n = 12 if size == "max" else rng.randint(1, 7)
    hi = 10_000 if size == "max" else rng.choice([3, 20, 10_000])
    dist = [[0 if i == j else rng.randint(1, hi) for j in range(n)] for i in range(n)]
    return {"dist": dist}


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
