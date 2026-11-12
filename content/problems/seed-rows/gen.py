"""Garden beds and plant widths."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): the bed length is n; 50 kinds, some very narrow.
        widths = [rng.randint(1, 100) for _ in range(50)]
        widths[:3] = [1, 1, 2]
        return {"length": size, "widths": widths}
    if size == "max":
        # A 10⁴ cm bed and 50 kinds, some very narrow: the number of plantings is
        # astronomical, so walking through them never ends.
        widths = [rng.randint(1, 100) for _ in range(50)]
        widths[:3] = [1, 1, 2]
        return {"length": 10_000, "widths": widths}
    k = rng.randint(1, 4)
    widths = [rng.randint(1, 4) for _ in range(k)]
    return {"length": rng.randint(1, 12), "widths": widths}


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
