"""Random trails for longest-affordable-stretch--f1 (refunds allowed)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, top = 6, 9
    elif size == "max":
        n, top = 50_000, 10**4
    else:
        n, top = rng.randint(1, 12), rng.choice([5, 50])
    bias = rng.choice([0, top // 3, -top // 4])
    costs = [max(-(10**4), min(10**4, rng.randint(-top, top) + bias)) for _ in range(n)]
    budget = rng.randint(-top, top * max(1, n // 3))
    return {"costs": costs, "budget": max(-(10**9), min(10**9, budget))}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
