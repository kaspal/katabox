"""Hillside height maps."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
