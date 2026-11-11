"""Random letter runs; max tests are 10^5 letters."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10**5
        if seed % 2 == 0:
            return {"text": rng.choice("abcdefghijklmnopqrstuvwxyz") * n}
        parts, total = [], 0
        while total < n:
            k = min(n - total, rng.choice([1, 1, 2, 3, rng.randint(1, 1000)]))
            parts.append(rng.choice("abcdefghijklmnopqrstuvwxyz") * k)
            total += k
        return {"text": "".join(parts)}
    alphabet = rng.choice(["ab", "abc", "xyz", "a"])
    parts = [rng.choice(alphabet) * rng.choice([1, 1, 2, 3, 9, 10, 12]) for _ in range(rng.randint(1, 6))]
    return {"text": "".join(parts)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
