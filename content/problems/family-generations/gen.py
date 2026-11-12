"""Family trees as parent arrays."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
