"""Rainfall logs and span queries for span-sums."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, q, top = 10**5, 10**4, 100
    elif size == "example":
        n, q, top = rng.randint(4, 6), rng.randint(2, 3), 9
    else:
        n, q, top = rng.randint(1, 10), rng.randint(1, 6), 20
    rain = [rng.randint(0, top) for _ in range(n)]
    queries = []
    for _ in range(q):
        l = rng.randrange(n)
        queries.append([l, rng.randint(l, n - 1)])
    return {"rain": rain, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
