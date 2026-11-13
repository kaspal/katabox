"""Random packing lists; the big ones have many similar items."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 200
        w = [rng.randint(30, 120) for _ in range(n)]
        v = [x * 10 + rng.randint(-25, 25) for x in w]
        return {"weights": w, "values": v, "capacity": 10_000}
    n = rng.randint(1, 8)
    return {"weights": [rng.randint(1, 9) for _ in range(n)], "values": [rng.randint(0, 20) for _ in range(n)], "capacity": rng.randint(0, 20)}


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
