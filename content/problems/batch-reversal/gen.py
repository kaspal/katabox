"""Random conveyor lines and batch sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 100_000
        return {"line": [rng.randint(1, 99) for _ in range(n)], "k": rng.choice([2, 7, 1000, 99_999, 100_000])}
    n = rng.randint(0, 10)
    return {"line": [rng.randint(1, 20) for _ in range(n)], "k": rng.randint(1, 5)}


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
