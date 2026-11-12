"""Texts and screen widths."""
import random

WORDS = ["a", "an", "the", "of", "sea", "light", "harbour", "boat", "wind", "lighthouse", "evening", "and", "stormy"]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        words = [rng.choice(WORDS) for _ in range(10_000)]
        return {"words": words, "width": rng.choice([7, 40, 100])}
    words = ["".join(rng.choice("xyz") for _ in range(rng.randint(1, 7))) for _ in range(rng.randint(1, 8))]
    return {"words": words, "width": rng.randint(1, 12)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
