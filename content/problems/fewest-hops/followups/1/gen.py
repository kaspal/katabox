"""Random meshes for fewest-hops--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m = rng.randint(4, 6), rng.randint(3, 6)
    elif size == "max":
        n, m = 50_000, rng.choice([49_999, 100_000])
    else:
        n, m = rng.randint(1, 9), rng.randint(0, 12)
    links = []
    if size == "max" and m == 49_999:
        # 0, n - 1, n - 2, ..., 1: the longest path, in an order that compresses well
        # (the test budget, ADR 0018) but is not the label order.
        order = [0] + list(range(n - 1, 0, -1))
        links = [[order[i], order[i + 1]] for i in range(n - 1)]  # one long path
    elif n >= 2:
        for _ in range(m):
            a, b = rng.sample(range(n), 2)
            links.append([a, b])
    return {"n": n, "links": links}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
