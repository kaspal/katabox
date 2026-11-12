"""Hierarchies (boss arrays) and disputes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A hierarchy that is one line of 5·10⁴ members, and 5·10⁴ disputes between
        # members near the bottom: walking up from both is about 5·10⁹ steps.
        n, q = 50_000, 50_000
        boss = [-1] + list(range(n - 1))
        pairs = [[n - 1 - rng.randrange(100), n - 1 - rng.randrange(100)] for _ in range(q)]
        return {"boss": boss, "pairs": pairs}
    n = rng.randint(1, 10)
    shape = rng.choice(["random", "random", "line", "star"])
    above = [-1]
    for k in range(1, n):
        above.append(k - 1 if shape == "line" else 0 if shape == "star" else rng.randrange(k))
    label = list(range(n))
    rng.shuffle(label)
    boss = [0] * n
    for k in range(n):
        boss[label[k]] = -1 if above[k] < 0 else label[above[k]]
    pairs = [[rng.randrange(n), rng.randrange(n)] for _ in range(rng.randint(1, 6))]
    return {"boss": boss, "pairs": pairs}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
