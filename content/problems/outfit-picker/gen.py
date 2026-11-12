"""Costume categories and a budget."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        prices = [[rng.randint(1, 1000) for _ in range(6)] for _ in range(6)]
        return {"prices": prices, "budget": 6000}
    k = rng.randint(1, 4)
    hi = rng.choice([5, 50, 1000])
    prices = [[rng.randint(1, hi) for _ in range(rng.randint(1, 4))] for _ in range(k)]
    budget = rng.randint(1, min(6000, hi * k + 3))
    return {"prices": prices, "budget": budget}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
