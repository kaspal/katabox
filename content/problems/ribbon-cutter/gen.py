"""Ribbon rolls and order sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 30_000
        ribbons = [rng.randint(1, 10**7) for _ in range(n)]
        total = sum(ribbons)
        # Small answers (about 3000 cm, or about 150 cm for 10⁹ pieces): trying lengths
        # downwards from 10⁷ checks millions of lengths, each over every roll.
        pieces = total // 3000 if seed % 2 else 10**9
    else:
        ribbons = [rng.randint(1, 30) for _ in range(rng.randint(1, 5))]
        pieces = rng.randint(1, sum(ribbons) + 2)
    return {"ribbons": ribbons, "pieces": pieces}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
