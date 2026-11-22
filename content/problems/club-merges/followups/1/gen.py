"""Random merges for club-merges--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m = 5, 4
    elif size == "max":
        n, m = 50_000, 60_000
    else:
        n, m = rng.randint(1, 8), rng.randint(1, 8)
    return {"n": n, "merges": [[rng.randrange(n), rng.randrange(n)] for _ in range(m)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
