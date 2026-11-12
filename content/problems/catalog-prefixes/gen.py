"""Catalogue titles and search texts."""
import random


def word(rng, alphabet, lo, hi):
    return "".join(rng.choice(alphabet) for _ in range(rng.randint(lo, hi)))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁴ titles that share long beginnings (8 letters from "ab"), and 2·10⁴
        # searches cut from them: checking every title per search compares about
        # 2·10⁸ pairs, most of them several letters deep.
        titles = [word(rng, "ab", 8, 8) + word(rng, "abcde", 4, 12) for _ in range(10_000)]
        typed = []
        for _ in range(20_000):
            t = rng.choice(titles)
            typed.append(t[: rng.randint(6, len(t))] if rng.random() < 0.9 else word(rng, "ab", 9, 20))
    else:
        alphabet = rng.choice(["ab", "abc", "xyz"])
        titles = [word(rng, alphabet, 1, 5) for _ in range(rng.randint(1, 8))]
        typed = []
        for _ in range(rng.randint(1, 8)):
            t = rng.choice(titles)
            typed.append(t[: rng.randint(1, len(t))] if rng.random() < 0.6 else word(rng, alphabet, 1, 6))
    return {"titles": titles, "typed": typed}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
