"""Bowling scores."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 100_000
        scores = [rng.randint(0, 300) for _ in range(n)]
        k = [5, 1000, n][seed % 3]
    else:
        n = rng.randint(1, 8)
        scores = [rng.randint(0, 12) for _ in range(n)]
        k = rng.randint(1, n)
    return {"scores": scores, "k": k}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
