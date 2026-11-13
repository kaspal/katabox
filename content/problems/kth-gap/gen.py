"""Random roads of weather posts; the big one has 5·10⁴ posts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` posts on a 10⁶ km road.
        n = max(2, size)
        return {"posts": [rng.randint(0, 10**6) for _ in range(n)], "k": rng.randint(1, n * (n - 1) // 2)}
    if size == "max":
        n = 50_000
        return {"posts": [rng.randint(0, 10**6) for _ in range(n)], "k": rng.randint(1, n * (n - 1) // 2)}
    n = rng.randint(2, 7)
    posts = [rng.randint(0, 12) for _ in range(n)]
    return {"posts": posts, "k": rng.randint(1, n * (n - 1) // 2)}


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
