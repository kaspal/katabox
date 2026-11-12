"""Strokes on a fence."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 60_000
        if seed % 2:
            # Short, separate strokes along the fence: the list of runs grows to n, so
            # merging each stroke by scanning the list costs about n²/2 checks. Listed
            # in fence order, which keeps the file small (the order does not help the
            # scan).
            strokes = []
            for i in range(n):
                start = 10 * i + rng.randint(0, 4)
                strokes.append([start, start + rng.randint(1, 5)])
        else:
            # Long, heavily overlapping strokes over the whole fence.
            strokes = []
            for _ in range(n):
                start = rng.randint(0, 10**9 - 1)
                strokes.append([start, min(10**9, start + rng.randint(1, 30_000))])
    else:
        strokes = []
        for _ in range(rng.randint(1, 6)):
            start = rng.randint(0, 15)
            strokes.append([start, start + rng.randint(1, 6)])
    return {"strokes": strokes}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
