"""Random logs and long-day questions for mileage-log--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Even seeds: small values, which keep the test compact (ADR 0018).
        n, q, top = 100_000, 12_000, 1000 if seed % 2 else 3
    elif size == "example":
        n, q, top = rng.randint(4, 7), 3, 40
    else:
        n, q, top = rng.randint(1, 10), rng.randint(1, 6), rng.choice([3, 15, 1000])
    miles = [rng.randint(0, top) for _ in range(n)]
    trips = []
    for _ in range(q):
        a = rng.randrange(n)
        b = rng.randrange(n) if size != "max" else rng.randint(max(a, n // 2), n - 1)
        a, b = min(a, b), max(a, b)
        trips.append([a, b, rng.randint(0, min(1000, top + 1))])
    return {"miles": miles, "trips": trips}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
