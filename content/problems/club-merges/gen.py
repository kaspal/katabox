"""Players and club merges."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ players joined one after another into a growing club: relabelling by
        # scanning every player is 10¹⁰ steps.
        n = 100_000
        return {"n": n, "merges": [[0, i] for i in range(1, n)] + [[n - 1, 0]]}
    n = rng.randint(1, 8)
    merges = [[rng.randrange(n), rng.randrange(n)] for _ in range(rng.randint(1, 10))]
    return {"n": n, "merges": merges}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
