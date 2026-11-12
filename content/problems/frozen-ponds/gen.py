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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
