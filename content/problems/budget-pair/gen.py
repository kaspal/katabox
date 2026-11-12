"""Random sorted price lists and budgets."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n sorted prices.
        prices = sorted(rng.randint(1, 10**6) for _ in range(size))
        return {"prices": prices, "budget": rng.randint(10**5, 2 * 10**6)}
    if size == "max":
        n = 50_000
        prices = sorted(rng.randint(1, 10**6) for _ in range(n))
        budget = rng.randint(10**5, 2 * 10**6)
    else:
        n = rng.randint(2, 8)
        prices = sorted(rng.randint(1, 20) for _ in range(n))
        budget = rng.randint(1, 40)
    return {"prices": prices, "budget": budget}


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
