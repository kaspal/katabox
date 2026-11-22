"""Random frozen marshes for frozen-ponds--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        rows, cols = 4, 5
    elif size == "max":
        rows, cols = 400, 400
    else:
        rows, cols = rng.randint(1, 6), rng.randint(1, 6)
    p = rng.choice([0.3, 0.5, 0.7])
    return {"ice": ["".join("#" if rng.random() < p else "." for _ in range(cols)) for _ in range(rows)]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
