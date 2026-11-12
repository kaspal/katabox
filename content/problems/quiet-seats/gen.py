"""Seat maps."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        h = w = 400
        # Mostly free seats with sold ones scattered, so many seats are checked fully.
        p = 0.03 if seed % 2 else 0.12
    else:
        h, w = rng.randint(1, 6), rng.randint(1, 6)
        p = rng.choice([0.0, 0.1, 0.3, 0.6, 1.0])
    rows = ["".join("#" if rng.random() < p else "." for _ in range(w)) for _ in range(h)]
    return {"rows": rows}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
