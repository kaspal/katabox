"""Road networks."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): a toll-1 chain of n towns listed from the far end, plus 1.5·n
        # expensive short cuts that never help, as in the max test.
        roads = [[i, i + 1, 1] for i in range(size - 2, -1, -1)]
        for _ in range(size * 3 // 2):
            a = rng.randrange(size - 1)
            roads.append([a, min(size - 1, a + rng.randint(1, 9_999)), 10**4])
        return {"n": size, "roads": roads, "start": 0, "goal": size - 1}
    if size == "max":
        n = 20_000
        if seed % 2:
            # The cheapest route is a chain 0 → 1 → … → n−1 of roads with toll 1, listed
            # from the far end. Each extra road joins towns less than 10⁴ apart with toll
            # 10⁴, more than the chain between them, so it never helps. Relaxing every
            # road per round gains one town per round: about n·m = 10⁹ steps.
            roads = [[i, i + 1, 1] for i in range(n - 2, -1, -1)]
            for _ in range(30_000):
                a = rng.randrange(n - 1)
                b = min(n - 1, a + rng.randint(1, 9_999))
                roads.append([a, b, 10**4])
            start, goal = 0, n - 1
        else:
            # A random network (smaller than the limits, to keep the file small).
            roads = []
            for _ in range(15_000):
                a, b = rng.sample(range(n), 2)
                roads.append([a, b, rng.randint(1, 999)])
            start, goal = rng.randrange(n), rng.randrange(n)
    else:
        n = rng.randint(1, 6)
        roads = []
        if n >= 2:
            for _ in range(rng.randint(0, 8)):
                a, b = rng.sample(range(n), 2)
                roads.append([a, b, rng.randint(1, 9)])
        start, goal = rng.randrange(n), rng.randrange(n)
    return {"n": n, "roads": roads, "start": start, "goal": goal}


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
