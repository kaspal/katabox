"""Archipelagos: random lines, or a long chain listed from its far end."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A chain 0-1-2-...-49 999 whose lines are listed from the far end, plus 5·10⁴
        # islands without ferries. Relabelling until stable needs one pass per island
        # of the chain: about 2.5·10⁹ steps.
        chain = 50_000
        n = 100_000
        links = [[i, i + 1] for i in range(chain - 2, -1, -1)]
    else:
        n = rng.randint(1, 8)
        links = []
        if n >= 2:
            for _ in range(rng.randint(0, 8)):
                a, b = rng.sample(range(n), 2)
                links.append([a, b])
    return {"n": n, "links": links}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
