"""Random price histories; the big one has 2·10⁵ small prices."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` prices up to 999.
        return {"prices": [rng.randint(1, 999) for _ in range(size)]}
    if size == "max":
        return {"prices": [rng.randint(1, 999) for _ in range(200_000)]}
    return {"prices": [rng.randint(1, 12) for _ in range(rng.randint(1, 8))]}


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
