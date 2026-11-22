"""Random temperature years for warmer-ahead--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, spread = 6, 9
    elif size == "max":
        n, spread = 100_000, rng.choice([2, 50])
    else:
        n, spread = rng.randint(1, 10), rng.choice([2, 10, 50])
    return {"temps": [rng.randint(-spread, spread) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
