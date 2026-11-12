"""Gift prices and a target."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Even prices and an odd target: no trio hits it exactly, so the sweep never
        # stops early. Every trio is about 1.3·10⁹ for 2000 gifts.
        n = 2000
        prices = [2 * rng.randint(1, 500_000) for _ in range(n)]
        target = 2 * rng.randint(1, 1_400_000) + 1
    else:
        n = rng.randint(3, 8)
        prices = [rng.randint(1, 30) for _ in range(n)]
        target = rng.randint(1, 100)
    return {"prices": prices, "target": target}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
