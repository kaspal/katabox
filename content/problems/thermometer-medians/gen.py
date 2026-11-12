"""Temperature readings in tenths of a degree."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A long log drifting like a real day: re-sorting every prefix is hopeless.
        n = 100_000
        t = rng.randint(-300, 300)
        readings = []
        for _ in range(n):
            t = max(-10_000, min(10_000, t + rng.randint(-15, 15)))
            readings.append(t)
    else:
        n = rng.randint(1, 8)
        readings = [rng.randint(-20, 20) for _ in range(n)]
    return {"readings": readings}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
