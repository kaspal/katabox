"""Random print jobs; max tests have tight deadlines, so the answer is large."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 50_000
        pages = [rng.randint(1, 10**9) for _ in range(n)]
        hours = n if seed % 2 else 2 * n
        return {"pages": pages, "hours": hours}
    n = rng.randint(1, 6)
    pages = [rng.randint(1, 30) for _ in range(n)]
    return {"pages": pages, "hours": rng.randint(n, n + 25)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
