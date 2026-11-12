"""Orchards."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
