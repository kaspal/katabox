"""Random limits and totals; the big ones near 10¹⁵."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        limit = rng.randint(10**14, 10**15)
        return {"limit": limit, "total": rng.randint(30, 90)}
    limit = rng.randint(1, 400)
    return {"limit": limit, "total": rng.randint(1, 20)}


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
