"""Random inputs for inclusive-total."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 5, "max": 10**5}.get(size, rng.randint(1, 10))
    top = 1000 if size == "max" else 20
    nums = [rng.randint(-top, top) for _ in range(n)]
    a = rng.randrange(n)
    b = a if rng.random() < 0.3 else rng.randint(a, n - 1)
    return {"nums": nums, "a": a, "b": b}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
