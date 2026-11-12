"""Valleys: a loop with tree branches."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A loop of 6·10⁴ villages with a branch of 4·10⁴ hanging off it: one search
        # per road is about 10¹⁰ steps.
        n, loop = 100_000, 60_000
        roads = [[i, i + 1] for i in range(loop - 1)] + [[loop - 1, 0]]
        roads += [[0 if i == loop else i - 1, i] for i in range(loop, n)]
        return {"n": n, "roads": roads}
    n = rng.randint(3, 9)
    loop = rng.randint(3, n)
    edges = [(i, (i + 1) % loop) for i in range(loop)]
    for v in range(loop, n):
        edges.append((rng.randrange(v), v))
    label = list(range(n))
    rng.shuffle(label)
    roads = []
    for a, b in edges:
        a, b = label[a], label[b]
        roads.append([a, b] if rng.random() < 0.5 else [b, a])
    rng.shuffle(roads)
    return {"n": n, "roads": roads}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
