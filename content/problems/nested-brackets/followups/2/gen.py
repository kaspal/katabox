"""Random bracket texts for nested-brackets--f2."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n = rng.randint(6, 9)
    elif size == "max":
        n = 100_000
    else:
        n = rng.randint(1, 12)
    kinds = rng.choice(["()", "()[]", "()[]{}"])
    out, stack = [], []
    for _ in range(n):
        r = rng.random()
        if stack and r < 0.45:
            c = stack.pop()
            out.append(c if rng.random() < 0.9 else rng.choice(")]}"[: len(kinds) // 2]))
        elif r < 0.9:
            k = rng.randrange(len(kinds) // 2)
            out.append(kinds[2 * k])
            stack.append(kinds[2 * k + 1])
        else:
            out.append(rng.choice(kinds))
    return {"text": "".join(out)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
