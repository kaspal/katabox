"""Random belts, sometimes with only one or two kinds."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = 100_000 if size == "max" else rng.randint(1, 9)
    kinds = rng.choice([[0, 1, 2], [0, 1, 2], [0, 2], [1], [2, 1]])
    return {"items": [rng.choice(kinds) for _ in range(n)]}


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
