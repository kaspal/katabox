"""Random mountain trails."""
import random


def rising(rng, k, start):
    out, v = [], start
    for _ in range(k):
        v += rng.randint(1, 9)
        out.append(v)
    return out


def trail(rng, n):
    up = rng.choice([0, n - 1, rng.randint(0, n - 1)])
    left = rising(rng, up + 1, rng.randint(-1, 5))
    right = rising(rng, n - up - 1, rng.randint(-1, 5))
    top = max(left[-1], (right[-1] + 1) if right else 0)
    left[-1] = top
    return left + right[::-1]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"heights": trail(rng, 100_000)}
    return {"heights": trail(rng, rng.randint(1, 8))}


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
