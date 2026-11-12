"""Locker questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Many questions with large limits: per-question trial division does about
        # q·L·√L work, the sieve runs once.
        q = 20_000
        limits = [rng.randint(900_000, 1_000_000) if rng.random() < 0.5 else rng.randint(1, 1_000_000) for _ in range(q)]
        limits[rng.randrange(q)] = 1_000_000
    else:
        limits = [rng.randint(1, 60) for _ in range(rng.randint(1, 6))]
    return {"limits": limits}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
