"""Random numbers in random bases."""
import random

DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def write(value, base):
    if value == 0:
        return "0"
    out = []
    while value:
        value, d = divmod(value, base)
        out.append(DIGITS[d])
    return "".join(reversed(out))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        value = rng.randint(10**15 - 10**6, 10**15)
        source, target = rng.choice([(2, 36), (36, 2), (3, 35), (10, 2)])
    else:
        value = rng.choice([0, rng.randint(1, 50), rng.randint(1, 10**6), rng.randint(1, 10**15)])
        source, target = rng.randint(2, 36), rng.randint(2, 36)
    return {"number": write(value, source), "source": source, "target": target}


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
