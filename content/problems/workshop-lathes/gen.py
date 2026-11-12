"""Job queues and lathe counts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n jobs on n / 2 lathes.
        return {"jobs": [rng.randint(1, 999) for _ in range(size)], "lathes": max(1, size // 2)}
    if size == "max":
        # 10⁵ jobs on 5·10⁴ lathes: scanning every lathe per job is 5·10⁹ steps.
        # Jobs under 1000 minutes keep the file small; the constraint allows 10⁴.
        n = 100_000
        jobs = [rng.randint(1, 999) for _ in range(n)]
        lathes = 50_000
    else:
        jobs = [rng.randint(1, 6) for _ in range(rng.randint(1, 8))]
        lathes = rng.randint(1, 4)
    return {"jobs": jobs, "lathes": lathes}


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
