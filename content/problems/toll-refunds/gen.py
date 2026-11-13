"""Random toll maps; refunds are rare enough that most have no negative loop."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, m = 500, 3000
    else:
        n, m = rng.randint(1, 6), rng.randint(0, 9)
    # A hidden potential p makes c = w + p[a] − p[b] (w ≥ 0) loop-free of negative totals;
    # sometimes one road breaks it.
    p = [rng.randint(0, 300) for _ in range(n)]
    roads = []
    for _ in range(m):
        if n < 2:
            break
        a, b = rng.sample(range(n), 2)
        c = rng.randint(0, 400) + p[a] - p[b]
        roads.append([a, b, max(-1000, min(1000, c))])
    if roads and rng.random() < (0.3 if size != "max" else 0.5):
        roads[rng.randrange(len(roads))][2] = -1000
    return {"n": n, "roads": roads, "start": rng.randrange(n)}


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
