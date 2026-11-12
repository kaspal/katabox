"""Stamp albums and parcel prices."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 18
        if seed % 2:
            # Even stamps and an odd price: no exact postage, so the search has to
            # rule out every choice that stays under the price.
            values = [2 * rng.randint(1, 500) for _ in range(n)]
            target = sum(values) // 2 | 1
        else:
            values = [rng.randint(1, 1000) for _ in range(n)]
            target = sum(rng.sample(values, 9))
        return {"values": values, "target": target}
    n = rng.randint(1, 8)
    hi = rng.choice([3, 10, 1000])
    values = [rng.randint(1, hi) for _ in range(n)]
    if rng.random() < 0.7:
        target = sum(rng.sample(values, rng.randint(1, n)))
    else:
        target = rng.randint(1, min(18_000, sum(values) + 2))
    return {"values": values, "target": target}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
