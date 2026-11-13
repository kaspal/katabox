"""Random ledgers; nearly full totals in the big cases."""
import random


def day(rng, n, q, top, wide):
    ops, args = [], []
    for _ in range(q):
        if rng.random() < 0.25:
            ops.append("set")
            args.append([rng.randrange(n), rng.randint(0, top)])
        else:
            ops.append("total")
            if wide:
                a = rng.randint(0, min(9, n - 1))
                args.append([a, rng.randint(max(a, n - 10), n - 1)])
            else:
                a = rng.randrange(n)
                args.append([a, rng.randint(a, n - 1)])
    return ops, args


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n shelves and n / 2 operations, totals nearly full.
        ops, args = day(rng, size, max(1, size // 2), 9, True)
        return {"stock": [rng.randint(0, 9) for _ in range(size)], "ops": ops, "args": args}
    if size == "max":
        n = 100_000
        ops, args = day(rng, n, 90_000, 9, True)
        return {"stock": [rng.randint(0, 9) for _ in range(n)], "ops": ops, "args": args}
    n = rng.randint(1, 7)
    ops, args = day(rng, n, rng.randint(1, 8), 20, False)
    return {"stock": [rng.randint(0, 20) for _ in range(n)], "ops": ops, "args": args}


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
