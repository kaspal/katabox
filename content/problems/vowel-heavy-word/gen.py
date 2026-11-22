"""Random word lists for vowel-heavy-word."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 4, "max": 10**4}.get(size, rng.randint(1, 6))
    words = ["".join(rng.choice("abcdeiou") for _ in range(rng.randint(1, 12))) for _ in range(n)]
    return {"words": words}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
