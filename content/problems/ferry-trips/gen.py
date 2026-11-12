"""Hikers and boat limits."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Mostly heavy hikers (over half the limit), who never share: looking through
        # everyone left for a companion costs about n²/2 checks. A limit of 1000 keeps
        # the file small; the constraint allows 3·10⁴.
        n, limit = 100_000, 1000
        heavy = 80_000 if seed % 2 else 60_000
        weights = [rng.randint(501, 1000) for _ in range(heavy)]
        weights += [rng.randint(1, 500) for _ in range(n - heavy)]
        rng.shuffle(weights)
    else:
        limit = rng.randint(1, 12)
        weights = [rng.randint(1, limit) for _ in range(rng.randint(1, 8))]
    return {"weights": weights, "limit": limit}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
