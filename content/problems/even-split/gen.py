"""Trip payments in euros and cents."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 20_000
        # Whole euros and cents, like real receipts; some friends paid nothing.
        paid = [rng.randint(0, 1_000_000) / 100 if rng.random() < 0.8 else 0.0 for _ in range(n)]
    else:
        n = rng.randint(1, 8)
        paid = [rng.randint(0, 5000) / 100 for _ in range(n)]
    return {"paid": paid}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
