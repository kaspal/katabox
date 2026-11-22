"""Random seasons for top-scores--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, k = 6, 3
    elif size == "max":
        n = 100_000
        k = rng.choice([1, 100, 50_000])
    else:
        n = rng.randint(1, 12)
        k = rng.randint(1, n)
    return {"scores": [rng.randint(0, 300) for _ in range(n)], "k": k}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
