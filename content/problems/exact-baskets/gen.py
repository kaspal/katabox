"""Distinct weights for exact-baskets; the target is often reachable."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 12
    elif size == "example":
        n = rng.randint(4, 5)
    else:
        n = rng.randint(1, 8)
    weights = rng.sample(range(1, 31), n)
    k = rng.randint(1, n)
    if rng.random() < 0.8:
        target = sum(rng.sample(weights, k))
    else:
        target = rng.randint(1, 300)
    return {"weights": weights, "k": k, "target": target}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
