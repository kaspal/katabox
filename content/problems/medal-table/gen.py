"""Random medal tables with many ties."""
import random
import string


def names(rng, n):
    out = set()
    while len(out) < n:
        out.add("".join(rng.choice(string.ascii_lowercase) for _ in range(rng.randint(1, 8))))
    out = list(out)
    rng.shuffle(out)
    return out


def generate(seed, size):
    rng = random.Random(seed)
    n, top = (10_000, 4) if size == "max" else (rng.randint(1, 6), 2)
    return {"countries": names(rng, n), "medals": [[rng.randint(0, top) for _ in range(3)] for _ in range(n)]}


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
