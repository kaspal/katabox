"""Random road networks for delivery-times."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, m, top = 10**4, 3 * 10**4, 1000
    elif size == "example":
        n, m, top = rng.randint(4, 5), rng.randint(4, 6), 9
    else:
        n, m, top = rng.randint(1, 8), rng.randint(0, 14), 20
    roads = []
    for _ in range(m if n > 1 else 0):
        a, b = rng.sample(range(n), 2)
        roads.append([a, b, rng.randint(1, top)])
    return {"n": n, "roads": roads, "start": rng.randrange(n)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
