"""Random operation lists for recent-pages--f1 (least frequently used)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        cap, n, pages = 2, rng.randint(6, 8), 4
    elif size == "max":
        cap, n, pages = rng.choice([1000, 50_000]), 100_000, 100_000
    else:
        cap, n, pages = rng.randint(1, 4), rng.randint(1, 16), rng.choice([3, 6, 50])
    ops, args = [], []
    for _ in range(n):
        p = rng.randint(1, pages)
        if rng.random() < 0.5:
            ops.append("get")
            args.append([p])
        else:
            ops.append("put")
            args.append([p, rng.randint(0, 999)])
    return {"capacity": cap, "ops": ops, "args": args}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
