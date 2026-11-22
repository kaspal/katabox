"""Random inputs for backpack-weight--f1 (several copies of each item)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, wmax, cmax, cap = 3, 6, 3, rng.randint(8, 14)
    elif size == "max":
        n, wmax, cmax, cap = 50, 200, 100, 5000
    else:
        n, wmax, cmax, cap = rng.randint(1, 4), rng.choice([3, 8]), rng.randint(1, 4), rng.randint(0, 20)
    weights = [rng.randint(1, wmax) for _ in range(n)]
    values = [rng.randint(0, 100) for _ in range(n)]
    counts = [rng.randint(1, cmax) for _ in range(n)]
    return {"weights": weights, "values": values, "counts": counts, "capacity": cap}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
