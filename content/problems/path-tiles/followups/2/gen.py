"""Path lengths for path-tiles--f2 (no two long slabs together)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        return {"n": rng.randint(4, 7)}
    if size == "max":
        return {"n": rng.randint(9 * 10**5, 10**6)}
    return {"n": rng.randint(1, 18)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
