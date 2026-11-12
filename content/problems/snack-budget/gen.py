"""Snack prices and a budget."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n cheap snacks and a budget that buys almost all of them.
        prices = [rng.randint(1, 999) for _ in range(size)]
        return {"prices": prices, "budget": max(0, sum(prices) - rng.randint(0, 2000))}
    if size == "max":
        # Cheap snacks and a budget that buys almost all of them, so a fresh scan for
        # the cheapest one runs about n times. Prices up to 999 keep the file small.
        n = 100_000
        prices = [rng.randint(1, 999) for _ in range(n)]
        budget = sum(prices) - rng.randint(0, 2000)
    else:
        prices = [rng.randint(1, 30) for _ in range(rng.randint(1, 8))]
        budget = rng.randint(0, sum(prices) + 5)
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
