"""Snack prices and a budget."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Cheap snacks and a budget that buys almost all of them, so a fresh scan for
        # the cheapest one runs about n times. Prices up to 999 keep the file small.
        n = 100_000
        prices = [rng.randint(1, 999) for _ in range(n)]
        budget = sum(prices) - rng.randint(0, 2000)
    else:
        prices = [rng.randint(1, 30) for _ in range(rng.randint(1, 8))]
        budget = rng.randint(0, sum(prices) + 5)
    return {"prices": prices, "budget": budget}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
