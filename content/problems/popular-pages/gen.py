"""Random traffic; the big one fills a large cache, then keeps adding new pages."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        cap = 50_000
        ops = ["put"] * cap
        args = [[p, p % 10] for p in range(1, cap + 1)]
        for _ in range(70_000):
            if rng.random() < 0.5:
                ops.append("get")
                args.append([rng.randint(1, 60_000)])
            else:
                ops.append("put")
                args.append([rng.randint(1, 120_000), rng.randint(0, 9)])
        return {"capacity": cap, "ops": ops, "args": args}
    cap = rng.randint(1, 3)
    ops, args = [], []
    for _ in range(rng.randint(1, 12)):
        page = rng.randint(1, 5)
        if rng.random() < 0.5:
            ops.append("get")
            args.append([page])
        else:
            ops.append("put")
            args.append([page, rng.randint(0, 9)])
    return {"capacity": cap, "ops": ops, "args": args}


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
