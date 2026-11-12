"""Sales logs and window lengths."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 2·10⁵ days, k = half of them: re-summing every window costs 10¹⁰ additions.
        # Values below 100 keep the file small; the constraint allows up to 10⁴.
        n = 200_000
        sales = [rng.randint(0, 99) for _ in range(n)]
        sales[rng.randrange(n)] = 10**4
        k = n // 2
    else:
        n = rng.randint(1, 8)
        sales = [rng.randint(0, 20) for _ in range(n)]
        k = rng.randint(1, n)
    return {"sales": sales, "k": k}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
