"""Random playlists and press counts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n songs and a k near 10⁹ with k mod n = n / 2, so n / 2
        # presses matter (one by one, that is about n² / 2 steps).
        k = (10**9 // size - 1) * size + size // 2
        return {"playlist": [rng.randint(1, 9999) for _ in range(size)], "k": k}
    if size == "max":
        n = 100_000
        # Small ids keep the data lean (one max test fits the budget); k is near 10⁹ and
        # about half the songs move, so pressing one by one is about n² / 2 steps.
        k = (10**9 // n - 1) * n + rng.randint(n // 3, 2 * n // 3)
        return {"playlist": [rng.randint(1, 99) for _ in range(n)], "k": k}
    n = rng.randint(0, 8)
    return {"playlist": [rng.randint(1, 20) for _ in range(n)], "k": rng.randint(0, 30)}


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
