"""Random corrections; nearly full ranges in the big cases."""
import random


def day(rng, n, q, wide):
    ops, args = [], []
    for _ in range(q):
        if rng.random() < 0.3:
            ops.append("read")
            args.append([rng.randrange(n)])
        else:
            ops.append("add")
            if wide:
                a = rng.randint(0, min(9, n - 1))
                b = rng.randint(max(a, n - 10), n - 1)
            else:
                a = rng.randrange(n)
                b = rng.randint(a, n - 1)
            args.append([a, b, rng.choice([-1, 1]) * rng.randint(0, 9 if wide else 20)])
    return ops, args


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n gauges and n / 2 operations, ranges nearly full.
        ops, args = day(rng, size, max(1, size // 2), True)
        return {"readings": [rng.randint(0, 9) for _ in range(size)], "ops": ops, "args": args}
    if size == "max":
        n = 100_000
        ops, args = day(rng, n, 60_000, True)
        return {"readings": [rng.randint(0, 9) for _ in range(n)], "ops": ops, "args": args}
    n = rng.randint(1, 7)
    ops, args = day(rng, n, rng.randint(1, 8), False)
    return {"readings": [rng.randint(-20, 20) for _ in range(n)], "ops": ops, "args": args}


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
