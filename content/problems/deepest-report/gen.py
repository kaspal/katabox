"""Company charts for deepest-report: one boss, no cycles; max tests are long chains."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 5 * 10**4
    elif size == "example":
        n = rng.randint(4, 6)
    else:
        n = rng.randint(1, 9)
    # A random order; each person reports to someone earlier in it.
    order = list(range(n))
    rng.shuffle(order)
    chain = size == "max" and seed % 2 == 1
    if chain:
        # Person 0 at the very bottom: a top-down recursion starts 5·10⁴ calls deep.
        k = order.index(0)
        order[k], order[-1] = order[-1], order[k]
    manager = [-1] * n
    for i in range(1, n):
        boss = order[i - 1] if chain else order[rng.randrange(max(0, i - 3), i)]
        manager[order[i]] = boss
    return {"manager": manager}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
