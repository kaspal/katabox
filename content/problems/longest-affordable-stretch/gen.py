"""Random stage costs; max tests have budgets that allow very long stretches."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n costs and a budget of half their total.
        costs = [rng.randint(0, 10**4) for _ in range(size)]
        return {"costs": costs, "budget": sum(costs) // 2}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
