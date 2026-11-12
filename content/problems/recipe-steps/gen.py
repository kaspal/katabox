"""Recipes: steps and ordering rules."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
