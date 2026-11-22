"""Random inputs for change-machine--f1 (count the combinations)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        coins, amount = rng.sample(range(1, 8), 3), rng.randint(5, 12)
    elif size == "max":
        coins, amount = rng.sample(range(1, 200), 30), 20_000
    else:
        coins = rng.sample(range(1, 12), rng.randint(1, 4))
        amount = rng.randint(0, 25)
    return {"coins": coins, "amount": amount}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
