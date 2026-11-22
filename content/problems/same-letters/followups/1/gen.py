"""Random tile bags and changes for same-letters--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, q, letters = 4, 3, "abc"
    elif size == "max":
        n, q, letters = 100_000, 100_000, rng.choice(["ab", "abcdefghijklmnopqrstuvwxyz"])
    else:
        n, q, letters = rng.randint(1, 6), rng.randint(1, 6), rng.choice(["ab", "abc"])
    a = "".join(rng.choice(letters) for _ in range(n))
    if rng.random() < 0.5:
        b = "".join(rng.sample(a, len(a)))
    else:
        b = "".join(rng.choice(letters) for _ in range(rng.randint(max(1, n - 1), n + 1)))
    positions = [rng.randrange(len(b)) for _ in range(q)]
    changes = "".join(rng.choice(letters) for _ in range(q))
    return {"a": a, "b": b, "positions": positions, "letters": changes}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
