"""Word lists and typed prefixes."""
import random


def word(rng, alphabet, lo, hi):
    return "".join(rng.choice(alphabet) for _ in range(rng.randint(lo, hi)))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁴ words sharing 8-letter beginnings and 2·10⁴ prefixes cut from them:
        # scanning every word per prefix compares about 2·10⁸ pairs, several letters deep.
        words = [word(rng, "ab", 8, 8) + word(rng, "abcde", 2, 10) for _ in range(10_000)]
        prefixes = []
        for _ in range(20_000):
            w = rng.choice(words)
            prefixes.append(w[: rng.randint(6, len(w))] if rng.random() < 0.9 else word(rng, "ab", 9, 18))
        return {"words": words, "prefixes": prefixes}
    alphabet = rng.choice(["ab", "abc", "xyz"])
    words = [word(rng, alphabet, 1, 5) for _ in range(rng.randint(1, 8))]
    prefixes = []
    for _ in range(rng.randint(1, 8)):
        w = rng.choice(words)
        prefixes.append(w[: rng.randint(1, len(w))] if rng.random() < 0.6 else word(rng, alphabet, 1, 6))
    return {"words": words, "prefixes": prefixes}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
