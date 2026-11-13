"""Random fences; the big one has 10⁵ panels."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` panels up to 999.
        return {"panels": [rng.randint(0, 999) for _ in range(size)]}
    if size == "max":
        n = 100_000
        if seed % 2:
            return {"panels": [rng.randint(0, 999) for _ in range(n)]}
        # A slow rise and fall: wide posters.
        return {"panels": [min(i, n - i) // 10 + rng.randint(0, 3) for i in range(n)]}
    return {"panels": [rng.randint(0, 6) for _ in range(rng.randint(1, 9))]}


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
