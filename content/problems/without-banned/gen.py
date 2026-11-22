"""Random item lists for without-banned: banned items often come in a row."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 6, "max": 10**5}.get(size, rng.randint(1, 10))
    k = rng.choice([3, 6, 50])
    items = [rng.randint(1, k) for _ in range(n)]
    banned = rng.sample(range(1, k + 1), rng.randint(0, min(k, 3)))
    return {"items": items, "banned": banned}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
