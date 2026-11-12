"""Temperature logs."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
