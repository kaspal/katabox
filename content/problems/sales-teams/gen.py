"""Companies as boss arrays, with sales."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): one chain of command of n people.
        return {"boss": [-1] + list(range(size - 1)), "sales": [1] * size}
    if size == "max":
        # One chain of command of 10⁵ people (lean data: sales of 1 each): adding to
        # every boss above is about 5·10⁹ additions.
        n = 100_000
        return {"boss": [-1] + list(range(n - 1)), "sales": [1] * n}
    n = rng.randint(1, 9)
    shape = rng.choice(["random", "line", "star"])
    above = [-1]
    for k in range(1, n):
        above.append(k - 1 if shape == "line" else 0 if shape == "star" else rng.randrange(k))
    label = list(range(n))
    rng.shuffle(label)
    boss = [0] * n
    for k in range(n):
        boss[label[k]] = -1 if above[k] < 0 else label[above[k]]
    hi = rng.choice([1, 9, 10_000])
    return {"boss": boss, "sales": [rng.randint(0, hi) for _ in range(n)]}


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
