"""Random checkout lines for checkout-merge--f1 (as arrays: ADR 0025)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        k, total, top = rng.choice([1000, 10**4]), 100_000, rng.choice([10**5, 10**6])
    elif size == "example":
        k, total, top = 3, 9, 30
    else:
        k = rng.randint(1, 4)
        total, top = rng.randint(k, 12), rng.choice([5, 30])
    lines = [[rng.randint(1, top)] for _ in range(k)]
    for _ in range(total - k):
        lines[rng.randrange(k)].append(rng.randint(1, top))
    return {"lines": [sorted(line) for line in lines]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
