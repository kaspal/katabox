"""Random ledgers for stock-ledger--f1 (deliveries to a row of shelves)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, q = 5, 5
    elif size == "max":
        n, q = 40_000, 40_000
    else:
        n, q = rng.randint(1, 8), rng.randint(1, 8)
    stock = [rng.randint(0, 999 if size == "max" else 20) for _ in range(n)]
    ops, args = [], []
    for _ in range(q):
        a, b = sorted(rng.randrange(n) for _ in range(2))
        if rng.random() < 0.5:
            ops.append("add")
            args.append([a, b, rng.randint(0, 999 if size == "max" else 9)])
        else:
            ops.append("total")
            args.append([a, b])
    if "total" not in ops:
        ops.append("total")
        args.append([0, n - 1])
    return {"stock": stock, "ops": ops, "args": args}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
