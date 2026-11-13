"""Random banners with few letters, so palindromes are common."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        letters = "ab" if seed % 2 else "abc"
        return {"text": "".join(rng.choice(letters) for _ in range(1500))}
    return {"text": "".join(rng.choice("abc") for _ in range(rng.randint(1, 10)))}


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
