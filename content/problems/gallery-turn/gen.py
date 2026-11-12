"""Random mosaics of every shape."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        r, c = (300, 300) if seed % 2 else (300, 117)
        top = 255
    else:
        r, c = rng.randint(1, 5), rng.randint(1, 5)
        top = rng.choice([1, 9, 255])
    return {"grid": [[rng.randint(0, top) for _ in range(c)] for _ in range(r)]}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
