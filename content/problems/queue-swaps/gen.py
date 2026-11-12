"""Rows of children."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n children with small heights.
        return {"heights": [rng.randint(1, 1000) for _ in range(size)]}
    if size == "max":
        # 10⁵ children with small heights (lean test data, many ties): checking every
        # pair is 5·10⁹ comparisons.
        return {"heights": [rng.randint(1, 1000) for _ in range(100_000)]}
    n = rng.randint(1, 9)
    hi = rng.choice([2, 5, 10**9])
    heights = [rng.randint(1, hi) for _ in range(n)]
    if rng.random() < 0.2:
        heights.sort(reverse=rng.random() < 0.5)
    return {"heights": heights}


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
