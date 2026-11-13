"""Random manuscripts; one big one has a long pasted passage."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 100_000
        if seed % 2:
            s = [rng.choice("abcd") for _ in range(n)]
            a, b = rng.randint(0, 20_000), rng.randint(50_000, 70_000)
            s[b:b + 20_000] = s[a:a + 20_000]
            return {"text": "".join(s)}
        return {"text": "".join(rng.choice("ab") for _ in range(n))}
    n = rng.randint(1, 12)
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
