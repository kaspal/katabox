"""Notebook operations."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
