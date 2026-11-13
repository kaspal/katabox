"""Random logs and trips; long trips in the big cases."""
import random


def wide(rng, n):
    a = rng.randint(0, min(9, n - 1))
    b = rng.randint(max(a, n - 10), n - 1)
    return [a, b]


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n days and n / 2 nearly full trips.
        return {"miles": [rng.randint(0, 9) for _ in range(size)], "trips": [wide(rng, size) for _ in range(max(1, size // 2))]}
    if size == "max":
        # Small values and nearly full trips keep the data lean; summing each trip
        # directly is 7.5·10⁹ additions.
        n = 100_000
        return {"miles": [rng.randint(0, 9) for _ in range(n)], "trips": [wide(rng, n) for _ in range(75_000)]}
    n = rng.randint(1, 8)
    trips = []
    for _ in range(rng.randint(1, 6)):
        a = rng.randint(0, n - 1)
        trips.append([a, rng.randint(a, n - 1)])
    return {"miles": [rng.randint(0, 20) for _ in range(n)], "trips": trips}


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
