"""Shop operations."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, items, qty = 10_000, [rng.randint(1, 10**9) for _ in range(300)], 10_000
    else:
        n, items, qty = rng.randint(1, 10), [rng.randint(1, 5) for _ in range(3)], rng.choice([3, 10])
    ops, args = [], []
    for _ in range(n):
        op = rng.choice(["stock", "sell", "sell", "count"])
        item = rng.choice(items) if rng.random() < 0.9 else rng.randint(1, 10**9)
        ops.append(op)
        args.append([item] if op == "count" else [item, rng.randint(1, qty)])
    return {"ops": ops, "args": args}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
