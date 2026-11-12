"""Orchards."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n cells, a square orchard.
        side = max(1, round(size ** 0.5))
        return {"grid": [[rng.randint(0, 9) for _ in range(side)] for _ in range(side)]}
    if size == "max":
        # A full 300 × 300 orchard. Values up to 9 keep the file small; paths number
        # about 10¹⁷⁸, so exploring them is hopeless.
        rows = cols = 300
        hi = 9
    else:
        rows, cols = rng.randint(1, 5), rng.randint(1, 5)
        hi = rng.choice([1, 9, 100])
    return {"grid": [[rng.randint(0, hi) for _ in range(cols)] for _ in range(rows)]}


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
