"""Token values and amounts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        k = 30
        if seed % 2:
            coins = rng.sample(range(2, 120), k)
        else:
            # All even: odd amounts are impossible, and only exhausting every
            # combination proves it.
            coins = rng.sample(range(2, 240, 2), k)
        amount = 20_000 - (seed % 2 == 0)
    else:
        k = rng.randint(1, 4)
        coins = rng.sample(range(1, 9), k)
        amount = rng.randint(0, 25)
    return {"coins": coins, "amount": amount}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
