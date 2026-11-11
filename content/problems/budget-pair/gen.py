"""Random sorted price lists and budgets."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 50_000
        prices = sorted(rng.randint(1, 10**6) for _ in range(n))
        budget = rng.randint(10**5, 2 * 10**6)
    else:
        n = rng.randint(2, 8)
        prices = sorted(rng.randint(1, 20) for _ in range(n))
        budget = rng.randint(1, 40)
    return {"prices": prices, "budget": budget}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
