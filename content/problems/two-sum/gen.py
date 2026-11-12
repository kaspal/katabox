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
    if isinstance(size, int):
        # Profile (ADR 0024): n numbers, all even except the last one, and the target odd.
        # A pair summing to it needs that odd number and the even number before it (2·v,
        # which no other number repeats): one answer, at the very end. Small values keep
        # the data lean.
        v = rng.randint(-1000, 1000)
        nums = [2 * x for x in (rng.randint(-500_000, 500_000) for _ in range(size - 2)) if x != v]
        nums = nums + [2 * rng.randint(1001, 400_000) for _ in range(size - 2 - len(nums))]
        nums += [2 * v, 2 * rng.randint(-1000, 1000) + 1]
        return {"nums": nums, "target": nums[-2] + nums[-1]}
    if size == "max":
        n, lo, hi = 100_000, -10**9, 10**9
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
