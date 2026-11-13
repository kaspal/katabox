"""Random play logs; the big one is nearly free of repeats, so stretches are long."""
import random


def long_log(rng, n):
    plays = list(range(1, n + 1))
    rng.shuffle(plays)
    for _ in range(3):
        a = rng.randrange(n)
        plays[rng.randrange(n)] = plays[a]
    return plays


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` plays with a few repeats.
        return {"plays": long_log(rng, size)}
    if size == "max":
        n = 100_000
        return {"plays": long_log(rng, n) if seed % 2 else [rng.randint(1, 300) for _ in range(n)]}
    return {"plays": [rng.randint(1, 6) for _ in range(rng.randint(1, 9))]}


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
