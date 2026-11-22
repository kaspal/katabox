"""Random family trees and pairs for family-generations--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, q = 6, 3
    elif size == "max":
        n, q = 50_000, 50_000
    else:
        n, q = rng.randint(1, 9), rng.randint(1, 5)
    labels = list(range(n))
    rng.shuffle(labels)
    parent = [-1] * n
    deep = size == "max" and seed % 2 == 0
    if deep:
        q = 25_000  # a long chain: fewer pairs keep the test small and still rule out walking up
    for k in range(1, n):
        p = k - 1 if deep else rng.randrange(k)  # deep: one long chain
        parent[labels[k]] = labels[p]
    pairs = [[rng.randrange(n), rng.randrange(n)] for _ in range(q)]
    return {"parent": parent, "pairs": pairs}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
