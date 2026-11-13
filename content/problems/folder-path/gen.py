"""Random messy paths."""
import random

PARTS = ["", ".", "..", "..", "...", "a", "b", "docs", "x_1", ".cfg", "2024", "..a"]


def path(rng, k):
    return "/" + "/".join(rng.choice(PARTS) for _ in range(k)) + rng.choice(["", "/", "//"])


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        p = path(rng, 900)
        return {"path": p[:3000]}
    return {"path": path(rng, rng.randint(0, 6))}


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
