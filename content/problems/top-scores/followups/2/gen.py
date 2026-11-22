"""Random seasons for top-scores--f2."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, w = 6, 3
    elif size == "max":
        n = 100_000
        w = rng.choice([10, 1000, 100_000])
    else:
        n = rng.randint(1, 12)
        w = rng.randint(1, n)
    k = rng.randint(1, w)
    return {"scores": [rng.randint(0, 300) for _ in range(n)], "w": w, "k": k}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
