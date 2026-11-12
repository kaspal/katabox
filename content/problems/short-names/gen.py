"""Member names."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n names sharing long beginnings.
        names = set()
        while len(names) < size:
            names.add("".join(rng.choice("ab") for _ in range(8)) + "".join(rng.choice("abc") for _ in range(rng.randint(4, 12))))
        names = sorted(names)
        rng.shuffle(names)
        return {"names": names}
    if size == "max":
        # 2·10⁴ names sharing long beginnings: comparing every pair of names reads
        # about 4·10⁸ pairs, several letters deep.
        n, head, tail = 20_000, (8, "ab"), (4, 12)
    else:
        n, head, tail = rng.randint(1, 8), (rng.randint(0, 2), "ab"), (1, 3)
    names = set()
    while len(names) < n:
        start = "".join(rng.choice(head[1]) for _ in range(head[0]))
        word = start + "".join(rng.choice("abc") for _ in range(rng.randint(*tail)))
        names.add(word)
        if size != "max" and rng.random() < 0.3 and len(word) > 1:
            names.add(word[: rng.randint(1, len(word) - 1)])
    names = sorted(names)[:n] if size == "max" else sorted(names)
    rng.shuffle(names)
    return {"names": names}


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
