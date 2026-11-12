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
    if isinstance(size, int):
        # Profile (ADR 0024): both words have n letters.
        meant = "".join(rng.choice(string.ascii_lowercase) for _ in range(size))
        return {"typed": "".join(rng.choice("abcd") for _ in range(size)), "meant": meant}
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
