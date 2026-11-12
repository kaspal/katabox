"""Delivery rounds: one-way travel times between stops."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = 12 if size == "max" else rng.randint(1, 7)
    hi = 10_000 if size == "max" else rng.choice([3, 20, 10_000])
    dist = [[0 if i == j else rng.randint(1, hi) for j in range(n)] for i in range(n)]
    return {"dist": dist}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
