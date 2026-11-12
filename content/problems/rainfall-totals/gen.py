"""Random rainfall and range queries; max tests have long ranges."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n days and n nearly full ranges.
        rain = [rng.randint(0, 99) for _ in range(size)]
        queries = [[rng.randint(0, min(9, size - 1)), rng.randint(max(0, size - 10), size - 1)] for _ in range(size)]
        return {"rain": rain, "queries": queries}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
