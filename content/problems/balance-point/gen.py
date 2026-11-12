"""Rows of forces; some built to balance at a chosen index."""
import random


def balanced_at(rng, n, p, lo, hi):
    """n forces in [lo, hi] that balance at index p (when the numbers allow it)."""
    w = [rng.randint(lo, hi) for _ in range(n)]
    diff = sum(w[:p]) - sum(w[p + 1:])  # add `diff` to the right side
    k = p + 1
    while diff != 0 and k < n:
        step = max(lo - w[k], min(hi - w[k], diff))
        w[k] += step
        diff -= step
        k += 1
    return w


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n positive forces (a balance point, if any, near the middle).
        return {"weights": [rng.randint(1, 99) for _ in range(size)]}
    if size == "max":
        n = 100_000
        if seed % 2:
            # Positive forces: a balance point, if any, is near the middle, so summing
            # both sides for every index before it costs about n²/2 additions.
            w = [rng.randint(1, 99) for _ in range(n)]
        else:
            w = balanced_at(rng, n, n - 50, -99, 99)
    else:
        n = rng.randint(1, 8)
        if rng.random() < 0.5 and n >= 1:
            w = balanced_at(rng, n, rng.randrange(n), -9, 9)
        else:
            w = [rng.randint(-9, 9) for _ in range(n)]
    return {"weights": w}


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
