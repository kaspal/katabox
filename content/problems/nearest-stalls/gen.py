"""Random stalls around the entrance (small coordinates give many ties)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n stalls, k = n / 2.
        return {"stalls": [[rng.randint(-300, 300), rng.randint(-300, 300)] for _ in range(size)], "k": max(1, size // 2)}
    if size == "max":
        n = 80_000
        return {"stalls": [[rng.randint(-300, 300), rng.randint(-300, 300)] for _ in range(n)], "k": n // 2}
    n = rng.randint(1, 8)
    return {"stalls": [[rng.randint(-3, 3), rng.randint(-3, 3)] for _ in range(n)], "k": rng.randint(1, n)}


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
