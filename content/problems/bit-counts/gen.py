"""Random table sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"n": 100_000 - (seed - 1) * rng.randint(1, 999)}
    return {"n": rng.randint(0, 40)}


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
