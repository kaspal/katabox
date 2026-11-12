"""Players and club merges."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n players joined one after another into a growing club.
        return {"n": size, "merges": [[0, i] for i in range(1, size)] + [[size - 1, 0]]}
    if size == "max":
        # 10⁵ players joined one after another into a growing club: relabelling by
        # scanning every player is 10¹⁰ steps.
        n = 100_000
        return {"n": n, "merges": [[0, i] for i in range(1, n)] + [[n - 1, 0]]}
    n = rng.randint(1, 8)
    merges = [[rng.randrange(n), rng.randrange(n)] for _ in range(rng.randint(1, 10))]
    return {"n": n, "merges": merges}


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
