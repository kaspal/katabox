"""Random buildings for reachable-count."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, m = 10**4, 2 * 10**4
    elif size == "example":
        n = rng.randint(4, 6)
        m = rng.randint(2, n)
    else:
        n = rng.randint(1, 9)
        m = rng.randint(0, 2 * n)
    doors = []
    for _ in range(m if n > 1 else 0):
        a, b = rng.sample(range(n), 2)
        doors.append([a, b])
    return {"n": n, "doors": doors, "start": rng.randrange(n)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
