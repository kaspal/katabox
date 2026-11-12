"""Locker questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n is the largest limit; 50 questions between n / 2 and n.
        limits = [rng.randint(max(1, size // 2), size) for _ in range(50)]
        limits[0] = size
        return {"limits": limits}
    if size == "max":
        # Many questions with large limits: per-question trial division does about
        # q·L·√L work, the sieve runs once.
        q = 20_000
        limits = [rng.randint(900_000, 1_000_000) if rng.random() < 0.5 else rng.randint(1, 1_000_000) for _ in range(q)]
        limits[rng.randrange(q)] = 1_000_000
    else:
        limits = [rng.randint(1, 60) for _ in range(rng.randint(1, 6))]
    return {"limits": limits}


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
