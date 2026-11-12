"""Random rainfall and range queries; max tests have long ranges."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # The largest n and q; nearly full ranges, so re-summing each range costs about
        # n·q = 10¹⁰ additions. Small values and first ≤ 9 keep the test file compact.
        n = q = 100_000
        rain = [rng.randint(0, 99) for _ in range(n)]
        rain[rng.randrange(n)] = 10**4
        queries = [[rng.randint(0, 9), rng.randint(n - 10, n - 1)] for _ in range(q)]
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
