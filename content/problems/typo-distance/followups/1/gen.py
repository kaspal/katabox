"""Random inputs for typo-distance--f1: a word and a copy with a few random fixes."""
import random


def _mutate(rng, s, fixes, letters):
    s = list(s)
    for _ in range(fixes):
        op = rng.randrange(3)
        if op == 0 or not s:
            s.insert(rng.randint(0, len(s)), rng.choice(letters))
        elif op == 1 and len(s) > 1:
            del s[rng.randrange(len(s))]
        else:
            s[rng.randrange(len(s))] = rng.choice(letters)
    return "".join(s) or rng.choice(letters)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, k, letters = rng.randint(5, 8), 2, "abcde"
    elif size == "max":
        n, k, letters = 50_000, 20, "abcdefghijklmnopqrstuvwxyz"
    else:
        n, k, letters = rng.randint(1, 10), rng.randint(0, 4), rng.choice(["ab", "abc", "abcdefgh"])
    typed = "".join(rng.choice(letters) for _ in range(n))
    meant = _mutate(rng, typed, rng.randint(0, 2 * k + 2), letters)[:50_000]
    return {"typed": typed, "meant": meant, "k": k}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
