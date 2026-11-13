"""Random trains and track counts (often more tracks than wagons)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10_000
        return {"train": [rng.randint(1, 999) for _ in range(n)], "k": rng.randint(1, 100)}
    n = rng.randint(0, 9)
    return {"train": [rng.randint(1, 20) for _ in range(n)], "k": rng.randint(1, 6)}


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
