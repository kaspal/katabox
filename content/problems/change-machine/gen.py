"""Token values and amounts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): the amount is n. Ten even coins and an odd amount: no exact
        # change, so trying combinations has to exhaust them.
        return {"coins": rng.sample(range(2, 120, 2), 10), "amount": size}
    if size == "max":
        k = 30
        if seed % 2:
            coins = rng.sample(range(2, 120), k)
        else:
            # All even: odd amounts are impossible, and only exhausting every
            # combination proves it.
            coins = rng.sample(range(2, 240, 2), k)
        amount = 20_000 - (seed % 2 == 0)
    else:
        k = rng.randint(1, 4)
        coins = rng.sample(range(1, 9), k)
        amount = rng.randint(0, 25)
    return {"coins": coins, "amount": amount}


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
