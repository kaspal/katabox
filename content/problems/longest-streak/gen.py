"""Random value lists for longest-streak."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 7, "max": 10**5}.get(size, rng.randint(1, 10))
    k = rng.choice([2, 3, 10])
    return {"values": [rng.randint(1, k) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
