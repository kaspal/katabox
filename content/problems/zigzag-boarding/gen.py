"""Random boarding queues."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n passengers.
        return {"queue": [rng.randint(1, 99) for _ in range(size)]}
    if size == "max":
        # Small numbers keep the data lean; fetching the last node each time is n² / 2 steps.
        return {"queue": [rng.randint(1, 99) for _ in range(100_000)]}
    return {"queue": [rng.randint(1, 20) for _ in range(rng.randint(0, 9))]}


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
