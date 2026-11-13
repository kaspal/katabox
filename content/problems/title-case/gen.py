"""Random titles with mixed case and repeated spaces."""
import random
import string


def word(rng):
    first = rng.choice(string.ascii_letters + string.digits * 2)
    return first + "".join(rng.choice(string.ascii_letters + string.digits) for _ in range(rng.randint(0, 7)))


def generate(seed, size):
    rng = random.Random(seed)
    limit = 10_000 if size == "max" else 30
    parts, total = [" " * rng.randint(0, 2)], 0
    while True:
        w = word(rng) + " " * rng.randint(1, 3)
        if total + len(w) + len(parts[0]) > limit or (size != "max" and rng.random() < 0.2):
            break
        parts.append(w)
        total += len(w)
    return {"text": "".join(parts)}


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
