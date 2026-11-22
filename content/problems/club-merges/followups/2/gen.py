"""Random merges and undos for club-merges--f2."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m = 5, 6
    elif size == "max":
        n, m = 50_000, 60_000
    else:
        n, m = rng.randint(1, 7), rng.randint(1, 10)
    ops = []
    for _ in range(m):
        if rng.random() < 0.3:
            ops.append([0])
        else:
            ops.append([1, rng.randrange(n), rng.randrange(n)])
    return {"n": n, "ops": ops}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
