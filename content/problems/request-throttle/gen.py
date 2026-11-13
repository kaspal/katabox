"""Random request logs with a few busy users."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, users, limit, window = 100_000, 50, rng.randint(5, 30), rng.randint(20, 200)
    else:
        n, users, limit, window = rng.randint(1, 10), rng.randint(1, 3), rng.randint(1, 3), rng.randint(1, 8)
    ops, args, t = [], [], rng.randint(0, 5)
    for _ in range(n):
        t += rng.choice([0, 0, 1, 1, 2, 5])
        ops.append("request" if rng.random() < 0.8 else "count")
        args.append([rng.randint(1, users), t])
    return {"limit": limit, "window": window, "ops": ops, "args": args}


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
