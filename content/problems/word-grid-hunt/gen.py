"""Random grids and dictionaries; the big cases use two letters, so many paths match."""
import random
import string


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        r = c = 10
        grid = ["".join(rng.choice("ab") for _ in range(c)) for _ in range(r)]
        words = []
        for _ in range(30_000):
            length = rng.randint(6, 8)
            w = "".join(rng.choice("ab") for _ in range(length - 1))
            # Most words end in a letter the grid lacks: they share long prefixes with
            # real paths but are never found.
            words.append(w + ("c" if rng.random() < 0.97 else rng.choice("ab")))
        return {"grid": grid, "words": words}
    r, c = rng.randint(1, 4), rng.randint(1, 4)
    letters = rng.sample(string.ascii_lowercase[:6], 3)
    grid = ["".join(rng.choice(letters) for _ in range(c)) for _ in range(r)]
    words = ["".join(rng.choice(letters) for _ in range(rng.randint(1, 5))) for _ in range(rng.randint(1, 8))]
    return {"grid": grid, "words": words}


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
