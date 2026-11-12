"""Temperature readings in tenths of a degree."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n readings drifting like a real day.
        t = rng.randint(-300, 300)
        readings = []
        for _ in range(size):
            t = max(-10_000, min(10_000, t + rng.randint(-15, 15)))
            readings.append(t)
        return {"readings": readings}
    if size == "max":
        # A long log drifting like a real day: re-sorting every prefix is hopeless.
        n = 100_000
        t = rng.randint(-300, 300)
        readings = []
        for _ in range(n):
            t = max(-10_000, min(10_000, t + rng.randint(-15, 15)))
            readings.append(t)
    else:
        n = rng.randint(1, 8)
        readings = [rng.randint(-20, 20) for _ in range(n)]
    return {"readings": readings}


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
