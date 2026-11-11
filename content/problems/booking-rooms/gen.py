"""Random meeting lists; small ones reuse boundaries so back-to-back meetings are common."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 20_000
        meetings = []
        for _ in range(n):
            start = rng.randint(0, 10**9 - 1)
            length = rng.randint(1, 10**7) if seed % 2 else rng.randint(1, 10**9)
            meetings.append([start, min(10**9, start + length)])
        return {"meetings": meetings}
    meetings = []
    for _ in range(rng.randint(1, 7)):
        start = rng.randint(0, 9)
        meetings.append([start, start + rng.randint(1, 5)])
    return {"meetings": meetings}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
