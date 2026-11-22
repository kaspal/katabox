"""Random toll meshes for cheapest-route--f2."""
import random

def _roads(rng, n, m, top):
    roads = []
    if n >= 2:
        for _ in range(m):
            a, b = rng.sample(range(n), 2)
            roads.append([a, b, rng.randint(1, top)])
    return roads


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m, top = 5, 6, 9
    elif size == "max":
        n, m, top = 20_000, 50_000, 10**4
    else:
        n, m, top = rng.randint(1, 7), rng.randint(0, 9), rng.choice([3, 20])
    return {"n": n, "roads": _roads(rng, n, m, top), "start": rng.randrange(n), "goal": rng.randrange(n)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
