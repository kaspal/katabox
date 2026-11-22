"""Random inputs for backpack-weight--f2 (a huge backpack, small worths)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, wmax = 4, 10**6
    elif size == "max":
        n, wmax = 100, 10**7
    else:
        n, wmax = rng.randint(1, 10), rng.choice([10, 10**7])
    weights = [rng.randint(1, wmax) for _ in range(n)]
    values = [rng.randint(0, 100) for _ in range(n)]
    cap = rng.randint(0, sum(weights))
    return {"weights": weights, "values": values, "capacity": cap}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
