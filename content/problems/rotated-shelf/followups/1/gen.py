"""Random rotated shelves and free-number questions for rotated-shelf--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, q, step, kmax = 100_000, 20_000, rng.choice([2, 4]), rng.choice([400_000, 10**9])
    elif size == "example":
        n, q, step, kmax = rng.randint(3, 6), rng.randint(2, 4), 3, 12
    else:
        n, q, step, kmax = rng.randint(1, 10), rng.randint(1, 6), rng.choice([1, 2, 4]), 25
    x = rng.randint(1, 3 * step)
    sorted_shelf = []
    for _ in range(n):
        sorted_shelf.append(x)
        x += rng.randint(1, step)
    cut = rng.randrange(n)
    shelf = sorted_shelf[cut:] + sorted_shelf[:cut]
    queries = [rng.randint(1, kmax) for _ in range(q)]
    return {"shelf": shelf, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
