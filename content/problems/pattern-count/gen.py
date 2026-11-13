"""Random cloth strips; the big ones are almost all `a`."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` letters from "ab", a short repetitive motif.
        return {"text": "".join(rng.choice("ab") for _ in range(size)), "pattern": "abab"}
    if size == "max":
        m = 100_000
        pattern = "a" * (m - 1) + ("b" if seed % 2 else "a")
        return {"text": "a" * 200_000, "pattern": pattern}
    n = rng.randint(1, 12)
    text = "".join(rng.choice("ab") for _ in range(n))
    pattern = "".join(rng.choice("ab") for _ in range(rng.randint(1, 4)))
    return {"text": text, "pattern": pattern}


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
