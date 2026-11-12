"""Spice stalls and a bag."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Many stalls for the sort (no naive: the greedy is the natural solution), with
        # small numbers to keep the data lean.
        n = 30_000
        weights = [rng.randint(1, 1000) for _ in range(n)]
        prices = [rng.randint(1, 1000) for _ in range(n)]
        return {"weights": weights, "prices": prices, "capacity": sum(weights) // 2 + 1}
    n = rng.randint(1, 6)
    hi = rng.choice([5, 100, 10_000])
    weights = [rng.randint(1, hi) for _ in range(n)]
    prices = [rng.randint(1, hi) for _ in range(n)]
    return {"weights": weights, "prices": prices, "capacity": rng.randint(1, sum(weights) + 5)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
