"""Sales logs and window lengths."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n days, windows of n / 2.
        return {"sales": [rng.randint(0, 99) for _ in range(size)], "k": max(1, size // 2)}
    if size == "max":
        # 2·10⁵ days, k = half of them: re-summing every window costs 10¹⁰ additions.
        # Values below 100 keep the file small; the constraint allows up to 10⁴.
        n = 200_000
        sales = [rng.randint(0, 99) for _ in range(n)]
        sales[rng.randrange(n)] = 10**4
        k = n // 2
    else:
        n = rng.randint(1, 8)
        sales = [rng.randint(0, 20) for _ in range(n)]
        k = rng.randint(1, n)
    return {"sales": sales, "k": k}


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
