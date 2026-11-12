"""Path lengths."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"n": 1_000_000 - rng.randrange(3)}
    return {"n": rng.randint(1, 18)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
