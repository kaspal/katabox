"""Random inputs for rotated-shelf--f1 (many labels)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, q, top = rng.randint(4, 7), 3, 60
    elif size == "max":
        n, q, top = 60_000, 60_000, 10**6
    else:
        n, q, top = rng.randint(1, 10), rng.randint(1, 6), rng.choice([20, 10**9])
    values = sorted(rng.sample(range(top + 1), n))
    k = rng.randrange(n)
    shelf = values[k:] + values[:k]
    labels = [rng.choice(shelf) if rng.random() < 0.6 else rng.randint(0, top) for _ in range(q)]
    return {"shelf": shelf, "labels": labels}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
