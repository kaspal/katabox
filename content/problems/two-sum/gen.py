"""Random two-sum inputs with exactly one answer (kb-pack stress test and max tests)."""
import random


def _pairs(nums, target):
    count = {}
    pairs = 0
    for x in nums:
        pairs += count.get(target - x, 0)
        count[x] = count.get(x, 0) + 1
    return pairs


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, lo, hi = 10_000, -10**9, 10**9
    else:
        n, lo, hi = rng.randint(2, 8), -20, 20
    while True:
        nums = [rng.randint(lo, hi) for _ in range(n)]
        if size == "max":
            # The answer at the very end: checking pairs in order scans everything.
            i, j = n - 2, n - 1
        else:
            i, j = sorted(rng.sample(range(n), 2))
        target = nums[i] + nums[j]
        if lo <= target <= hi and _pairs(nums, target) == 1:
            return {"nums": nums, "target": target}


if __name__ == "__main__":
    import json
    import sys

    seed = int(sys.argv[1]) if len(sys.argv) > 1 else 1
    print(json.dumps(generate(seed, sys.argv[2] if len(sys.argv) > 2 else "small")))
