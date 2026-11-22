"""Sorted price lists with many repeats for distinct-pairs."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, top = 10**5, 1000
    elif size == "example":
        n, top = rng.randint(5, 7), 6
    else:
        n, top = rng.randint(1, 10), 8
    prices = sorted(rng.randint(1, top) for _ in range(n))
    if n >= 2 and rng.random() < 0.8:
        budget = prices[rng.randrange(n)] + prices[rng.randrange(n)]
    else:
        budget = rng.randint(2, 2 * top)
    return {"prices": prices, "budget": budget}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
