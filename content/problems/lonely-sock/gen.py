"""Pairs of sock ids plus one lonely id."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): about n socks: pairs far apart, the lonely one last.
        pairs = rng.sample(range(1, 10**6), max(2, size // 2))
        lonely = pairs.pop()
        return {"ids": pairs + pairs + [lonely]}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
