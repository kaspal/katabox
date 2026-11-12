"""Gift prices and a target."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n even prices and an odd target (no exact trio).
        return {"prices": [2 * rng.randint(1, 500_000) for _ in range(size)], "target": 2 * rng.randint(1, 1_400_000) + 1}
    if size == "max":
        # Even prices and an odd target: no trio hits it exactly, so the sweep never
        # stops early. Every trio is about 1.3·10⁹ for 2000 gifts.
        n = 2000
        prices = [2 * rng.randint(1, 500_000) for _ in range(n)]
        target = 2 * rng.randint(1, 1_400_000) + 1
    else:
        n = rng.randint(3, 8)
        prices = [rng.randint(1, 30) for _ in range(n)]
        target = rng.randint(1, 100)
    return {"prices": prices, "target": target}


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
