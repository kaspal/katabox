"""Random inputs for two-sum--f2 (closest pair sum)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, spread = rng.randint(4, 6), 20
    elif size == "max":
        n, spread = 100_000, rng.choice([1000, 10**6])
    else:
        n, spread = rng.randint(2, 12), rng.choice([5, 50, 10**9])
    nums = [rng.randint(-spread, spread) for _ in range(n)]
    target = rng.randint(-2 * spread, 2 * spread)
    return {"nums": nums, "target": max(-(2 * 10**9), min(2 * 10**9, target))}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
