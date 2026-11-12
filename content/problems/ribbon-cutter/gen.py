"""Ribbon rolls and order sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n is the longest roll. 100 rolls cut into pieces of about 30 cm:
        # trying lengths downwards from the longest roll costs about n checks of every roll.
        ribbons = [rng.randint(1, size) for _ in range(100)]
        ribbons[0] = size
        return {"ribbons": ribbons, "pieces": max(1, sum(ribbons) // 30)}
    if size == "max":
        n = 30_000
        ribbons = [rng.randint(1, 10**7) for _ in range(n)]
        total = sum(ribbons)
        # Small answers (about 3000 cm, or about 150 cm for 10⁹ pieces): trying lengths
        # downwards from 10⁷ checks millions of lengths, each over every roll.
        pieces = total // 3000 if seed % 2 else 10**9
    else:
        ribbons = [rng.randint(1, 30) for _ in range(rng.randint(1, 5))]
        pieces = rng.randint(1, sum(ribbons) + 2)
    return {"ribbons": ribbons, "pieces": pieces}


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
