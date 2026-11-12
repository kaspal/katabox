"""Sorted catalogues and questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 6·10⁴ numbers and 6·10⁴ questions: counting per question is 3.6·10⁹
        # comparisons. Small, repeated numbers keep the data lean.
        n = q = 60_000
        codes = sorted(rng.randint(0, 1000) for _ in range(n))
        queries = [rng.randint(0, 1001) for _ in range(q)]
        return {"codes": codes, "queries": queries}
    hi = rng.choice([3, 20, 10**9])
    codes = sorted(rng.randint(0, hi) for _ in range(rng.randint(1, 8)))
    queries = [rng.randint(0, hi + 1) if rng.random() < 0.5 else rng.choice(codes) for _ in range(rng.randint(1, 6))]
    return {"codes": codes, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
