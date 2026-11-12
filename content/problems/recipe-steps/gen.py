"""Recipes: steps and ordering rules."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n steps and 2·n rules that respect a hidden order.
        rank = list(range(size))
        rng.shuffle(rank)
        rules = []
        for _ in range(2 * size):
            a, b = rng.sample(range(size), 2)
            if rank[a] > rank[b]:
                a, b = b, a
            rules.append([a, b])
        return {"n": size, "before": rules}
    if size == "max":
        # 2·10⁴ steps and 4·10⁴ rules that respect a hidden random order (so there is
        # no cycle and every step gets done): rescanning everything before each step is
        # about 2·10⁴ × 6·10⁴ operations.
        n, m = 20_000, 40_000
    else:
        n, m = rng.randint(1, 7), rng.randint(0, 8)
    rank = list(range(n))
    rng.shuffle(rank)
    rules = []
    for _ in range(m):
        if n < 2:
            break
        a, b = rng.sample(range(n), 2)
        if rank[a] > rank[b]:
            a, b = b, a
        rules.append([a, b])
    if size != "max" and n >= 2 and rng.random() < 0.3:
        a, b = rng.sample(range(n), 2)
        rules.append([b, a] if [a, b] in rules else [a, b])
        rules.append([a, b] if [b, a] in rules else [b, a])
    return {"n": n, "before": rules}


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
