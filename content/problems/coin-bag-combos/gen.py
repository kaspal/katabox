"""Random coin sets whose number of ways stays at most 1000."""
import random


def ways(coins, target):
    dp = [1] + [0] * target
    for c in coins:
        for t in range(c, target + 1):
            dp[t] += dp[t - c]
    return dp[target]


def generate(seed, size):
    rng = random.Random(seed)
    while True:
        if size == "max":
            coins = rng.sample(range(2, 30), rng.randint(5, 10))
            target = rng.randint(45, 60)
            if 300 <= ways(coins, target) <= 1000:
                return {"coins": coins, "target": target}
        else:
            coins = rng.sample(range(1, 12), rng.randint(1, 4))
            target = rng.randint(1, 15)
            if ways(coins, target) <= 1000:
                return {"coins": coins, "target": target}


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
