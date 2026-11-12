"""Villages and candidate cables."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
