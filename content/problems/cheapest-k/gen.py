"""Random price lists for cheapest-k."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, top = 10**5, 10**4
    elif size == "example":
        n, top = rng.randint(4, 6), 30
    else:
        n, top = rng.randint(1, 10), 50
    prices = [rng.randint(1, top) for _ in range(n)]
    return {"prices": prices, "k": rng.randint(1, max(1, n - 1)) if n > 1 else 1}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
