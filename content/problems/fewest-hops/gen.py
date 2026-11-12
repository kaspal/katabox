"""Radio meshes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): a chain of n stations listed from the far end, as in the max test.
        return {"n": size, "links": [[i, i + 1] for i in range(size - 2, -1, -1)]}
    if size == "max":
        # A chain 0 - 1 - ... - (n−1) listed from the far end: each sweep over the links
        # moves the distance only one station forward, so repeated sweeps take n rounds
        # of n links (10¹⁰ steps).
        n = 100_000
        links = [[i, i + 1] for i in range(n - 2, -1, -1)]
        return {"n": n, "links": links}
    n = rng.randint(1, 8)
    m = rng.randint(0, 10) if n > 1 else 0
    links = []
    for _ in range(m):
        a, b = rng.sample(range(n), 2)
        links.append([a, b])
    return {"n": n, "links": links}


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
