"""Hillside height maps."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n cells, a square hillside of random heights.
        side = max(1, round(size ** 0.5))
        return {"heights": [[rng.randint(0, 100_000) for _ in range(side)] for _ in range(side)]}
    if size == "max":
        # A 200 × 200 hillside of random heights: neighbour differences take tens of
        # thousands of distinct values, and the answer sits well up among them, so
        # trying them one by one means thousands of full searches. Heights up to 10⁵
        # keep the file small.
        rows = cols = 200
        hi = 100_000
    else:
        rows, cols = rng.randint(1, 5), rng.randint(1, 5)
        hi = rng.choice([3, 20, 1000])
    return {"heights": [[rng.randint(0, hi) for _ in range(cols)] for _ in range(rows)]}


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
