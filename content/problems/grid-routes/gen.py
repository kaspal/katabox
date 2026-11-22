"""Warehouse floors ('.' free, '#' shelf) for grid-routes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        rows, cols, p = 100, 100, 0.05
    elif size == "example":
        rows, cols, p = rng.randint(2, 3), rng.randint(3, 4), 0.2
    else:
        rows, cols, p = rng.randint(1, 6), rng.randint(1, 6), 0.2
    floor = ["".join("#" if rng.random() < p else "." for _ in range(cols)) for _ in range(rows)]
    return {"floor": floor}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
