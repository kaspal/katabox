"""Random sorted lists for first-at-least."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 5, "max": 10**5}.get(size, rng.randint(1, 8))
    top = 10**9 if size == "max" else 20
    a = sorted(rng.randint(-top, top) for _ in range(n))
    r = rng.random()
    x = a[-1] + rng.randint(1, 5) if r < 0.3 else rng.randint(-top - 2, top + 2)
    return {"sorted": a, "x": max(-(10**9), min(10**9, x))}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
