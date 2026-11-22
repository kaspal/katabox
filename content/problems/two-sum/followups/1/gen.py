"""Random inputs for two-sum--f1 (count the pairs): many repeated values, so the count
is large and every pair of equal halves counts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, spread = rng.randint(4, 7), 4
    elif size == "max":
        # Few distinct values: about 5·10⁹ pairs at most, still JS-safe.
        n, spread = 100_000, rng.choice([3, 50])
    else:
        n, spread = rng.randint(1, 12), rng.choice([3, 10, 1000])
    nums = [rng.randint(-spread, spread) for _ in range(n)]
    if rng.random() < 0.5 and n >= 2:
        target = nums[rng.randrange(n)] + nums[rng.randrange(n)]
    else:
        target = rng.randint(-2 * spread, 2 * spread)
    target = max(-2 * 10**9, min(2 * 10**9, target))
    return {"nums": nums, "target": target}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
