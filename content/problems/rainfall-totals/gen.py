"""Random rainfall and range queries; max tests have long ranges."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 50_000
        q = 50_000 if seed % 2 else 25_000
        rain = [rng.randint(0, 10**4) for _ in range(n)]
        queries = []
        for _ in range(q):
            first = rng.randint(0, n // 10)
            last = rng.randint(n - n // 10, n - 1)
            queries.append([first, last])
    else:
        n = rng.randint(1, 8)
        rain = [rng.randint(0, 9) for _ in range(n)]
        queries = []
        for _ in range(rng.randint(1, 5)):
            a, b = sorted((rng.randrange(n), rng.randrange(n)))
            queries.append([a, b])
    return {"rain": rain, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
