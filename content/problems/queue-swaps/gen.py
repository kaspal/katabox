"""Rows of children."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ children with small heights (lean test data, many ties): checking every
        # pair is 5·10⁹ comparisons.
        return {"heights": [rng.randint(1, 1000) for _ in range(100_000)]}
    n = rng.randint(1, 9)
    hi = rng.choice([2, 5, 10**9])
    heights = [rng.randint(1, hi) for _ in range(n)]
    if rng.random() < 0.2:
        heights.sort(reverse=rng.random() < 0.5)
    return {"heights": heights}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
