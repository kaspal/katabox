"""Sorted catalogues and questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n catalogue numbers and n questions, small repeated values.
        codes = sorted(rng.randint(0, 1000) for _ in range(size))
        return {"codes": codes, "queries": [rng.randint(0, 1001) for _ in range(size)]}
    if size == "max":
        # 6·10⁴ numbers and 6·10⁴ questions: counting per question is 3.6·10⁹
        # comparisons. Small, repeated numbers keep the data lean.
        n = q = 60_000
        codes = sorted(rng.randint(0, 1000) for _ in range(n))
        queries = [rng.randint(0, 1001) for _ in range(q)]
        return {"codes": codes, "queries": queries}
    hi = rng.choice([3, 20, 10**9])
    codes = sorted(rng.randint(0, hi) for _ in range(rng.randint(1, 8)))
    queries = [rng.randint(0, hi + 1) if rng.random() < 0.5 else rng.choice(codes) for _ in range(rng.randint(1, 6))]
    return {"codes": codes, "queries": queries}


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
