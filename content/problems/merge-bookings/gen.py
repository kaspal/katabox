"""Random booking sheets."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        out = []
        for _ in range(10_000):
            s = rng.randint(0, 999_000)
            out.append([s, s + rng.randint(0, 300)])
        return {"bookings": out}
    out = []
    for _ in range(rng.randint(1, 7)):
        s = rng.randint(0, 20)
        out.append([s, s + rng.randint(0, 5)])
    return {"bookings": out}


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
