"""Random inputs for balanced-streaks--f1 (the longest balanced run)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n = rng.randint(4, 7)
    elif size == "max":
        n = 100_000
    else:
        n = rng.randint(1, 14)
    weights = rng.choice([(1, 1, 1), (3, 1, 1), (1, 1, 0), (5, 1, 0)])
    return {"results": "".join(rng.choices("WLD", weights=weights, k=n))}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
