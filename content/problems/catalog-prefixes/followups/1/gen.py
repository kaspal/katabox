"""Random catalogue changes for catalog-prefixes--f1."""
import random


def _word(rng, letters, top):
    return "".join(rng.choice(letters) for _ in range(rng.randint(1, top)))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, letters, top = 6, "ab", 3
    elif size == "max":
        n, letters, top = 30_000, rng.choice(["ab", "abcdefghijklmnopqrstuvwxyz"]), 20
    else:
        n, letters, top = rng.randint(1, 10), rng.choice(["ab", "abc"]), 3
    ops, words, live = [], [], []
    for _ in range(n):
        r = rng.random()
        if live and r < 0.2:
            w = rng.choice(live)
            live.remove(w)
            ops.append("remove")
            words.append(w)
        elif r < 0.6:
            w = _word(rng, letters, top)
            live.append(w)
            ops.append("add")
            words.append(w)
        else:
            base = rng.choice(live) if live and rng.random() < 0.7 else _word(rng, letters, top)
            ops.append("count")
            words.append(base[: rng.randint(1, len(base))])
    if "count" not in ops:
        ops.append("count")
        words.append(letters[0])
    return {"ops": ops, "words": words}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
