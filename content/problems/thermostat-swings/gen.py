"""Temperature logs."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n readings drifting slowly, windows of n / 2.
        t, readings = 0, []
        for _ in range(size):
            t = max(-10_000, min(10_000, t + rng.randint(-3, 3)))
            readings.append(t)
        return {"readings": readings, "k": max(1, size // 2)}
    if size == "max":
        # 10⁵ readings and windows of 5·10⁴: scanning every window is 2.5·10⁹ steps.
        # A slow drift keeps the data lean and the queues busy.
        n, k = 100_000, 50_000
        t, readings = 0, []
        for _ in range(n):
            t = max(-10_000, min(10_000, t + rng.randint(-3, 3)))
            readings.append(t)
        return {"readings": readings, "k": k}
    n = rng.randint(1, 10)
    hi = rng.choice([2, 20, 10_000])
    readings = [rng.randint(-hi, hi) for _ in range(n)]
    return {"readings": readings, "k": rng.randint(1, n)}


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
