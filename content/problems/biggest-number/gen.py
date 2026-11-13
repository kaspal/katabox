"""Random hands of cards with shared prefixes, so the order matters."""
import random


def card(rng):
    kind = rng.random()
    if kind < 0.15:
        return 0
    if kind < 0.6:
        stem = rng.choice(["4", "40", "12", "9", "98", "1"])
        return int(stem + "".join(rng.choice(stem) for _ in range(rng.randint(0, 3))))
    return rng.randint(0, 10**9)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"cards": [card(rng) for _ in range(1000)]}
    if rng.random() < 0.1:
        return {"cards": [0] * rng.randint(1, 4)}
    return {"cards": [card(rng) if rng.random() < 0.5 else rng.randint(0, 120) for _ in range(rng.randint(1, 6))]}


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
