"""Random tower heights; max tests are 5·10^4 towers (random, and a noisy rise)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 50_000
        if seed % 2:
            heights = [rng.randint(1, 10**6) for _ in range(n)]
        else:
            heights = [min(10**6, 1 + i * 20 + rng.randint(0, 40)) for i in range(n)]
        return {"heights": heights}
    n = rng.randint(1, 10)
    top = rng.choice([3, 10, 30])
    return {"heights": [rng.randint(1, top) for _ in range(n)]}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
