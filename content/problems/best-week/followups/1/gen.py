"""Random profit logs for best-week--f1 (losses allowed)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n = 6
    elif size == "max":
        n = 200_000
    else:
        n = rng.randint(1, 12)
    k = rng.randint(1, n)
    top = 999 if size == "max" else 9
    bias = rng.choice([-top // 2, 0, top // 3])
    profits = [max(-(10**4), min(10**4, rng.randint(-top, top) + bias)) for _ in range(n)]
    return {"profits": profits, "k": k}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
