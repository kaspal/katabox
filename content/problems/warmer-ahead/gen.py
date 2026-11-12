"""Seasons of noon temperatures."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 100_000
        if seed % 2:
            # A long flat season with short cold snaps and no warmer day at all:
            # scanning forward from each 12° day reaches the end every time.
            temps = [12] * n
            for _ in range(2000):
                k = rng.randrange(n)
                temps[k] = rng.randint(-50, 11)
        else:
            # A random season with a slow warming trend.
            temps = [max(-50, min(50, -40 + 80 * i // n + rng.randint(-10, 10))) for i in range(n)]
    else:
        temps = [rng.randint(-3, 3) for _ in range(rng.randint(1, 9))]
    return {"temps": temps}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
