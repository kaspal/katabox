"""Garden beds and plant widths."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
