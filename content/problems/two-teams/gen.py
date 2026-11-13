"""Random rivalry graphs: bipartite ones (two hidden teams), plus an odd cycle sometimes."""
import random


def graph(rng, n, m, odd):
    team = [rng.randrange(2) for _ in range(n)]
    seen, edges = set(), []
    tries = 0
    while len(edges) < m and tries < 20 * m + 100:
        tries += 1
        a, b = rng.randrange(n), rng.randrange(n)
        if a == b or (min(a, b), max(a, b)) in seen or team[a] == team[b]:
            continue
        seen.add((min(a, b), max(a, b)))
        edges.append([a, b])
    if odd and n >= 3:
        a, b = rng.sample([i for i in range(n)], 2)
        while team[a] != team[b]:
            a, b = rng.sample(range(n), 2)
        if (min(a, b), max(a, b)) not in seen:
            edges.append([a, b])
    rng.shuffle(edges)
    return edges


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 20_000
        return {"n": n, "rivalries": graph(rng, n, 30_000, seed % 2 == 0)}
    n = rng.randint(1, 7)
    return {"n": n, "rivalries": graph(rng, n, rng.randint(0, 8), rng.random() < 0.5)}


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
