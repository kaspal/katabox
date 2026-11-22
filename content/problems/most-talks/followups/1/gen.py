"""Random conference programmes for most-talks--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, top, length = 4, 12, 5
    elif size == "max":
        n, top, length = 60_000, 10**6, rng.choice([50, 5000])
    else:
        n, top, length = rng.randint(1, 8), rng.choice([10, 30]), rng.randint(1, 8)
    talks = []
    for _ in range(n):
        s = rng.randint(0, top - 1)
        e = min(top, s + rng.randint(1, length))
        talks.append([s, e, rng.randint(1, 10**4 if size == "max" else 20)])
    return {"talks": talks}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
