"""Random sorted checkout lines."""
import random


def lines_of(rng, sizes, top):
    return [sorted(rng.randint(1, top) for _ in range(s)) for s in sizes]


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n customers in n / 10 lines of 10.
        k = max(1, size // 10)
        return {"lines": lines_of(rng, [10] * k, 10**6)}
    if size == "max":
        # Tickets up to 1000 keep the data lean (sorted repeats compress well).
        if seed % 2:
            # Many short lines: scanning every front per customer is 10⁹ steps.
            return {"lines": lines_of(rng, [10] * 10_000, 1000)}
        # Two long lines.
        return {"lines": lines_of(rng, [50_000, 50_000], 1000)}
    k = rng.randint(0, 5)
    return {"lines": lines_of(rng, [rng.randint(0, 5) for _ in range(k)], 20)}


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
