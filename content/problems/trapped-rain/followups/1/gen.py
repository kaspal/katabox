"""Random gardens for trapped-rain--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        rows, cols, top = 4, 5, 6
    elif size == "max":
        rows, cols, top = 200, 200, rng.choice([20, 10**4])
    else:
        rows, cols, top = rng.randint(1, 5), rng.randint(1, 5), rng.choice([3, 9])
    return {"heights": [[rng.randint(0, top) for _ in range(cols)] for _ in range(rows)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
