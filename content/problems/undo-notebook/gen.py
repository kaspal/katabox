"""Notebook operations."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n / 2 writes (key 1 first), then n / 2 reads of key 1 or of a
        # key never written, as in the max test.
        h = max(1, size // 2)
        ops = ["write"] * h + ["read"] * (size - h)
        args = [[1, 7]] + [[2 + k % 19, k % 10] for k in range(h - 1)]
        args += [[1 if rng.random() < 0.5 else 21] for _ in range(size - h)]
        return {"ops": ops, "args": args}
    if size == "max":
        # 5·10⁴ writes (key 1 first, then small keys 2-20 over and over), then 5·10⁴
        # reads of key 1 or of a key never written: searching the writes from the newest
        # is 2.5·10⁹ steps. Small numbers keep the data lean.
        n = 50_000
        ops = ["write"] * n + ["read"] * n
        args = [[1, 7]] + [[2 + k % 19, k % 10] for k in range(n - 1)]
        args += [[1 if rng.random() < 0.5 else 21] for _ in range(n)]
        return {"ops": ops, "args": args}
    keys = rng.randint(1, 3)
    ops, args = [], []
    for _ in range(rng.randint(1, 14)):
        op = rng.choice(["write", "write", "read", "undo", "redo"])
        ops.append(op)
        if op == "write":
            args.append([rng.randint(1, keys), rng.randint(0, 9)])
        elif op == "read":
            args.append([rng.randint(1, keys)])
        else:
            args.append([])
    return {"ops": ops, "args": args}


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
