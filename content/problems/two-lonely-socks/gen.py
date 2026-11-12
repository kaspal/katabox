"""Pairs of sock ids plus two lonely ids."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 50 000 pairs, each partner 50 000 positions away, and the two lonely socks
        # last, so scanning for partners costs about n²/2 checks. Ids below 10⁶ keep
        # the file small.
        values = rng.sample(range(1, 10**6), 50_002)
        a, b = values.pop(), values.pop()
        ids = values + values + [a, b]
    else:
        k = rng.randint(0, 4)
        values = rng.sample(range(1, 40), k + 2)
        ids = values[:k] * 2 + values[k:]
        rng.shuffle(ids)
    return {"ids": ids}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
