"""Random texts for k-letter-stretch."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 9, "max": 10**5}.get(size, rng.randint(1, 12))
    letters = rng.choice(["abc", "abcd", "abcdefghij"])
    text = "".join(rng.choice(letters) for _ in range(n))
    return {"text": text, "k": rng.randint(1, 3)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
