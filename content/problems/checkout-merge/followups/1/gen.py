"""Random checkout lines for checkout-merge--f1 (as arrays: ADR 0025)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        k, total, top = 3, 8, 9
    elif size == "max":
        k, total, top = rng.choice([2, 1000, 10**4]), 100_000, rng.choice([1000, 10**6])
    else:
        k, total, top = rng.randint(0, 4), rng.randint(0, 12), rng.choice([5, 30])
    lines = [[] for _ in range(k)]
    for _ in range(total if k else 0):
        lines[rng.randrange(k)].append(rng.randint(1, top))
    return {"lines": [sorted(l) for l in lines]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
