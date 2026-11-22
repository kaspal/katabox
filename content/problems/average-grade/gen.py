"""Random grade lists for average-grade."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 4, "max": 10**5}.get(size, rng.randint(1, 8))
    return {"grades": [rng.randint(0, 100) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
