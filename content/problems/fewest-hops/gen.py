"""Radio meshes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A chain 0 - 1 - ... - (n−1) listed from the far end: each sweep over the links
        # moves the distance only one station forward, so repeated sweeps take n rounds
        # of n links (10¹⁰ steps).
        n = 100_000
        links = [[i, i + 1] for i in range(n - 2, -1, -1)]
        return {"n": n, "links": links}
    n = rng.randint(1, 8)
    m = rng.randint(0, 10) if n > 1 else 0
    links = []
    for _ in range(m):
        a, b = rng.sample(range(n), 2)
        links.append([a, b])
    return {"n": n, "links": links}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
