"""Random archipelagos, sometimes split into separate groups of islands."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, m, q = 100, 5000, 10_000
    else:
        n, m, q = rng.randint(1, 6), rng.randint(0, 8), rng.randint(1, 6)
    groups = rng.randint(1, 3)
    group = [rng.randrange(groups) for _ in range(n)]
    routes = []
    for _ in range(m):
        if n < 2:
            break
        a, b = rng.sample(range(n), 2)
        if group[a] == group[b]:
            routes.append([a, b, rng.randint(1, 10**4 if size == "max" else 30)])
    queries = [[rng.randrange(n), rng.randrange(n)] for _ in range(q)]
    return {"n": n, "routes": routes, "queries": queries}


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
