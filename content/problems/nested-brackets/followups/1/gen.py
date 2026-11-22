"""Random bracket texts for nested-brackets--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n = rng.randint(4, 7)
    elif size == "max":
        n = 100_000
    else:
        n = rng.randint(1, 8)
    alphabet = "()" + ("ab" if rng.random() < 0.5 else "")
    return {"text": "".join(rng.choice(alphabet) for _ in range(n))}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
