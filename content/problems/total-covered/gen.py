"""Random shifts for total-covered."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, top, span = 10**4, 10**6, 300
    elif size == "example":
        n, top, span = rng.randint(3, 4), 20, 8
    else:
        n, top, span = rng.randint(1, 8), 30, 12
    shifts = []
    for _ in range(n):
        s = rng.randint(0, top - 1)
        shifts.append([s, min(top, s + rng.randint(1, span))])
    return {"shifts": shifts}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
