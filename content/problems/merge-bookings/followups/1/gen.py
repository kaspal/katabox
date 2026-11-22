"""Random booking sheets for merge-bookings--f1."""
import random


def _sheet(rng, n, top, length):
    out = []
    for _ in range(n):
        s = rng.randint(0, top)
        out.append([s, min(top, s + rng.randint(0, length))])
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, top, length = 3, 20, 5
    elif size == "max":
        n, top, length = 10_000, 10**6, rng.choice([20, 500])
    else:
        n, top, length = rng.randint(1, 5), rng.choice([10, 40]), rng.randint(0, 8)
    return {"roomA": _sheet(rng, n, top, length), "roomB": _sheet(rng, min(10_000, max(1, n + rng.randint(-1, 1))), top, length)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
