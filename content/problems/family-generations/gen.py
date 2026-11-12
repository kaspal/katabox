"""Family trees as parent arrays."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): one line of n descendants.
        return {"parent": [-1] + list(range(size - 1))}
    if size == "max":
        # One line of 10⁵ descendants: walking up from everyone is about 5·10⁹ steps.
        n = 100_000
        return {"parent": [-1] + list(range(n - 1))}
    n = rng.randint(1, 9)
    shape = rng.choice(["random", "line", "star"])
    above = [-1]
    for k in range(1, n):
        above.append(k - 1 if shape == "line" else 0 if shape == "star" else rng.randrange(k))
    # Renumber the people so the numbers do not follow the generations.
    label = list(range(n))
    rng.shuffle(label)
    parent = [0] * n
    for k in range(n):
        parent[label[k]] = -1 if above[k] < 0 else label[above[k]]
    return {"parent": parent}


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
