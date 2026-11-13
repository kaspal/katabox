"""Random clean calendars and a new slot."""
import random


def calendar(rng, n, gap, width):
    out, t = [], rng.randint(0, gap)
    for _ in range(n):
        s = t
        e = s + rng.randint(0, width)
        out.append([s, e])
        t = e + rng.randint(1, gap)
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        slots = calendar(rng, 10_000, 60, 30)
        top = slots[-1][1]
        a = rng.randint(0, top // 2)
        return {"slots": slots, "fresh": [a, a + rng.randint(0, top // 3)]}
    slots = calendar(rng, rng.randint(0, 5), 3, 3)
    a = rng.randint(0, 25)
    return {"slots": slots, "fresh": [a, a + rng.randint(0, 6)]}


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
