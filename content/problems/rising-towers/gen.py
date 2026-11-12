"""Random tower heights; max tests are 5·10^4 towers (random, and a noisy rise)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n random heights.
        return {"heights": [rng.randint(1, 10**6) for _ in range(size)]}
    if size == "max":
        n = 50_000
        if seed % 2:
            heights = [rng.randint(1, 10**6) for _ in range(n)]
        else:
            heights = [min(10**6, 1 + i * 20 + rng.randint(0, 40)) for i in range(n)]
        return {"heights": heights}
    n = rng.randint(1, 10)
    top = rng.choice([3, 10, 30])
    return {"heights": [rng.randint(1, top) for _ in range(n)]}


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
