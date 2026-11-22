"""Random meeting lists for booking-rooms--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, top, length = 5, 60, 30
    elif size == "max":
        n, top, length = 20_000, 10**9, rng.choice([10**4, 10**7])
    else:
        n, top, length = rng.randint(1, 8), rng.choice([10, 50]), rng.randint(1, 20)
    meetings = []
    for _ in range(n):
        s = rng.randint(0, top - 1)
        meetings.append([s, min(top, s + rng.randint(1, length))])
    return {"meetings": meetings}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
