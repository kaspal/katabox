"""Toll grids."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n districts, a square city with the winding cheap road of the
        # max test.
        side = max(2, round(size ** 0.5))
        tolls = [[1] * side if r % 2 == 0 else [10_000] * side for r in range(side)]
        for r in range(1, side, 2):
            tolls[r][side - 1 if r % 4 == 1 else 0] = 1
        return {"tolls": tolls}
    if size == "max":
        # A 150 × 150 city whose cheap districts form a winding road: rows of 1-coin
        # districts joined alternately at the right and the left end, separated by
        # expensive rows. Half of the road runs right to left, against a row-order
        # sweep, so repeated sweeps need thousands of rounds.
        n = 150
        tolls = [[1] * n if r % 2 == 0 else [10_000] * n for r in range(n)]
        for r in range(1, n, 2):
            tolls[r][n - 1 if r % 4 == 1 else 0] = 1
        return {"tolls": tolls}
    rows, cols = rng.randint(1, 5), rng.randint(1, 5)
    hi = rng.choice([2, 9, 10_000])
    return {"tolls": [[rng.randint(1, hi) for _ in range(cols)] for _ in range(rows)]}


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
