"""Strokes on a fence."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n short, separate strokes in fence order.
        strokes = []
        for i in range(size):
            start = 10 * i + rng.randint(0, 4)
            strokes.append([start, start + rng.randint(1, 5)])
        return {"strokes": strokes}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
