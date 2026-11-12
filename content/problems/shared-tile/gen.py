"""Floor sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n is the floor size; consecutive sides share no factor.
        return {"width": size + 1, "height": size}
    if size == "max":
        # Two large consecutive numbers share no factor: trying sizes downwards takes
        # about 10¹² steps, Euclid a few dozen.
        a = rng.randint(9 * 10**11, 10**12 - 1)
        return {"width": a + 1, "height": a} if seed % 2 else {"width": a, "height": a + 1}
    g = rng.randint(1, 30)
    return {"width": g * rng.randint(1, 40), "height": g * rng.randint(1, 40)}


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
