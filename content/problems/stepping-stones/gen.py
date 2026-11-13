"""Random rivers; the big ones have long jumps (a dead zone sometimes)."""
import random


def river(rng, n, top, gap):
    jumps = [rng.randint(0, top) for _ in range(n)]
    if gap and n > 10:
        # A dead zone wider than the jumps around it.
        a = rng.randint(n // 3, 2 * n // 3)
        width = rng.randint(top + 1, top + 50)
        for i in range(max(0, a - top - 60), min(n, a + width)):
            if i < a:
                jumps[i] = min(jumps[i], a - i - 1)
            else:
                jumps[i] = 0
    return jumps


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n stones with jumps up to n / 2 and no dead zone.
        return {"jumps": [rng.randint(size // 4, size // 2) for _ in range(size)]}
    if size == "max":
        n = 100_000
        return {"jumps": [rng.randint(n // 2, n) for _ in range(n)] if seed % 2 else river(rng, n, 3000, True)}
    n = rng.randint(1, 9)
    return {"jumps": river(rng, n, 3, rng.random() < 0.3)}


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
