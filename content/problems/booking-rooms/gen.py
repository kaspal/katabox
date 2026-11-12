"""Random meeting lists; small ones reuse boundaries so back-to-back meetings are common."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n meetings, overlapping heavily; small times keep the data lean.
        meetings = []
        for _ in range(size):
            start = rng.randint(0, 10**7)
            meetings.append([start, start + rng.randint(1, 10**5)])
        return {"meetings": meetings}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
