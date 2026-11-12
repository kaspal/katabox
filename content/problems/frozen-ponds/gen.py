"""Frozen marsh maps."""
import random


def snake(h, w):
    # One long winding pond: full rows joined at alternating ends. Labels must travel
    # along the whole path, about h·w/2 cells.
    rows = []
    for r in range(h):
        if r % 2 == 0:
            rows.append("#" * w)
        elif r % 4 == 1:
            rows.append("." * (w - 1) + "#")
        else:
            rows.append("#" + "." * (w - 1))
    return rows


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n cells, a square map of random ice.
        side = max(1, round(size ** 0.5))
        return {"ice": ["".join("#" if rng.random() < 0.55 else "." for _ in range(side)) for _ in range(side)]}
    if size == "max":
        h = w = 400
        if seed % 2:
            return {"ice": snake(h, w)}
        p = 0.55
    else:
        h, w = rng.randint(1, 6), rng.randint(1, 6)
        p = rng.choice([0.0, 0.3, 0.5, 0.7, 1.0])
    rows = ["".join("#" if rng.random() < p else "." for _ in range(w)) for _ in range(h)]
    return {"ice": rows}


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
