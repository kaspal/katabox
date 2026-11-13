"""Chains of users with back links that close loops: compact data, long reach."""
import random


def chain_graph(rng, n):
    # i -> i + 1 along a chain, plus back links that tie stretches into groups.
    links = [[i, i + 1] for i in range(n - 1)]
    i = 0
    while i < n - 1:
        length = rng.randint(1, 40)
        j = min(n - 1, i + length)
        if rng.random() < 0.6 and j > i:
            links.append([j, i])
        i = j + 1
    return links


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n users in a chain with loops.
        return {"n": size, "links": chain_graph(rng, size)}
    if size == "max":
        n = 50_000
        return {"n": n, "links": chain_graph(rng, n)}
    n = rng.randint(1, 7)
    links = set()
    for _ in range(rng.randint(0, 10)):
        if n < 2:
            break
        a, b = rng.sample(range(n), 2)
        links.add((a, b))
    return {"n": n, "links": [list(e) for e in sorted(links)]}


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
