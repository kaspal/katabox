"""Receipts for till-total; cent values that floats cannot hold exactly are common."""
import random

TRICKY = ["0.10", "0.20", "0.29", "0.57", "1.15", "4.35", "0.07", "2.01", "9.99", "19.99"]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10**4
    elif size == "example":
        n = rng.randint(2, 4)
    else:
        n = rng.randint(1, 8)
    prices = []
    for _ in range(n):
        if rng.random() < 0.5:
            prices.append(rng.choice(TRICKY))
        else:
            cents = rng.randint(1, 999_999 if size == "max" else 5000)
            prices.append(f"{cents // 100}.{cents % 100:02d}")
    return {"prices": prices}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
