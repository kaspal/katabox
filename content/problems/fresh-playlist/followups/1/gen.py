"""Random play logs for fresh-playlist--f1."""
import random

def _plays(rng, size):
    if size == "example":
        n, songs = 8, 4
    elif size == "max":
        n, songs = 100_000, rng.choice([3, 1000, 10**9])
    else:
        n, songs = rng.randint(1, 12), rng.choice([2, 4, 20])
    return [rng.randint(1, songs) for _ in range(n)]


def generate(seed, size):
    return {"plays": _plays(random.Random(seed), size)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
