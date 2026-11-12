"""Runners and finishing times."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, hi, length = 10_000, 3_000, 10
    else:
        n, hi, length = rng.randint(1, 8), rng.choice([3, 50, 100_000]), rng.randint(2, 4)
    names = set()
    while len(names) < n:
        names.add("".join(rng.choice("abcdefghij" if size == "max" else "abt") for _ in range(rng.randint(1, length))))
    names = sorted(names)
    rng.shuffle(names)
    return {"names": names, "times": [rng.randint(1, hi) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
