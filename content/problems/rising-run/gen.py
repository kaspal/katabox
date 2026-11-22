"""Height lists with many repeats for rising-run."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, top = 10**5, 5000
    elif size == "example":
        n, top = rng.randint(5, 7), 5
    else:
        n, top = rng.randint(1, 10), 6
    return {"heights": [rng.randint(1, top) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
