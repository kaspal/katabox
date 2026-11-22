"""Random inputs for change-machine--f2 (each token at most once)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        tokens = [rng.randint(1, 9) for _ in range(5)]
        amount = sum(rng.sample(tokens, 3)) + rng.choice([0, 0, 1])
    elif size == "max":
        tokens = [rng.randint(1, 10**4) for _ in range(100)]
        amount = rng.choice([sum(rng.sample(tokens, 20)), 10**4])
        amount = min(amount, 10**4)
    else:
        tokens = [rng.randint(1, 12) for _ in range(rng.randint(1, 9))]
        amount = rng.randint(0, 40)
    return {"tokens": tokens, "amount": amount}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
