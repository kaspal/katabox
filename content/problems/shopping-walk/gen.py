"""Random markets; the big one has its two rare products far apart."""
import random
import string


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): `size` stalls of 8 kinds and a list of one of each.
        return {"stalls": "".join(rng.choice("abcdefgh") for _ in range(size)), "need": "abcdefgh"}
    if size == "max":
        n = 100_000
        if seed % 2:
            # One `y` near the start and one `z` near the end: every cover is long.
            s = [rng.choice(string.ascii_lowercase[:24]) for _ in range(n)]
            s[rng.randint(500, 1500)] = "y"
            s[rng.randint(n - 1500, n - 500)] = "z"
            return {"stalls": "".join(s), "need": "yz" + "".join(rng.choice("abcd") for _ in range(20))}
        s = "".join(rng.choice("abc") for _ in range(n))
        need = "".join(rng.choice("abc") for _ in range(rng.randint(30_000, 34_000)))
        return {"stalls": s, "need": need}
    s = "".join(rng.choice("abcd") for _ in range(rng.randint(1, 10)))
    need = "".join(rng.choice("abcd") for _ in range(rng.randint(1, 4)))
    return {"stalls": s, "need": need}


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
