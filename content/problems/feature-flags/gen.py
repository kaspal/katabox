"""User settings and questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n users and n questions with 8-bit masks (one full mask).
        users = [rng.getrandbits(8) for _ in range(size)]
        users[0] = 65_535
        return {"users": users, "queries": [rng.getrandbits(8) | rng.getrandbits(8) for _ in range(size)]}
    if size == "max":
        # 10⁵ users and 10⁵ questions: testing every pair is 10¹⁰ checks. 8-bit masks
        # keep the data lean; one value uses the top bit, so the table has all 2¹⁶
        # entries.
        n = q = 100_000
        users = [rng.getrandbits(8) for _ in range(n)]
        users[0] = 65_535
        queries = [rng.getrandbits(8) | rng.getrandbits(8) for _ in range(q)]
        return {"users": users, "queries": queries}
    bits = rng.choice([2, 4, 16])
    users = [rng.getrandbits(bits) for _ in range(rng.randint(1, 8))]
    queries = [rng.getrandbits(bits) & rng.getrandbits(bits) for _ in range(rng.randint(1, 6))]
    return {"users": users, "queries": queries}


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
