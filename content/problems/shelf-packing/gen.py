"""Books and shelves."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 15 books that pack exactly into full shelves, which first fit decreasing misses,
        # so the search has to find the tight packing.
        width = 1000
        widths = []
        while len(widths) < 15:
            a = rng.randint(200, 500)
            b = rng.randint(200, 1000 - a - 100)
            c = width - a - b
            widths += [a, b, c]
        rng.shuffle(widths)
        return {"widths": widths[:15], "width": width}
    width = rng.choice([5, 10, 20])
    return {"widths": [rng.randint(1, width) for _ in range(rng.randint(1, 7))], "width": width}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
