"""Random inputs for balance-point--f1 (two pivots): small values make equal parts common."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, spread = rng.randint(4, 7), 2
    elif size == "max":
        n = 100_000
        spread = rng.choice([0, 1, 2])
    else:
        n, spread = rng.randint(2, 12), rng.choice([0, 1, 2, 10])
    weights = [rng.randint(-spread, spread) for _ in range(n)]
    return {"weights": weights}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
