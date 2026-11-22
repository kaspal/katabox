"""Random fleet logs for mileage-log--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        rows, cols, q = 2, 4, 3
    elif size == "max":
        rows = rng.choice([1, 100, 1000])
        cols, q = 50_000 // rows, 50_000
    else:
        rows, cols, q = rng.randint(1, 4), rng.randint(1, 5), rng.randint(1, 5)
    top = 99 if size == "max" else (1000 if size != "example" else 20)
    miles = [[rng.randint(0, top) for _ in range(cols)] for _ in range(rows)]
    queries = []
    for _ in range(q):
        v1, v2 = sorted(rng.randrange(rows) for _ in range(2))
        d1, d2 = sorted(rng.randrange(cols) for _ in range(2))
        queries.append([v1, d1, v2, d2])
    return {"miles": miles, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
