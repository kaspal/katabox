"""Random cities; the big one has 6·10⁴ wide buildings."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        out = []
        for _ in range(60_000):
            left = rng.randint(0, 99_000)
            out.append([left, left + rng.randint(1, 2000), rng.randint(1, 99)])
        return {"buildings": out}
    out = []
    for _ in range(rng.randint(1, 6)):
        left = rng.randint(0, 15)
        out.append([left, left + rng.randint(1, 6), rng.randint(1, 6)])
    return {"buildings": out}


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
