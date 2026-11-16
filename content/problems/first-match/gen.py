"""Word lists and typed prefixes."""
import random


def word(rng, alphabet, lo, hi):
    return "".join(rng.choice(alphabet) for _ in range(rng.randint(lo, hi)))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n words and n prefixes, shaped like the max test.
        words = [word(rng, "ab", 8, 8) + word(rng, "abcde", 2, 10) for _ in range(size)]
        prefixes = []
        for _ in range(size):
            w = rng.choice(words)
            prefixes.append(w[: rng.randint(6, len(w))] if rng.random() < 0.9 else word(rng, "ab", 9, 18))
        return {"words": words, "prefixes": prefixes}
    if size == "max":
        # 10⁴ words sharing 3-letter beginnings and 2·10⁴ short prefixes cut from them:
        # scanning every word per prefix checks 2·10⁸ pairs, and each prefix matches
        # thousands of words that a scan must then compare to find the smallest.
        words = [word(rng, "ab", 3, 3) + word(rng, "abcde", 7, 17) for _ in range(10_000)]
        prefixes = []
        for _ in range(20_000):
            w = rng.choice(words)
            prefixes.append(w[: rng.randint(1, 5)] if rng.random() < 0.9 else word(rng, "ab", 4, 8))
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
