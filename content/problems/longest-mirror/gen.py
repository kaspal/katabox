"""Random lines; the big ones are one repeated letter or nearly so."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 3000
        if seed % 2:
            return {"text": "a" * n}
        s = ["a"] * n
        for _ in range(3):
            s[rng.randrange(n)] = "b"
        return {"text": "".join(s)}
    n = rng.randint(1, 10)
    return {"text": "".join(rng.choice("abc") for _ in range(n))}


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
