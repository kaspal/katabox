"""Seasons of noon temperatures."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): a flat season of n days with short cold snaps, no warmer day.
        temps = [12] * size
        for _ in range(max(1, size // 50)):
            temps[rng.randrange(size)] = rng.randint(-50, 11)
        return {"temps": temps}
    if size == "max":
        n = 100_000
        if seed % 2:
            # A long flat season with short cold snaps and no warmer day at all:
            # scanning forward from each 12° day reaches the end every time.
            temps = [12] * n
            for _ in range(2000):
                k = rng.randrange(n)
                temps[k] = rng.randint(-50, 11)
        else:
            # A random season with a slow warming trend.
            temps = [max(-50, min(50, -40 + 80 * i // n + rng.randint(-10, 10))) for i in range(n)]
    else:
        temps = [rng.randint(-3, 3) for _ in range(rng.randint(1, 9))]
    return {"temps": temps}


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
