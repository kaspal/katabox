"""Random print jobs; max tests have tight deadlines, so the answer is large."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n is the largest job. 200 jobs and one hour each: the answer is
        # about n, so trying speeds upwards costs about n rounds.
        pages = [rng.randint(1, size) for _ in range(200)]
        pages[0] = size
        return {"pages": pages, "hours": 200}
    if size == "max":
        n = 50_000
        pages = [rng.randint(1, 10**9) for _ in range(n)]
        hours = n if seed % 2 else 2 * n
        return {"pages": pages, "hours": hours}
    n = rng.randint(1, 6)
    pages = [rng.randint(1, 30) for _ in range(n)]
    return {"pages": pages, "hours": rng.randint(n, n + 25)}


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
