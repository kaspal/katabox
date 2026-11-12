"""Seasons of results."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): a season of n matches.
        return {"results": "".join(rng.choice("WLD") for _ in range(size))}
    if size == "max":
        # 10⁵ matches: extending every run is 5·10⁹ steps.
        return {"results": "".join(rng.choice("WLD") for _ in range(100_000))}
    letters = rng.choice(["WL", "WLD", "D", "WWLD"])
    return {"results": "".join(rng.choice(letters) for _ in range(rng.randint(1, 12)))}


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
