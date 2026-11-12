"""Pairs of tile bags: shuffled twins, or twins with one tile changed."""
import random
import string


def bags(rng, n, alphabet, twins):
    a = [rng.choice(alphabet) for _ in range(n)]
    b = a[:]
    rng.shuffle(b)
    if not twins:
        k = rng.randrange(len(b))
        if rng.random() < 0.3:
            b.pop(k) if len(b) > 1 else b.append(rng.choice(alphabet))
        else:
            b[k] = rng.choice([c for c in alphabet if c != b[k]] or ["z"])
    return {"a": "".join(a), "b": "".join(b)}


def generate(seed, size):
    rng = random.Random(seed)
    twins = seed % 2 == 1
    if size == "max":
        return bags(rng, 100_000, string.ascii_lowercase, twins)
    alphabet = rng.choice(["ab", "abc", string.ascii_lowercase])
    return bags(rng, rng.randint(1, 7), alphabet, twins)


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
