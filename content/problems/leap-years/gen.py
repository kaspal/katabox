"""Random year ranges, often around century years."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"start": 1, "end": 9999}
    a = rng.choice([rng.randint(1, 9999), rng.choice([100, 400, 1600, 1700, 1900, 2000, 2100, 2400]) + rng.randint(-3, 3)])
    a = min(max(a, 1), 9999)
    b = min(9999, a + rng.randint(0, 30))
    return {"start": a, "end": b}


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
