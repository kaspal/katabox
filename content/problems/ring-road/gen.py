"""Valleys: a loop with tree branches."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n villages, a loop of 0.6·n with a branch hanging off it.
        loop = max(3, size * 3 // 5)
        roads = [[i, i + 1] for i in range(loop - 1)] + [[loop - 1, 0]]
        roads += [[0 if i == loop else i - 1, i] for i in range(loop, size)]
        return {"n": size, "roads": roads}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
