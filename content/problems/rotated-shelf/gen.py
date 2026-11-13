"""Random rotated shelves; the label is on the shelf about two times in three."""
import random


def rising(rng, k, start):
    out, v = [], start
    for _ in range(k):
        v += rng.randint(1, 9)
        out.append(v)
    return out


def generate(seed, size):
    rng = random.Random(seed)
    n = 100_000 if size == "max" else rng.randint(1, 8)
    s = rising(rng, n, -1)
    k = rng.randrange(n)
    shelf = s[k:] + s[:k]
    label = rng.choice(shelf) if rng.random() < 0.67 else rng.randint(0, s[-1] + 3)
    return {"shelf": shelf, "label": label}


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
