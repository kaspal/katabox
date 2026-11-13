"""Random seat maps, increasing along rows and columns."""
import random


def seat_map(rng, r, c):
    g = [[0] * c for _ in range(r)]
    for i in range(r):
        for j in range(c):
            base = max(g[i - 1][j] if i else 0, g[i][j - 1] if j else 0)
            g[i][j] = base + rng.randint(1, 4)
    return g


def generate(seed, size):
    rng = random.Random(seed)
    r, c = (300, 300) if size == "max" else (rng.randint(1, 4), rng.randint(1, 4))
    g = seat_map(rng, r, c)
    if rng.random() < 0.5:
        ticket = g[rng.randrange(r)][rng.randrange(c)]
    else:
        ticket = rng.randint(0, g[-1][-1] + 2)
    return {"seats": g, "ticket": ticket}


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
