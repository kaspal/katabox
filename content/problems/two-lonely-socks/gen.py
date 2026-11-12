"""Pairs of sock ids plus two lonely ids."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): about n socks, partners far apart, the two lonely ones last.
        values = rng.sample(range(1, 10**6), max(4, size // 2 + 2))
        a, b = values.pop(), values.pop()
        return {"ids": values + values + [a, b]}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
