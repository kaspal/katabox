"""Path lengths for path-tiles--f1 (very long paths)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        return {"n": rng.randint(4, 9)}
    if size == "max":
        return {"n": rng.randint(10**14, 10**15)}
    return {"n": rng.choice([rng.randint(1, 30), rng.randint(1, 3000)])}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
