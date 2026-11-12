"""Villages and candidate cables."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n villages, a random spanning tree plus n / 4 extra cables.
        order = list(range(size))
        rng.shuffle(order)
        cables = [[order[i], order[rng.randrange(i)], rng.randint(1, 999)] for i in range(1, size)]
        for _ in range(size // 4):
            a, b = rng.sample(range(size), 2)
            cables.append([a, b, rng.randint(1, 999)])
        rng.shuffle(cables)
        return {"n": size, "cables": cables}
    if size == "max":
        # 2·10⁴ villages and 2.5·10⁴ cables: a random spanning tree plus extra cables, so
        # the network always connects. Rescanning every cable per village is about
        # 5·10⁸ checks. Costs up to 999 keep the file small.
        n, extra, hi = 20_000, 5_001, 999
    else:
        n, extra, hi = rng.randint(1, 5), rng.randint(0, 4), 9
    order = list(range(n))
    rng.shuffle(order)
    cables = []
    connected = size == "max" or rng.random() < 0.7
    for i in range(1, n):
        if connected or rng.random() < 0.6:
            cables.append([order[i], order[rng.randrange(i)], rng.randint(1, hi)])
    for _ in range(extra if n >= 2 else 0):
        a, b = rng.sample(range(n), 2)
        cables.append([a, b, rng.randint(1, hi)])
    rng.shuffle(cables)
    return {"n": n, "cables": cables}


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
