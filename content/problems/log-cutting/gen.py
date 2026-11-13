"""Random logs and cut marks."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        length = rng.randint(10**5, 10**6)
        return {"length": length, "marks": rng.sample(range(1, length), 100)}
    length = rng.randint(2, 20)
    return {"length": length, "marks": rng.sample(range(1, length), rng.randint(1, min(6, length - 1)))}


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
