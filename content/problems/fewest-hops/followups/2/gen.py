"""Random one-way meshes for fewest-hops--f2."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m = rng.randint(4, 6), rng.randint(3, 6)
    elif size == "max":
        n, m = 40_000, 80_000
    else:
        n, m = rng.randint(1, 8), rng.randint(0, 12)
    links = []
    if n >= 2:
        for _ in range(m):
            a, b = rng.sample(range(n), 2)
            links.append([a, b])
    return {"n": n, "links": links}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
