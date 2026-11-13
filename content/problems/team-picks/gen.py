"""Random team questions; the big one asks 5·10⁴ questions about huge squads."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` questions about squads of up to 1000.
        qs = []
        for _ in range(size):
            n = rng.randint(0, 1000)
            qs.append([n, rng.randint(0, n + 2)])
        return {"queries": qs}
    if size == "max":
        return {"queries": [[rng.randint(99_000, 100_000), rng.randint(40_000, 60_000)] for _ in range(50_000)]}
    qs = []
    for _ in range(rng.randint(1, 5)):
        n = rng.randint(0, 30)
        qs.append([n, rng.randint(0, n + 2)])
    return {"queries": qs}


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
