"""Toll grids."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
