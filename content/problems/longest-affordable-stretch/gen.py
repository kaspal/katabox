"""Random stage costs; max tests have budgets that allow very long stretches."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 50_000
        costs = [rng.randint(0, 10**4) for _ in range(n)]
        total = sum(costs)
        budget = 10**9 if seed % 2 else total // 2
        return {"costs": costs, "budget": budget}
    n = rng.randint(1, 8)
    costs = [rng.choice([0, rng.randint(0, 10)]) for _ in range(n)]
    return {"costs": costs, "budget": rng.randint(0, 25)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
