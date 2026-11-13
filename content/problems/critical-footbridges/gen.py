"""Chains of islands with loops: compact data, many critical footbridges."""
import random


def chain_graph(rng, n):
    edges = [[i, i + 1] for i in range(n - 1)]
    i = 0
    while i < n - 1:
        length = rng.randint(1, 30)
        j = min(n - 1, i + length)
        if rng.random() < 0.6 and j - i >= 2:
            edges.append([i, j])
        i = j
    return edges


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n islands in a chain with loops.
        return {"n": size, "bridges": chain_graph(rng, size)}
    if size == "max":
        n = 50_000
        return {"n": n, "bridges": chain_graph(rng, n)}
    n = rng.randint(1, 7)
    pairs = set()
    for _ in range(rng.randint(0, 9)):
        if n < 2:
            break
        a, b = sorted(rng.sample(range(n), 2))
        pairs.add((a, b))
    edges = [list(p) if rng.random() < 0.5 else [p[1], p[0]] for p in sorted(pairs)]
    rng.shuffle(edges)
    return {"n": n, "bridges": edges}


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
