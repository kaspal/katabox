"""Pairs of words: one made from the other with a few typos, or unrelated."""
import random
import string


def typos(rng, word, k, alphabet):
    w = list(word)
    for _ in range(k):
        op = rng.choice("idr") if len(w) > 1 else "i"
        p = rng.randrange(len(w))
        if op == "i":
            w.insert(p, rng.choice(alphabet))
        elif op == "d":
            w.pop(p)
        else:
            w[p] = rng.choice(alphabet)
    return "".join(w)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        alphabet = string.ascii_lowercase
        meant = "".join(rng.choice(alphabet) for _ in range(1000))
        if seed % 2:
            typed = typos(rng, meant, 120, alphabet)[:1000]
        else:
            typed = "".join(rng.choice(alphabet[:4]) for _ in range(1000))
    else:
        alphabet = rng.choice(["ab", "abc", "xyz"])
        meant = "".join(rng.choice(alphabet) for _ in range(rng.randint(1, 4)))
        typed = typos(rng, meant, rng.randint(0, 3), alphabet)[:5] or "a"
    return {"typed": typed, "meant": meant}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
