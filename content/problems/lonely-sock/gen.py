"""Pairs of sock ids plus one lonely id."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 49 999 pairs: the first half in one order, the partners in the same order in
        # the second half (each partner far away), the lonely sock last. Ids below 10⁶
        # keep the file small.
        pairs = rng.sample(range(1, 10**6), 50_000)
        lonely = pairs.pop()
        ids = pairs + pairs + [lonely]
    else:
        k = rng.randint(0, 4)
        values = rng.sample(range(1, 30), k + 1)
        ids = values[:k] * 2 + [values[k]]
        rng.shuffle(ids)
    return {"ids": ids}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
