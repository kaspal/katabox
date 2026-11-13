"""Random repair plans; nearly full repairs in the big cases."""
import random


def plan(rng, n, k, wide):
    out = []
    for _ in range(k):
        if wide:
            a = rng.randint(0, min(9, n - 1))
            b = rng.randint(max(a, n - 10), n - 1)
            out.append([a, b, rng.randint(1, 3)])
        else:
            a = rng.randrange(n)
            out.append([a, rng.randint(a, n - 1), rng.randint(1, 9)])
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n sections and n nearly full repairs.
        return {"length": size, "repairs": plan(rng, size, size, True)}
    if size == "max":
        # Adding each repair section by section is 10¹⁰ steps.
        return {"length": 100_000, "repairs": plan(rng, 100_000, 100_000, True)}
    n = rng.randint(1, 8)
    return {"length": n, "repairs": plan(rng, n, rng.randint(0, 6), False)}


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
